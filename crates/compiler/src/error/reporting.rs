use colored::{Color, Colorize};
use common::{
    diagnostic::{Diagnostic, DiagnosticLabel, DiagnosticSeverity},
    source::SourceFile,
};

use crate::map::source_map::SourceMap;

pub struct DiagnosticRenderer<'a> {
    map: &'a SourceMap,
}

impl<'a> DiagnosticRenderer<'a> {
    pub fn new(map: &'a SourceMap) -> Self {
        Self { map }
    }

    pub fn emit(&self, diag: &Diagnostic) {
        let (color, prefix) = match diag.severity {
            DiagnosticSeverity::Error => (Color::Red, "error"),
            DiagnosticSeverity::Warning => (Color::Yellow, "warning"),
            DiagnosticSeverity::Note => (Color::Cyan, "note"),
        };

        if let Some(code) = diag.code {
            eprint!("{}", format!("{}[{}]", prefix, code.0).color(color).bold());
        } else {
            eprint!("{}", prefix.color(color).bold());
        }
        eprintln!(": {}", diag.message.bold());

        if let (Some(file_id), Some(span)) = (diag.file, diag.primary_span)
            && let Ok(file) = self.map.load_by_id(file_id)
        {
            self.render_snippet(&file.value, span.start, span.end(), color, &diag.labels);
        }

        for note in &diag.notes {
            eprintln!("  {}: {}", "note".cyan().bold(), note);
        }

        eprintln!();
    }

    fn render_snippet(
        &self,
        file: &SourceFile,
        start: usize,
        end: usize,
        color: Color,
        labels: &[DiagnosticLabel],
    ) {
        let (line_idx, col_idx) = file.line_col(start);

        let line_num = line_idx + 1;

        let line_range = file.line_range(line_idx);
        let line_text = &file.text()[line_range.clone().unwrap_or(0..0)];

        let line_text_clean = line_text.trim_end();

        let path_str = file.path().to_string_lossy();
        eprintln!(
            "{} {}:{}:{}",
            "-->".blue().bold(),
            path_str,
            line_num,
            col_idx + 1
        );

        let gutter = format!(" {} |", line_num).blue().bold();
        let empty_gutter = format!("{}  |", " ".repeat(line_num.to_string().len()))
            .blue()
            .bold();

        eprintln!(" {}", empty_gutter);
        eprintln!(" {} {}", gutter, line_text_clean);

        let len = (end - start).max(1);

        let padding = " ".repeat(col_idx);
        let markers = "^".repeat(len).color(color).bold();

        let decoration_start = format!(" {} {}{}", empty_gutter, padding, markers);

        if labels.is_empty() {
            eprintln!("{}", decoration_start);
        } else {
            eprint!("{}", decoration_start);

            for label in labels {
                if let Some(msg) = &label.message {
                    eprintln!(" {}", msg.color(Color::Red));
                }
            }
        }
    }
}
