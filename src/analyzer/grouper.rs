use crate::types::{ErrorGroup, LogEntry, Severity};
use crate::Result;
use regex::Regex;
use std::collections::HashMap;

pub struct ErrorGrouper {
    // Regex to normalize dynamic values (IDs, numbers, URLs, etc.)
    normalizer: Regex,
}

impl Default for ErrorGrouper {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorGrouper {
    pub fn new() -> Self {
        Self {
            normalizer: Regex::new(
                r"(?x)
                # Timestamps (various formats)
                \d{4}-\d{2}-\d{2}[T\s]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})?  # ISO timestamps
                |\d{4}/\d{2}/\d{2}\s+\d{2}:\d{2}:\d{2}                             # Nginx-style timestamps
                # UUIDs and IDs
                |\b[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\b  # UUIDs
                |\b\d{5,}\b                                                         # Numbers with 5+ digits (IDs, timestamps)
                |\b0x[0-9a-fA-F]+\b                                                # Hex numbers
                # Network
                |https?://[^\s]+                                                   # URLs
                |\b\d+\.\d+\.\d+\.\d+\b                                           # IP addresses
                # Paths and threads
                |/[\w/.-]+:\d+                                                     # File paths with line numbers
                |\[[\w-]+-\d+\]                                                    # Thread names like [nio-8080-exec-1]
                |\bexec-\d+\b                                                      # Thread IDs like exec-1
                "
            ).unwrap(),
        }
    }

    /// Normalize a message by replacing dynamic values with placeholders
    fn normalize_message(&self, message: &str) -> String {
        self.normalizer
            .replace_all(message, "<DYNAMIC>")
            .to_string()
    }

    /// Generate a unique ID for an error pattern
    fn generate_id(pattern: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        pattern.hash(&mut hasher);
        format!("err-{:x}", hasher.finish())
    }

    pub fn group(&self, entries: Vec<LogEntry>) -> Result<Vec<ErrorGroup>> {
        let mut groups: HashMap<String, ErrorGroup> = HashMap::new();

        for entry in entries {
            // Only group errors and warnings
            if !matches!(entry.severity, Severity::Error | Severity::Warning) {
                continue;
            }

            let pattern = self.normalize_message(&entry.message);
            let id = Self::generate_id(&pattern);

            groups
                .entry(id.clone())
                .and_modify(|group| {
                    group.count += 1;
                    if let Some(ts) = entry.timestamp {
                        if ts > group.last_seen {
                            group.last_seen = ts;
                        }
                        if ts < group.first_seen {
                            group.first_seen = ts;
                        }
                    }
                    group.entries.push(entry.clone());
                })
                .or_insert_with(|| {
                    let timestamp = entry.timestamp.unwrap_or_else(chrono::Utc::now);
                    ErrorGroup {
                        id,
                        pattern,
                        count: 1,
                        first_seen: timestamp,
                        last_seen: timestamp,
                        severity: entry.severity,
                        entries: vec![entry],
                        analysis: None,
                    }
                });
        }

        let mut result: Vec<ErrorGroup> = groups.into_values().collect();

        // Sort by severity (Error first) then by count (most frequent first)
        result.sort_by(|a, b| {
            let severity_order = |s: &Severity| match s {
                Severity::Error => 0,
                Severity::Warning => 1,
                _ => 2,
            };

            severity_order(&a.severity)
                .cmp(&severity_order(&b.severity))
                .then_with(|| b.count.cmp(&a.count))
        });

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LogMetadata;
    use std::collections::HashMap;

    #[test]
    fn test_normalize_message() {
        let grouper = ErrorGrouper::new();

        let msg1 = "User 12345 not found";
        let msg2 = "User 67890 not found";

        assert_eq!(
            grouper.normalize_message(msg1),
            grouper.normalize_message(msg2)
        );
    }

    #[test]
    fn test_group_similar_errors() {
        let grouper = ErrorGrouper::new();

        let entries = vec![
            LogEntry {
                timestamp: Some(chrono::Utc::now()),
                severity: Severity::Error,
                message: "Connection failed to 192.168.1.1".to_string(),
                metadata: LogMetadata {
                    file: None,
                    line: None,
                    function: None,
                    thread: None,
                    extra: HashMap::new(),
                },
                raw: "".to_string(),
            },
            LogEntry {
                timestamp: Some(chrono::Utc::now()),
                severity: Severity::Error,
                message: "Connection failed to 192.168.1.2".to_string(),
                metadata: LogMetadata {
                    file: None,
                    line: None,
                    function: None,
                    thread: None,
                    extra: HashMap::new(),
                },
                raw: "".to_string(),
            },
        ];

        let groups = grouper.group(entries).unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].count, 2);
    }
}
