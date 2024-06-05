use futures::Stream;

pub struct AnalyzerImplementation {
    pub name: String,
    pub author: String,
}

pub trait Analyzer {
    fn analyze<TFileSystem>(
        &mut self,
        file_system: TFileSystem,
        files: impl Stream<Item = FileEvent>,
    ) -> impl Stream<Item = Diagnostic>
    where
        TFileSystem: FileSystem;
}

pub trait FileSystem {
    fn read_file(&self, path: &str) -> impl AsRef<[u8]>;
}

pub struct Diagnostic {
    pub start: u64,
    pub end: u64,
    pub level: u8,
    pub message: String,
    pub path: String,
}

impl Diagnostic {
    pub fn new(start: u64, end: u64, level: u8, message: String, path: String) -> Self {
        Diagnostic {
            start,
            end,
            level,
            message,
            path,
        }
    }
}

pub enum FileEvent {
    Loaded {
        path: String,
    },
    Changed {
        path: String,
        changes: Vec<FileChange>,
    },
    Created {
        path: String,
    },
    Deleted {
        path: String,
    },
}

pub struct FileChange {
    pub start: u64,
    pub end: u64,
    pub new_value: String,
}
