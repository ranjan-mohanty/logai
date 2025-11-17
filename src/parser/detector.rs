use super::{json::JsonParser, plain::PlainTextParser, LogParser};
use std::sync::Arc;

pub struct FormatDetector;

impl FormatDetector {
    pub fn detect(sample: &str) -> Arc<dyn LogParser> {
        let json_parser = JsonParser::new();
        if json_parser.can_parse(sample) {
            return Arc::new(json_parser);
        }

        // Fallback to plain text
        Arc::new(PlainTextParser::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_json() {
        let sample = r#"{"level":"error","message":"test"}"#;
        let parser = FormatDetector::detect(sample);
        assert!(parser.can_parse(sample));
    }

    #[test]
    fn test_detect_plain() {
        let sample = "2025-11-17 ERROR Something went wrong";
        let parser = FormatDetector::detect(sample);
        assert!(parser.can_parse(sample));
    }
}
