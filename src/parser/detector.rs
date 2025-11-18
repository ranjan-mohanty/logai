use super::{
    formats::{ApacheParser, JsonParser, NginxParser, PlainTextParser, SyslogParser},
    LogParser, StackTraceParser,
};
use std::sync::Arc;

pub struct FormatDetector;

impl FormatDetector {
    /// Detect log format from a sample line and return appropriate parser
    /// Parsers are tried in order of specificity (most specific first)
    pub fn detect(sample: &str) -> Arc<dyn LogParser> {
        // Try JSON first (most specific)
        let json_parser = JsonParser::new();
        if json_parser.can_parse(sample) {
            return Arc::new(StackTraceParser::new(Arc::new(json_parser)));
        }

        // Try Apache format
        let apache_parser = ApacheParser::new();
        if apache_parser.can_parse(sample) {
            return Arc::new(apache_parser);
        }

        // Try Nginx format
        let nginx_parser = NginxParser::new();
        if nginx_parser.can_parse(sample) {
            return Arc::new(nginx_parser);
        }

        // Try Syslog format
        let syslog_parser = SyslogParser::new();
        if syslog_parser.can_parse(sample) {
            return Arc::new(syslog_parser);
        }

        // Fallback to plain text with stack trace support
        Arc::new(StackTraceParser::new(Arc::new(PlainTextParser::new())))
    }

    /// Detect format with confidence scoring
    pub fn detect_with_confidence(sample: &str) -> (Arc<dyn LogParser>, f32) {
        let json_parser = JsonParser::new();
        if json_parser.can_parse(sample) {
            return (Arc::new(StackTraceParser::new(Arc::new(json_parser))), 0.95);
        }

        let apache_parser = ApacheParser::new();
        if apache_parser.can_parse(sample) {
            return (Arc::new(apache_parser), 0.90);
        }

        let nginx_parser = NginxParser::new();
        if nginx_parser.can_parse(sample) {
            return (Arc::new(nginx_parser), 0.90);
        }

        let syslog_parser = SyslogParser::new();
        if syslog_parser.can_parse(sample) {
            return (Arc::new(syslog_parser), 0.85);
        }

        // Plain text fallback has low confidence
        (
            Arc::new(StackTraceParser::new(Arc::new(PlainTextParser::new()))),
            0.50,
        )
    }

    /// Detect format from multiple sample lines for better accuracy
    pub fn detect_from_samples(samples: &[&str]) -> Arc<dyn LogParser> {
        if samples.is_empty() {
            return Arc::new(StackTraceParser::new(Arc::new(PlainTextParser::new())));
        }

        // Try each parser and count matches
        let mut json_matches = 0;
        let mut apache_matches = 0;
        let mut nginx_matches = 0;
        let mut syslog_matches = 0;

        let json_parser = JsonParser::new();
        let apache_parser = ApacheParser::new();
        let nginx_parser = NginxParser::new();
        let syslog_parser = SyslogParser::new();

        for sample in samples {
            if json_parser.can_parse(sample) {
                json_matches += 1;
            }
            if apache_parser.can_parse(sample) {
                apache_matches += 1;
            }
            if nginx_parser.can_parse(sample) {
                nginx_matches += 1;
            }
            if syslog_parser.can_parse(sample) {
                syslog_matches += 1;
            }
        }

        // Return parser with most matches
        let max_matches = json_matches
            .max(apache_matches)
            .max(nginx_matches)
            .max(syslog_matches);

        if max_matches > 0 {
            if json_matches == max_matches {
                return Arc::new(StackTraceParser::new(Arc::new(json_parser)));
            }
            if apache_matches == max_matches {
                return Arc::new(apache_parser);
            }
            if nginx_matches == max_matches {
                return Arc::new(nginx_parser);
            }
            if syslog_matches == max_matches {
                return Arc::new(syslog_parser);
            }
        }

        // Fallback to plain text
        Arc::new(StackTraceParser::new(Arc::new(PlainTextParser::new())))
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
    fn test_detect_apache() {
        let sample =
            r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /test HTTP/1.0" 200 123"#;
        let parser = FormatDetector::detect(sample);
        assert!(parser.can_parse(sample));
    }

    #[test]
    fn test_detect_nginx() {
        let sample =
            r#"192.168.1.1 - - [17/Nov/2025:10:30:00 +0000] "GET /api HTTP/1.1" 200 123 0.001"#;
        let parser = FormatDetector::detect(sample);
        assert!(parser.can_parse(sample));
    }

    #[test]
    fn test_detect_syslog() {
        let sample = r#"<34>Oct 11 22:14:15 mymachine su: test message"#;
        let parser = FormatDetector::detect(sample);
        assert!(parser.can_parse(sample));
    }

    #[test]
    fn test_detect_plain() {
        let sample = "2025-11-17 ERROR Something went wrong";
        let parser = FormatDetector::detect(sample);
        assert!(parser.can_parse(sample));
    }

    #[test]
    fn test_detect_with_confidence() {
        let json_sample = r#"{"level":"error","message":"test"}"#;
        let (_, confidence) = FormatDetector::detect_with_confidence(json_sample);
        assert!(confidence > 0.9);

        let plain_sample = "Random log message";
        let (_, confidence) = FormatDetector::detect_with_confidence(plain_sample);
        assert!(confidence < 0.6);
    }

    #[test]
    fn test_detect_from_samples() {
        let samples = vec![
            r#"{"level":"error","message":"test1"}"#,
            r#"{"level":"info","message":"test2"}"#,
            r#"{"level":"warn","message":"test3"}"#,
        ];

        let parser = FormatDetector::detect_from_samples(&samples);
        assert!(parser.can_parse(samples[0]));
    }

    #[test]
    fn test_detect_from_mixed_samples() {
        let samples = vec![
            r#"127.0.0.1 - - [10/Oct/2000:13:55:36 -0700] "GET /1 HTTP/1.0" 200 123"#,
            r#"127.0.0.1 - - [10/Oct/2000:13:55:37 -0700] "GET /2 HTTP/1.0" 200 456"#,
            "Random line",
        ];

        let parser = FormatDetector::detect_from_samples(&samples);
        // Should detect Apache since it has 2 matches vs 0 for others
        assert!(parser.can_parse(samples[0]));
    }

    #[test]
    fn test_detect_from_empty_samples() {
        let samples: Vec<&str> = vec![];
        let parser = FormatDetector::detect_from_samples(&samples);
        // Should return plain text parser
        assert!(parser.can_parse("any text"));
    }
}
