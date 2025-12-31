use std::path::PathBuf;
use crate::lang_string::{LangKeys, LangString};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SelectionMode {
    Single,
    Multiple,
    Ranged,
}

pub enum SelectionResult {
    Single(PathBuf),
    Multiple(Vec<PathBuf>),
    Err(String)
}

impl SelectionResult {
    pub fn clone(&self) -> SelectionResult {
        match self {
            SelectionResult::Single(path) => SelectionResult::Single(path.clone()),
            SelectionResult::Multiple(paths) => SelectionResult::Multiple(paths.clone()),
            SelectionResult::Err(err) => SelectionResult::Err(err.clone())
        }
    }
}

/// This should answer these questions:
/// * what files/folders are selected? we go the unix way for calling, everything is a 'file'
/// * how are we selecting files? one by one, multiple, etc.
pub struct SelectAction {
    pub files: Vec<PathBuf>,
    pub mode: SelectionMode,
}

impl SelectAction {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            mode: SelectionMode::Single
        }
    }

    pub fn is_file_selected(&self, file: &PathBuf) -> bool {
        self.files.contains(file)
    }

    pub fn select_file(&mut self, file: &PathBuf, ) {
        match self.mode {
            // Only one file can be selected at a time
            // If the file is already selected, deselect it
            SelectionMode::Single => {
                if !self.files.is_empty() {
                    self.files.clear();
                    self.files.push(file.clone());
                    return;
                }

                if self.is_file_selected(file) {
                    self.files.clear();
                    return;
                }

                self.files.push(file.clone());
            }

            // Multiple files can be selected
            // If the file is already selected, deselect it
            SelectionMode::Multiple => {
                if self.is_file_selected(file) {
                    self.files.retain(|f| *f != *file);
                    return;
                }

                self.files.push(file.clone());
            }

            SelectionMode::Ranged => {
                // todo(bl4ze4447):
            }
        }
    }

    pub fn toggle_selection_mode(&mut self) {
        self.mode = match self.mode {
            SelectionMode::Single => SelectionMode::Multiple,
            SelectionMode::Multiple => SelectionMode::Single,
            SelectionMode::Ranged => SelectionMode::Ranged,
        }
    }

    pub fn clear_selection(&mut self) {
        self.files.clear();
        self.mode = SelectionMode::Single;
    }

    pub fn get_selected_files(&self, lang_string: &LangString) -> SelectionResult {
        if self.files.is_empty() {
            return SelectionResult::Err(lang_string.get(LangKeys::NothingSelected));
        }

        match self.mode {
            SelectionMode::Single => SelectionResult::Single(self.files[0].clone()),
            SelectionMode::Multiple => SelectionResult::Multiple(self.files.clone()),
            SelectionMode::Ranged => SelectionResult::Multiple(self.files.clone()),
        }
    }

    pub fn remove_deleted_files(&mut self) {
        self.files.retain(|f| f.exists());
    }
}