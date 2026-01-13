//! Source file representation.

use std::ops::Range;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use crate::source::line_index::LineIndex;

/// Stable identifier for a source file.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SourceFileId(u32);

impl SourceFileId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }

    pub fn invalid() -> Self {
        Self(u32::MAX)
    }

    pub fn is_invalid(&self) -> bool {
        self.0 == u32::MAX
    }
}

/// Immutable source file owned by the compiler.
#[derive(Debug)]
pub struct SourceFile {
    id: SourceFileId,
    path: PathBuf,
    text: Arc<str>,
    hash: u64,
    line_index: OnceLock<LineIndex>,
}

impl SourceFile {
    /// Creates a new source file.
    pub fn new(id: SourceFileId, path: PathBuf, text: impl Into<Arc<str>>) -> Self {
        let text = text.into();
        let hash = calculate_hash(&text);

        Self {
            id,
            path,
            text,
            hash,
            line_index: OnceLock::new(),
        }
    }

    #[inline]
    pub fn id(&self) -> SourceFileId {
        self.id
    }

    #[inline]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn line_range(&self, line: usize) -> Option<Range<usize>> {
        let index = self.line_index.get_or_init(|| LineIndex::new(&self.text));

        index.line_range(line, self.text.len())
    }

    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        let index = self.line_index.get_or_init(|| LineIndex::new(&self.text));

        index.line_col(offset)
    }
}

fn calculate_hash(text: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}
