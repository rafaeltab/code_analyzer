pub struct AnalyzerImplementation {
    pub name: String,
    pub author: String,
}

pub trait Analyzer {
    fn analyze<TFileSystem>(&self, file_system: TFileSystem, file_path: &str) -> Vec<Diagnostic>
    where
        TFileSystem: FileSystem;
}

pub trait FileSystem {
    fn read_file(&self, path: &str) -> impl AsRef<[u8]>;
}

pub struct Diagnostic {
    pub start: u32,
    pub end: u32,
    pub level: u8,
    pub message: String,
}

impl Diagnostic {
    pub fn new(start: u32, end: u32, level: u8, message: String) -> Self {
        Diagnostic {
            start,
            end,
            level,
            message,
        }
    }
}
