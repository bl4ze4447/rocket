use std::path::PathBuf;

pub struct CopyAction {
    pub files: Vec<PathBuf>,
    pub destination: Option<PathBuf>,
}

impl CopyAction {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            destination: None,
        }
    }

    pub fn upload_files(&mut self, files: &Vec<PathBuf>) {
        self.files.extend(files.clone());
    }

    pub fn copy_to(&mut self, destination: &PathBuf) {
        self.destination = Some(destination.clone());
    }
}
