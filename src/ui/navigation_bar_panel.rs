use crate::lang_string::{LangKeys, LangString};
use crate::path_manager::PathManager;
use crate::search_manager::SearchManager;
use eframe::egui;
use egui::{Key, TextEdit, Ui};

pub fn show(
    ui: &mut Ui,
    lang_string: &LangString,
    path_manager: &mut PathManager,
    search_manager: &mut SearchManager,
) {
    ui.horizontal(|ui| {
        if ui.button(lang_string.get(LangKeys::GoBack)).clicked()
            && let Some(new_path) = path_manager.previous_paths.pop()
        {
            path_manager
                .next_paths
                .push(path_manager.current_path.clone());
            path_manager.current_path = new_path;
            path_manager.update_folder_content = true;
        }

        if ui.button(lang_string.get(LangKeys::GoForward)).clicked()
            && let Some(new_path) = path_manager.next_paths.pop()
        {
            path_manager
                .previous_paths
                .push(path_manager.current_path.clone());
            path_manager.current_path = new_path;
            path_manager.update_folder_content = true;
        }

        let response = ui.add(
            TextEdit::singleline(&mut search_manager.search_query).hint_text("Search for ..."),
        );
        if response.lost_focus() {
            response.ctx.input(|input_state| {
                if input_state.key_pressed(Key::Enter) {
                    // todo
                }
            });
        }
    });
}
