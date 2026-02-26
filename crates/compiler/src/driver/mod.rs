use std::path::PathBuf;

use crate::{context::CompilerContext, error::reporting::DiagnosticRenderer};

pub struct CompilerOptions {
    root_file_path: PathBuf,
}

impl CompilerOptions {
    pub fn new(root_file_path: PathBuf) -> Self {
        Self { root_file_path }
    }
}

pub struct CompilerDriver {
    options: CompilerOptions,
}

impl CompilerDriver {
    pub fn new(options: CompilerOptions) -> Self {
        Self { options }
    }

    pub fn run(&self) -> bool {
        let ctx = CompilerContext::new();

        let root = match ctx.attach_root_file(&self.options.root_file_path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
        };

        let output = match ctx.compile(*root.value) {
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
            Ok(o) => o,
        };

        let diags = ctx.diagnostics.lock().unwrap();
        if !diags.is_empty() {
            let renderer = DiagnosticRenderer::new(ctx.source_map());
            for diag in diags.into_iter() {
                renderer.emit(diag);
            }
            false
        } else {
            println!("{:#?}", output.value);
            true
        }
    }
}
