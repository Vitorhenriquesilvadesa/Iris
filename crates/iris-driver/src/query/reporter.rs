use iris_diagnostic::Diagnostics;

pub(crate) trait ErrorReporter {
    fn emit_diagnostics(&self, diags: &Diagnostics);
}
