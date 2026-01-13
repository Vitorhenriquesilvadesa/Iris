use std::ops::Range;

/// Lazily-built line index for a source file.
///
/// Stores the byte offset of the start of each line.
#[derive(Debug)]
pub struct LineIndex {
    line_starts: Vec<usize>,
}

impl LineIndex {
    pub fn new(text: &str) -> Self {
        let mut line_starts = vec![0];

        for (i, byte) in text.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push(i + 1);
            }
        }

        Self { line_starts }
    }

    pub fn line_range(&self, line_idx: usize, source_len: usize) -> Option<Range<usize>> {
        let start = *self.line_starts.get(line_idx)? as usize;

        let end = self
            .line_starts
            .get(line_idx + 1)
            .map(|&pos| pos as usize)
            .unwrap_or(source_len);

        Some(start..end)
    }

    /// Returns the (line, column) for the given byte offset.
    ///
    /// Lines and columns are zero-based.
    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        let line = match self.line_starts.binary_search(&offset) {
            Ok(line) => line,
            Err(next_line) => next_line - 1,
        };

        let col = offset - self.line_starts[line];
        (line, col)
    }
}
