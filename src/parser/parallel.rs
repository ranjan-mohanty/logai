use crate::parser::{LogParser, ParsingStatistics};
use crate::types::LogEntry;
use crate::Result;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Parser that processes log files in parallel using multiple threads
pub struct ParallelParser {
    parser: Arc<dyn LogParser>,
    chunk_size: usize,
    num_threads: Option<usize>,
    streaming_threshold_bytes: u64,
}

impl ParallelParser {
    /// Create a new parallel parser with the given parser and configuration
    pub fn new(parser: Arc<dyn LogParser>, chunk_size: usize, num_threads: Option<usize>) -> Self {
        Self::with_threshold(parser, chunk_size, num_threads, 100 * 1024 * 1024)
        // 100MB default
    }

    /// Create a new parallel parser with custom streaming threshold
    pub fn with_threshold(
        parser: Arc<dyn LogParser>,
        chunk_size: usize,
        num_threads: Option<usize>,
        streaming_threshold_bytes: u64,
    ) -> Self {
        // Configure rayon thread pool if specified
        if let Some(threads) = num_threads {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build_global()
                .ok(); // Ignore error if already initialized
        }

        Self {
            parser,
            chunk_size,
            num_threads,
            streaming_threshold_bytes,
        }
    }

    /// Parse a file in parallel, returning all log entries
    pub fn parse_file(&self, path: &Path) -> Result<Vec<LogEntry>> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let file_size = metadata.len();

