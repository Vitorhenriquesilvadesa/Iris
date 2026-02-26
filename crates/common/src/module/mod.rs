use crate::source::SourceFileId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleId(u32);

impl ModuleId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }

    pub fn from_file(file: &SourceFileId) -> Self {
        Self(file.as_u32())
    }

    pub fn invalid() -> Self {
        Self(u32::MAX)
    }

    pub fn is_invalid(&self) -> bool {
        self.0 == u32::MAX
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub id: ModuleId,
}
