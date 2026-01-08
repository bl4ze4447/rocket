use egui::{Image, include_image};
use std::collections::HashMap;
use std::fs;
use std::fs::{ReadDir, read};
use std::path::PathBuf;

pub struct IconsManager<'a> {
    pub folder_icon: Image<'a>,
    file_icon: Image<'a>,
    icons: HashMap<String, Image<'a>>,
}

impl IconsManager<'_> {
    fn load_icons<'a>(entries: &mut ReadDir) -> HashMap<String, Image<'a>> {
        let mut icons = HashMap::new();
        for entry in entries.flatten() {
            let file_path = entry.path();

            let file_path_str = file_path.to_str();
            let extension = file_path.extension().and_then(|ext| ext.to_str());
            let file_stem = file_path.file_stem().and_then(|stem| stem.to_str());
            let is_file = entry.metadata().map(|metadata| metadata.is_file());

            if let (Some(extension), Some(file_stem), Ok(is_file), Some(file_path_str)) =
                (extension, file_stem, is_file, file_path_str)
            {
                assert!(is_file, "All files inside icons must be FILES");
                assert_eq!(extension, "png", "Extension must be png");

                match read(&file_path) {
                    Ok(raw_bytes) => {
                        let uri = format!("bytes://{}", file_path_str);
                        icons.insert(file_stem.into(), Image::from_bytes(uri, raw_bytes));
                    }

                    Err(e) => {
                        // todo: better error handling
                        println!("cannot read bytes: {}", e);
                    }
                }
            }
        }

        icons
    }

    pub fn new() -> Self {
        #[cfg(windows)]
        let (folder_icon, file_icon) = (
            include_image!(".\\resources\\icons\\default_folder.png"),
            include_image!(".\\resources\\icons\\default_file.png"),
        );
        #[cfg(unix)]
        let (folder_icon, file_icon) = (
            Image::new(include_image!("./resources/icons/default_folder.png")),
            Image::new(include_image!("./resources/icons/default_file.png")),
        );

        let mut icons = HashMap::new();
        let mut icons_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        icons_folder.push("src");
        icons_folder.push("resources");
        icons_folder.push("icons");

        match fs::read_dir(&icons_folder) {
            Ok(mut entries) => {
                icons = Self::load_icons(&mut entries);
            }

            Err(e) => {
                // todo: error modal
                println!("cannot read icons dir: {}", e);
            }
        }

        Self {
            folder_icon,
            file_icon,
            icons,
        }
    }

    pub fn get_icon(&'_ self, file: &PathBuf) -> &'_ Image<'_> {
        if file.is_dir() {
            return &self.folder_icon;
        }

        file.extension()
            .and_then(|extension| extension.to_str())
            .and_then(|extension| self.icons.get(extension))
            .unwrap_or(&self.file_icon)
    }
}