        // For large files (above threshold), use streaming approach
        if file_size > self.streaming_threshold_bytes {
            self.parse_streaming(BufReader::new(file))
        } else {
            // For smaller files, load all lines and parse in parallel
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
            self.parse_parallel(&lines)
        }
    }

    /// Parse a file in parallel and return statistics
    pub fn parse_file_with_stats(&self, path: &Path) -> Result<(Vec<LogEntry>, ParsingStatistics)> {
        let start_time = Instant::now();
        let total_lines = AtomicUsize::new(0);
        let parse_errors = AtomicUsize::new(0);

        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let file_size = metadata.len();

        let entries = if file_size > self.streaming_threshold_bytes {
            self.parse_streaming_with_tracking(BufReader::new(file), &total_lines, &parse_errors)?
        } else {
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
            total_lines.store(lines.len(), Ordering::Relaxed);
            self.parse_parallel_with_tracking(&lines, &parse_errors)?
        };

        let duration_ms = start_time.elapsed().as_millis() as u64;
        let parsed_entries = entries.len();
        let multiline_entries = entries.iter().filter(|e| e.message.contains('\n')).count();

        let stats = ParsingStatistics {
            total_lines: total_lines.load(Ordering::Relaxed),
            parsed_entries,
            parse_errors: parse_errors.load(Ordering::Relaxed),
            multiline_entries,
            duration_ms,
        };

        Ok((entries, stats))
    }

    /// Parse lines in parallel using rayon
    fn parse_parallel(&self, lines: &[String]) -> Result<Vec<LogEntry>> {
        let entries: Vec<LogEntry> = lines
            .par_chunks(self.chunk_size)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .filter_map(|line| self.parser.parse_line(line).ok().flatten())
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(entries)
    }

    /// Parse lines in parallel with error tracking
    fn parse_parallel_with_tracking(
        &self,
        lines: &[String],
        parse_errors: &AtomicUsize,
    ) -> Result<Vec<LogEntry>> {
        let entries: Vec<LogEntry> = lines
            .par_chunks(self.chunk_size)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .filter_map(|line| match self.parser.parse_line(line) {
                        Ok(Some(entry)) => Some(entry),
                        Ok(None) => None,
                        Err(_) => {
                            parse_errors.fetch_add(1, Ordering::Relaxed);
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(entries)
    }

    /// Parse a large file using streaming to limit memory usage
    fn parse_streaming(&self, reader: BufReader<File>) -> Result<Vec<LogEntry>> {
        let mut entries = Vec::new();
        let mut buffer = Vec::with_capacity(self.chunk_size);

        for line in reader.lines() {
            let line = line?;
            buffer.push(line);

            if buffer.len() >= self.chunk_size {
                // Process chunk in parallel
                let chunk_entries = self.parse_parallel(&buffer)?;
                entries.extend(chunk_entries);
                buffer.clear();
            }
        }

        // Process remaining lines
        if !buffer.is_empty() {
            let chunk_entries = self.parse_parallel(&buffer)?;
            entries.extend(chunk_entries);
        }

        Ok(entries)
    }

    /// Parse a large file using streaming with statistics tracking
    fn parse_streaming_with_tracking(
        &self,
        reader: BufReader<File>,
        total_lines: &AtomicUsize,
        parse_errors: &AtomicUsize,
    ) -> Result<Vec<LogEntry>> {
        let mut entries = Vec::new();
        let mut buffer = Vec::with_capacity(self.chunk_size);

        for line in reader.lines() {
            let line = line?;
            buffer.push(line);
            total_lines.fetch_add(1, Ordering::Relaxed);

            if buffer.len() >= self.chunk_size {
                // Process chunk in parallel
                let chunk_entries = self.parse_parallel_with_tracking(&buffer, parse_errors)?;
                entries.extend(chunk_entries);
                buffer.clear();
            }
        }

        // Process remaining lines
        if !buffer.is_empty() {
            let chunk_entries = self.parse_parallel_with_tracking(&buffer, parse_errors)?;
            entries.extend(chunk_entries);
        }

        Ok(entries)
    }

    /// Get the configured chunk size
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Get the configured number of threads
    pub fn num_threads(&self) -> Option<usize> {
        self.num_threads
    }

    /// Get the configured streaming threshold in bytes
    pub fn streaming_threshold_bytes(&self) -> u64 {
        self.streaming_threshold_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::formats::PlainTextParser;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parallel_parser_small_file() {
        let parser = Arc::new(PlainTextParser::new());
        let parallel_parser = ParallelParser::new(parser, 100, None);

        // Create a temporary file with test data
        let mut temp_file = NamedTempFile::new().unwrap();
        for i in 0..500 {
            writeln!(temp_file, "Log line {}", i).unwrap();
        }

        let entries = parallel_parser.parse_file(temp_file.path()).unwrap();
        assert_eq!(entries.len(), 500);
    }

    #[test]
    fn test_parallel_parser_with_stats() {
        let parser = Arc::new(PlainTextParser::new());
        let parallel_parser = ParallelParser::new(parser, 100, None);

        // Create a temporary file with test data
        let mut temp_file = NamedTempFile::new().unwrap();
        for i in 0..1000 {
            writeln!(temp_file, "Log line {}", i).unwrap();
        }

        let (entries, stats) = parallel_parser
            .parse_file_with_stats(temp_file.path())
            .unwrap();

        assert_eq!(entries.len(), 1000);
        assert_eq!(stats.total_lines, 1000);
        assert_eq!(stats.parsed_entries, 1000);
        assert_eq!(stats.parse_errors, 0);
    }

    #[test]
    fn test_parallel_parser_chunk_processing() {
        let parser = Arc::new(PlainTextParser::new());
        let parallel_parser = ParallelParser::new(parser, 50, Some(2));

        // Create lines that will be split into multiple chunks
        let lines: Vec<String> = (0..250).map(|i| format!("Log line {}", i)).collect();

        let entries = parallel_parser.parse_parallel(&lines).unwrap();
        assert_eq!(entries.len(), 250);
    }

    #[test]
    fn test_parallel_parser_custom_threshold() {
        let parser = Arc::new(PlainTextParser::new());
        // Set a very low threshold (1KB) to force streaming mode
        let parallel_parser = ParallelParser::with_threshold(parser, 100, None, 1024);

        // Create a temporary file larger than 1KB
        let mut temp_file = NamedTempFile::new().unwrap();
        for i in 0..100 {
            writeln!(temp_file, "Log line with some content {}", i).unwrap();
        }

        let entries = parallel_parser.parse_file(temp_file.path()).unwrap();
        assert_eq!(entries.len(), 100);
        assert_eq!(parallel_parser.streaming_threshold_bytes(), 1024);
    }

    #[test]
    fn test_streaming_large_file() {
        let parser = Arc::new(PlainTextParser::new());
        let parallel_parser = ParallelParser::new(parser, 100, None);

        // Create a temporary file with many lines
        let mut temp_file = NamedTempFile::new().unwrap();
        for i in 0..10000 {
            writeln!(temp_file, "Log line {}", i).unwrap();
        }

        let (entries, stats) = parallel_parser
            .parse_file_with_stats(temp_file.path())
            .unwrap();

        assert_eq!(entries.len(), 10000);
        assert_eq!(stats.total_lines, 10000);
        assert_eq!(stats.parsed_entries, 10000);
    }
}
