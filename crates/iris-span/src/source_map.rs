use std::{
    fs, io,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};

use crate::source_file::{SourceFile, SourceFileId};
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

    pub fn load_by_id(&self, id: SourceFileId) -> Option<Arc<SourceFile>> {
        self.files.get(&id).map(|f| f.clone())
    }

    pub fn load_file<P: Into<PathBuf>>(&self, path: P) -> io::Result<Arc<SourceFile>> {
        let path = path.into();

        if let Some(id) = self.paths.get(&path)
            && let Some(file) = self.files.get(&*id)
        {
            return Ok(file.clone());
        }

        let text = fs::read_to_string(&path)?;

        let id_val = self.next_id.fetch_add(1, Ordering::Relaxed);
        let id = SourceFileId::new(id_val);

        let source_file = Arc::new(SourceFile::new(id, path.clone(), text));

        self.files.insert(id, source_file.clone());
        self.paths.insert(path, id);

        Ok(source_file)
    }
}
