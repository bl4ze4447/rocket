use std::env::home_dir;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(PartialEq)]
pub enum DirectoryActions {
    DisplayDirectory,
    DisplaySearchContent,
}

pub struct PathManager {
    /// The current directory path being viewed in the application.
    pub current_path: PathBuf,

    /// A stack of previously visited paths, used for "Go Back" navigation.
    pub previous_paths: Vec<PathBuf>,

    /// A stack of paths used for "Go Forward" navigation after moving backward in history.
    pub next_paths: Vec<PathBuf>,

    /// A list of files and folders contained within the current directory.
    pub directory_content: Vec<PathBuf>,

    /// The active directory action (displaying directory/search content).
    pub directory_action: DirectoryActions,

    /// This flag indicates if the current_path indicates to a deleted folder.
    ///
    /// This could occur if we "Go Back or Forward" to a deleted folder.
    pub deleted_folder: bool,

    /// This flag signals to the application that directory_content must be updated.
    pub update_folder_content: bool,

    /// A flag used to signal that the mouse cursor icon should be updated (e.g., to a loading or pointer state).
    pub update_cursor_icon: bool,
}

impl PathManager {
    pub fn new() -> Self {
        let home_path = home_dir().unwrap_or_else(|| {
            if cfg!(windows) {
                PathBuf::from("C://")
            } else {
                PathBuf::from("/")
            }
        });

        PathManager {
            current_path: home_path.clone(),
            previous_paths: Vec::new(),
            next_paths: Vec::new(),
            directory_content: Vec::new(),
            directory_action: DirectoryActions::DisplayDirectory,
            deleted_folder: home_path.exists(),
            update_folder_content: true,
            update_cursor_icon: true,
        }
    }

    pub fn update_current_directory(&mut self, path: &Path) {
        self.previous_paths.push(self.current_path.clone());
        self.current_path = path.into();
        self.update_folder_content = true;
        self.directory_action = DirectoryActions::DisplayDirectory;
    }

    pub fn fill_directory_content(&mut self) -> std::io::Result<()> {
        self.directory_content.clear();
        self.update_folder_content = false;

        let entries = match fs::read_dir(&self.current_path) {
            Ok(entries) => {
                self.deleted_folder = false;
                entries
            }
            Err(err) => {
                self.deleted_folder = true;
                return Err(err);
            }
        };

        self.directory_content.extend(
            entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path()),
        );

        self.directory_content.sort();
        self.directory_content.sort_by_key(|key| {
            key.metadata()
                .map(|metadata| metadata.is_file())
                .unwrap_or(true)
        });

        Ok(())
    }
}
