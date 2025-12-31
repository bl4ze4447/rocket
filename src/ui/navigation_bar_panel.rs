use eframe::egui;
use egui::Ui;
use crate::lang_string::{LangKeys, LangString};
use crate::path_manager::PathManager;

/// Navigation bar panel
/// Implements:
/// * Go back
/// * Go forward

pub fn show(ui: &mut Ui, lang_string: &LangString, path_manager: &mut PathManager) {
    ui.horizontal(|ui| {
        if ui.button(lang_string.get(LangKeys::GoBack)).clicked() {
            if let Some(new_path) = path_manager.previous_paths.pop() {
                path_manager.next_paths.push(path_manager.current_path.clone());
                path_manager.current_path = new_path;
                path_manager.update_folder_content = true;
            }
        }

        if ui.button(lang_string.get(LangKeys::GoForward)).clicked() {
            if let Some(new_path) = path_manager.next_paths.pop() {
                path_manager.previous_paths.push(path_manager.current_path.clone());
                path_manager.current_path = new_path;
                path_manager.update_folder_content = true;
            }
        }
    });
}
