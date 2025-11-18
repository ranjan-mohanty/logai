use std::io::{BufRead, BufReader, Read};

/// Read lines from a reader with UTF-8 encoding recovery
pub struct LossyLineReader<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> LossyLineReader<R> {
    /// Create a new lossy line reader
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }

    /// Read the next line with lossy UTF-8 conversion
    /// Returns None when EOF is reached
    pub fn read_line(&mut self) -> std::io::Result<Option<String>> {
        let mut buffer = Vec::new();
        let bytes_read = self.reader.read_until(b'\n', &mut buffer)?;

        if bytes_read == 0 {
            return Ok(None);
        }

        // Try UTF-8 first
        match String::from_utf8(buffer.clone()) {
            Ok(s) => Ok(Some(s.trim_end_matches('\n').to_string())),
            Err(_) => {
                // Use lossy conversion (replaces invalid sequences with �)
                log::warn!("Invalid UTF-8 detected, using lossy conversion");
                Ok(Some(
                    String::from_utf8_lossy(&buffer)
                        .trim_end_matches('\n')
                        .to_string(),
                ))
            }
        }
    }
}

impl<R: Read> Iterator for LossyLineReader<R> {
    type Item = std::io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_line() {
            Ok(Some(line)) => Some(Ok(line)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_valid_utf8() {
        let data = b"Hello\nWorld\n";
        let mut reader = LossyLineReader::new(Cursor::new(data));

        assert_eq!(reader.read_line().unwrap(), Some("Hello".to_string()));
        assert_eq!(reader.read_line().unwrap(), Some("World".to_string()));
        assert_eq!(reader.read_line().unwrap(), None);
    }

    #[test]
    fn test_invalid_utf8() {
        // Create data with invalid UTF-8 sequence
        let mut data = Vec::new();
        data.extend_from_slice(b"Valid line\n");
        data.extend_from_slice(&[0xFF, 0xFE, 0xFD]); // Invalid UTF-8
        data.extend_from_slice(b" with invalid bytes\n");
        data.extend_from_slice(b"Another valid line\n");

        let mut reader = LossyLineReader::new(Cursor::new(data));

        assert_eq!(reader.read_line().unwrap(), Some("Valid line".to_string()));
        // Invalid UTF-8 should be replaced with replacement character
        let line2 = reader.read_line().unwrap().unwrap();
        assert!(line2.contains('\u{FFFD}')); // Replacement character
        assert_eq!(
            reader.read_line().unwrap(),
            Some("Another valid line".to_string())
        );
        assert_eq!(reader.read_line().unwrap(), None);
    }

    #[test]
    fn test_iterator() {
        let data = b"Line 1\nLine 2\nLine 3\n";
        let reader = LossyLineReader::new(Cursor::new(data));

        let lines: Vec<String> = reader.map(|r| r.unwrap()).collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "Line 1");
        assert_eq!(lines[1], "Line 2");
        assert_eq!(lines[2], "Line 3");
    }

    #[test]
    fn test_empty_input() {
        let data = b"";
        let mut reader = LossyLineReader::new(Cursor::new(data));

        assert_eq!(reader.read_line().unwrap(), None);
    }

    #[test]
    fn test_no_trailing_newline() {
        let data = b"Line without newline";
        let mut reader = LossyLineReader::new(Cursor::new(data));

        assert_eq!(
            reader.read_line().unwrap(),
            Some("Line without newline".to_string())
        );
        assert_eq!(reader.read_line().unwrap(), None);
    }
}
