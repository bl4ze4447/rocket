use std::{env, fs};
use std::cmp::Ordering;
use std::env::home_dir;
use std::path::{Path, PathBuf};

/// PathManager should answer these questions:
/// * what is the default folder where we start? (maybe store this in a file and load it from file if exists)
/// * where are we?
/// * what files are inside?
/// * history of previous paths?
/// * history of paths where we went back to (Vec<PathBuf> which holds every path when we pressed Go Back)
/// * are we showing the directory contents or are we searching for a file? (enum)
/// * did we go back to a deleted folder?
/// * did we change folders?

pub enum DirectoryActions {
    DisplayContents,
    Searching
}

pub struct PathManager {
    default_path:               PathBuf,
    pub current_path:           PathBuf,
    pub previous_paths:         Vec<PathBuf>,
    pub next_paths:             Vec<PathBuf>,
    pub directory_content:      Vec<PathBuf>,
    pub directory_action:       DirectoryActions,
    pub deleted_folder:         bool,
    pub update_folder_content:  bool,
}

impl PathManager {
    pub fn new() -> Self {
        let home_path = home_dir()
            .unwrap_or_else(|| {
                if cfg!(windows) {
                    PathBuf::from("C://")
                } else {
                    PathBuf::from("/")
                }
        });

        PathManager {
            default_path: home_path.clone(),
            current_path: home_path.clone(),
            previous_paths: Vec::new(),
            next_paths: Vec::new(),
            directory_content: Vec::new(),
            directory_action: DirectoryActions::DisplayContents,
            deleted_folder: home_path.exists(),
            update_folder_content: true,
        }
    }

    pub fn update_current_directory(&mut self, path: &PathBuf) {
        self.previous_paths.push(self.current_path.clone());
        self.current_path = path.clone();
        self.update_folder_content = true;
    }

    pub fn fill_directory_content(&mut self) -> std::io::Result<()> {
        self.directory_content.clear();
        self.update_folder_content = false;

        let entries = match fs::read_dir(&self.current_path) {
            Ok(entries) => {
                self.deleted_folder = false;
                entries
            }
            Err(e) => {
                self.deleted_folder = true;
                return Err(e);
            }
        };

        self.directory_content.extend(
            entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
        );

        self.directory_content.sort();
        self.directory_content.sort_by_key(|key| {
            key.metadata().map(|metadata| metadata.is_file()).unwrap_or(true)
        });


        Ok(())
    }
}