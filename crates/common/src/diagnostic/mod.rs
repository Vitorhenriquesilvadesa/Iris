pub mod codes;

use crate::{source::SourceFileId, span::Span};

/// Severity level of a diagnostic.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Note,
}

/// Stable diagnostic code (used for tooling and documentation).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DiagnosticCode(pub &'static str);

/// Additional span information attached to a diagnostic.
#[derive(Debug, Clone)]
pub struct DiagnosticLabel {
    pub span: Span,
    pub message: Option<String>,
    pub is_primary: bool,
}

/// A compiler diagnostic describing an error, warning, or note.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub file: Option<SourceFileId>,
    pub severity: DiagnosticSeverity,
    pub code: Option<DiagnosticCode>,
    pub message: String,
    pub primary_span: Option<Span>,
    pub labels: Vec<DiagnosticLabel>,
    pub notes: Vec<String>,
}

pub type DiagnosticResult<T> = Result<T, Box<Diagnostic>>;

impl Diagnostic {
    /// Creates a new error diagnostic.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            file: None,
            severity: DiagnosticSeverity::Error,
            code: None,
            message: message.into(),
            primary_span: None,
            labels: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Creates a new warning diagnostic.
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            file: None,
            severity: DiagnosticSeverity::Warning,
            code: None,
            message: message.into(),
            primary_span: None,
            labels: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Creates a new note diagnostic.
    pub fn note(message: impl Into<String>) -> Self {
        Self {
            file: None,
            severity: DiagnosticSeverity::Note,
            code: None,
            message: message.into(),
            primary_span: None,
            labels: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Associates this diagnostic with a source file.
    pub fn with_file(mut self, file: SourceFileId) -> Self {
        self.file = Some(file);
        self
    }

    /// Attaches a diagnostic code.
    pub fn with_code(mut self, code: DiagnosticCode) -> Self {
        self.code = Some(code);
        self
    }

    /// Sets the primary span of the diagnostic.
    ///
    /// The primary span is also registered as a primary label.
    pub fn with_primary_span(mut self, span: Span) -> Self {
        self.primary_span = Some(span);
        self.labels.push(DiagnosticLabel {
            span,
            message: None,
            is_primary: true,
        });
        self
    }

    /// Adds a secondary label with a message.
    pub fn with_label(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push(DiagnosticLabel {
            span,
            message: Some(message.into()),
            is_primary: false,
        });
        self
    }

    /// Adds an additional note to the diagnostic.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
}

/// A collection of diagnostics.
#[derive(Debug, Clone)]
pub struct Diagnostics {
    items: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new(items: Vec<Diagnostic>) -> Self {
        Self { items }
    }

    /// Creates a diagnostics collection with a single diagnostic.
    pub fn single(diagnostic: Diagnostic) -> Self {
        Self {
            items: vec![diagnostic],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.items.push(diagnostic);
    }

    pub fn extend(&mut self, other: &Diagnostics) {
        for item in &other.items {
            self.items.push(item.clone());
        }
    }
}

impl<'a> IntoIterator for &'a Diagnostics {
    type Item = &'a Diagnostic;
    type IntoIter = std::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a> IntoIterator for &'a mut Diagnostics {
    type Item = &'a mut Diagnostic;
    type IntoIter = std::slice::IterMut<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}
