use std::{
    fs,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};

use common::{
    diagnostic::Diagnostics,
    source::{SourceFile, SourceFileId},
};
use compiler_api::queries::{AnalysisResult, QueryResult};
use dashmap::DashMap;

#[derive(Debug, Default)]
pub struct SourceMap {
    files: DashMap<SourceFileId, Arc<SourceFile>>,
    paths: DashMap<PathBuf, SourceFileId>,
    next_id: AtomicU32,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            files: DashMap::new(),
            paths: DashMap::new(),
            next_id: 1.into(),
        }
    }

    pub fn load_by_id(&self, id: SourceFileId) -> QueryResult<SourceFile> {
        if let Some(file) = self.files.get(&id) {
            return Ok(AnalysisResult {
                value: file.clone(),
                diagnostics: Arc::new(Diagnostics::new(vec![])),
            });
        }

        let msg = format!("File with id '{}' not found in registry.", id.as_u32());
        // let diag = Diagnostic::error(msg);
        Err(msg)
    }

    pub fn load_file<P: Into<PathBuf>>(&self, path: P) -> QueryResult<SourceFile> {
        let path = path.into();

        if let Some(id) = self.paths.get(&path)
            && let Some(file) = self.files.get(&*id)
        {
            return Ok(AnalysisResult {
                value: file.clone(),
                diagnostics: Arc::new(Diagnostics::new(vec![])),
            });
        }

        let text = match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                let msg = format!("Unable to read file '{}': {}", path.display(), e);
                return Err(msg);
            }
        };

        let id_val = self.next_id.fetch_add(1, Ordering::Relaxed);
        let id = SourceFileId::new(id_val);

        let source_file = Arc::new(SourceFile::new(id, path.clone(), text));

        self.files.insert(id, source_file.clone());
        self.paths.insert(path, id);

        Ok(AnalysisResult {
            value: source_file,
            diagnostics: Arc::new(Diagnostics::new(vec![])),
        })
    }
}
