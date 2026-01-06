use std::collections::HashSet;
use crate::lang_string::{LangKeys, LangString};
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SelectionMode {
    Single,
    Multiple,
    Ranged,
}

pub enum SelectionResult {
    Single(PathBuf),
    Multiple(HashSet<PathBuf>),
    Err(String),
}

impl SelectionResult {
    pub fn clone(&self) -> SelectionResult {
        match self {
            SelectionResult::Single(path) => SelectionResult::Single(path.clone()),
            SelectionResult::Multiple(paths) => SelectionResult::Multiple(paths.clone()),
            SelectionResult::Err(err) => SelectionResult::Err(err.clone()),
        }
    }
}

pub struct SelectAction {
    /// Selected files
    pub files: HashSet<PathBuf>,

    /// Current selection mode
    pub mode: SelectionMode,
}

impl SelectAction {
    pub fn new() -> Self {
        Self {
            files: HashSet::new(),
            mode: SelectionMode::Single,
        }
    }

    pub fn is_file_selected(&self, file: &PathBuf) -> bool {
        self.files.contains(file)
    }

    pub fn select_file(&mut self, file: &PathBuf, directory_content: Option<&[PathBuf]>) {
        match self.mode {
            // Only one file can be selected at a time
            // If the file is already selected, deselect it
            SelectionMode::Single => {
                if !self.files.is_empty() {
                    self.files.clear();
                    self.files.insert(file.clone());
                    return;
                }

                if self.is_file_selected(file) {
                    self.files.clear();
                    return;
                }

                self.files.insert(file.clone());
            }

            // Multiple files can be selected
            // If the file is already selected, deselect it
            SelectionMode::Multiple => {
                if self.is_file_selected(file) {
                    self.files.retain(|f| *f != *file);
                    return;
                }

                self.files.insert(file.clone());
            }

            SelectionMode::Ranged => {
                let Some(content) = directory_content else { return; };

                let Some(target_idx) = content.iter().position(|f| f == file) else { return; };

                let start_idx = content.iter()
                    .enumerate()
                    .rev()
                    .find(|(_, f)| self.files.contains(*f))
                    .map(|(idx, _)| idx)
                    .unwrap_or(0);

                let min = start_idx.min(target_idx);
                let max = start_idx.max(target_idx);

                for i in min..=max {
                    self.files.insert(content[i].clone());
                }
            }
        }
    }

    pub fn deselect_file(&mut self, file: &PathBuf) {
        self.files.retain(|f| *f != *file);
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
        // Attempt to get the first file immediately
        let Some(first_file) = self.files.iter().next() else {
            return SelectionResult::Err(lang_string.get(LangKeys::NothingSelected));
        };

        match self.mode {
            SelectionMode::Single => SelectionResult::Single(first_file.clone()),
            SelectionMode::Multiple | SelectionMode::Ranged => {
                SelectionResult::Multiple(self.files.clone())
            }
        }
    }

    pub fn remove_deleted_files(&mut self) {
        self.files.retain(|f| f.exists());
    }
}
