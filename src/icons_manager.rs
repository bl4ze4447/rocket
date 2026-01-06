use egui::{ImageSource, include_image};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct IconsManager<'a> {
    pub folder_icon: ImageSource<'a>,
    file_icon: ImageSource<'a>,
    icons: HashMap<&'a str, ImageSource<'a>>,
}

impl IconsManager<'_> {
    pub fn new() -> Self {
        #[cfg(windows)]
        let (folder_icon, file_icon) = (
            include_image!(".\\resources\\img\\default_folder.png"),
            include_image!(".\\resources\\img\\default_file.png"),
        );
        #[cfg(unix)]
        let (folder_icon, file_icon) = (
            include_image!("./resources/img/default_folder.png"),
            include_image!("./resources/img/default_file.png"),
        );

        let mut icons = HashMap::new();
        icons.insert("pdf", include_image!("./resources/img/pdf.png"));

        Self {
            folder_icon,
            file_icon,
            icons,
        }
    }

    pub fn get_icon(&'_ self, file: &PathBuf) -> &'_ ImageSource<'_> {
        if file.is_dir() {
            return &self.folder_icon;
        }

        file.extension()
            .and_then(|extension| extension.to_str())
            .and_then(|extension| self.icons.get(extension))
            .unwrap_or(&self.file_icon)
    }
}
