use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use sysinfo::Disks;

/// SearchManager should answer these questions:
pub struct SearchManager {
    pub search_content: Vec<PathBuf>,
    pub search_query: String,
    pub searching: bool,
    pub search_thread_sender: Sender<PathBuf>,
    pub search_thread_receiver: Receiver<PathBuf>,
}

impl SearchManager {
    pub fn new() -> Self {
        let search_channels = mpsc::channel();

        SearchManager {
            search_content: Vec::new(),
            search_query: String::new(),
            searching: false,
            search_thread_sender: search_channels.0,
            search_thread_receiver: search_channels.1,
        }
    }

    // Search for the query in the entire PC
    // Linux/MacOS: Searching from '/'
    // Windows: Searching inside all Volumes (C://, etc.)
    pub fn search(&mut self) {
        self.searching = true;
        self.search_query = self.search_query.to_lowercase();
        self.search_content.clear();

        if cfg!(unix) {
            //self.search_starting_from(&PathBuf::from("/"));
            self.searching = false;
            return;
        }

        if cfg!(windows) {
            let disks = Disks::new_with_refreshed_list();
            for disk in &disks {
                //let name = disk.mount_point().to_string_lossy().to_lowercase();
                //self.search_starting_from(&PathBuf::from(name));
            }
            self.searching = false;
        }
    }

    pub fn search_in_volume(&mut self, volume: &Path) {
        self.search_content.clear();
        self.searching = true;
        self.search_query = self.search_query.to_lowercase();

        self.searching = false;
    }

    // Internal function, used by search()
    pub fn search_starting_from(
        directory: &Path,
        search_query: &String,
        search_thread_sender: &Sender<PathBuf>,
    ) {
        match directory.read_dir() {
            Ok(entries) => {
                let entries: Vec<_> = entries.filter_map(|entry| entry.ok()).collect();
                entries
                    .par_iter()
                    .filter(|entry| {
                        entry
                            .file_name()
                            .to_string_lossy()
                            .to_lowercase()
                            .contains(search_query)
                    })
                    .for_each(|entry| {
                        let _ = search_thread_sender.send(entry.path());
                    });

                entries.par_iter().for_each(|entry| {
                    if entry.path().is_dir() {
                        Self::search_starting_from(
                            &entry.path(),
                            search_query,
                            search_thread_sender,
                        );
                    }
                });
            }
            Err(e) => {
                println!("Error reading directory: {}", e);
            }
        }
    }
}
