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

    pub fn run(&self) -> Result<(), ()> {
        let ctx = CompilerContext::new();

        ctx.attach_root_file(&self.options.root_file_path).unwrap();

        match ctx.compile() {
            Ok(o) => {
                println!("{:#?}", o);
                Ok(())
            }
            Err(e) => {
                let renderer = DiagnosticRenderer::new(ctx.source_map());

                for diag in e.into_iter() {
                    renderer.emit(diag);
                }

                Err(())
            }
        }
    }
}
