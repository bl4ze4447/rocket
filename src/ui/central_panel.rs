use crate::actions::select_action::{SelectAction, SelectionMode};
use crate::icons_manager::IconsManager;
use crate::lang_string::{LangKeys, LangString};
use crate::path_manager::PathManager;
use crate::ui::file_widget::file_widget;
use eframe::egui;
use egui::ScrollArea;
use egui::{CursorIcon, Image, Ui, Vec2};
use std::path::PathBuf;

pub fn show(
    ui: &mut Ui,
    lang_string: &LangString,
    path_manager: &mut PathManager,
    select_action: &mut SelectAction,
    icons_manager: &IconsManager,
) {
    if path_manager.update_folder_content {
        // Do we need to 'waste' this frame to update the cursor?
        if path_manager.update_cursor_icon {
            ui.ctx().set_cursor_icon(CursorIcon::Wait);
            path_manager.update_cursor_icon = false;
            return;
        }

        ui.ctx().set_cursor_icon(CursorIcon::Default);

        // todo
        if let Err(e) = path_manager.fill_directory_content() {
            ui.label(e.to_string());
            return;
        }
    }

    if path_manager.deleted_folder {
        ui.label(lang_string.get(LangKeys::DeletedFolder));
        return;
    }

    if path_manager.directory_content.is_empty() {
        ui.label(lang_string.get(LangKeys::EmptyFolder));
        return;
    }

    if let Some(new_current_path) = directory_builder(
        ui,
        &path_manager.directory_content,
        select_action,
        &icons_manager,
    ) {
        path_manager.update_current_directory(&new_current_path);
        select_action.clear_selection();
    }
}

fn directory_builder(
    ui: &mut Ui,
    directory_content: &[PathBuf],
    select_action: &mut SelectAction,
    icons_manager: &IconsManager,
) -> Option<PathBuf> {
    let total_widgets = directory_content.len();
    let widget_row_height = ui.spacing().interact_size.y * 2.0;

    ui.ctx().input(|input_state| {
        // On Windows and Linux, set this to the same value as ctrl.
        // On Mac, this should be set whenever one of the ⌘ Command keys is down (same as mac_cmd)
        let new_mode = match (input_state.modifiers.shift, input_state.modifiers.command) {
            (true, _) => SelectionMode::Ranged,
            (false, true) => SelectionMode::Multiple,
            (false, false) => SelectionMode::Single,
        };

        select_action.mode = new_mode;
    });

    let mut new_current_path = None;
    ScrollArea::both().show_rows(ui, widget_row_height, total_widgets, |ui, row_range| {
        let viewable_content = directory_content[row_range].to_vec();
        for entry in &viewable_content {
            let file_name = entry.file_name();
            if let Some(file_name) = file_name {
                ui.horizontal(|ui| {
                    ui.add(
                        Image::new(icons_manager.get_icon(&entry).clone())
                            .fit_to_exact_size(Vec2::new(32.0, 32.0)),
                    );

                    ui.vertical_centered_justified(|ui| {
                        let file_widget_response = file_widget(
                            ui,
                            select_action.is_file_selected(entry),
                            &file_name.to_string_lossy().to_string(),
                        );

                        if file_widget_response.clicked() {
                            select_action.select_file(entry);
                        }

                        if file_widget_response.double_clicked() {
                            if entry.is_dir() {
                                new_current_path = Some(entry.clone());
                            } else if entry.is_file() {
                                // todo(bl4ze4447): error modal to display information
                                if let Err(_) = opener::open(entry) {}
                            }
                        }

                        file_widget_response.context_menu(|ui| {
                            // If the file is not selected, clear the selection
                            // and add the current file to the selection
                            if !select_action.is_file_selected(entry) {
                                select_action.clear_selection();
                                select_action.select_file(entry);
                            }

                            if ui.button("Unselect (test)").clicked() {
                                select_action.deselect_file(entry);
                            }
                        });
                    });
                });
            }
        }
    });

    new_current_path
}
