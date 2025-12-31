use eframe::egui;
use egui::{CursorIcon, Image, ImageSource, Ui, Vec2};
use egui::ScrollArea;
use crate::actions::select_action::{SelectAction, SelectionMode};
use crate::lang_string::{LangKeys, LangString};
use crate::path_manager::PathManager;
use crate::ui::file_widget::file_widget;

pub fn show(ui: &mut Ui, lang_string: &LangString, path_manager: &mut PathManager, select_action: &mut SelectAction, folder_img: &ImageSource, file_img: &ImageSource) {
    if path_manager.update_folder_content {
        if path_manager.update_cursor_icon {
            ui.ctx().set_cursor_icon(CursorIcon::Wait);
            path_manager.update_cursor_icon = false;
            return;
        }

        ui.ctx().set_cursor_icon(CursorIcon::Default);

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

    let folder_img = Image::new(folder_img.clone()).fit_to_exact_size(Vec2::new(32.0, 32.0));
    let file_img = Image::new(file_img.clone()).fit_to_exact_size(Vec2::new(32.0, 32.0));
    directory_builder(ui, lang_string, path_manager, select_action, &folder_img, &file_img);
}

fn directory_builder(ui: &mut Ui, lang_string: &LangString, path_manager: &mut PathManager, select_action: &mut SelectAction, folder_img: &Image, file_img: &Image) {
    let total_widgets = path_manager.directory_content.len();
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

    ScrollArea::both().show_rows(ui, widget_row_height, total_widgets, |ui, row_range| {
        let viewable_content = path_manager.directory_content[row_range].to_vec();
        for entry in &viewable_content {
            let file_name = entry.file_name();
            if let Some(file_name) = file_name {
                ui.horizontal(|ui| {
                    if entry.is_dir() {
                        ui.add(folder_img.clone());
                    } else {
                        ui.add(file_img.clone());
                    }

                    ui.vertical_centered_justified(|ui| {
                        let file_widget_response = file_widget(ui, select_action.is_file_selected(entry), &file_name.to_string_lossy().to_string());

                        if file_widget_response.clicked() {
                            select_action.select_file(entry);
                        }

                        if file_widget_response.double_clicked() {
                            if entry.is_dir() {
                                path_manager.update_current_directory(entry);
                            }
                            else if entry.is_file() {
                                // todo(bl4ze4447): error modal to display information
                                if let Err(err) = opener::open(entry) {
                                }
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
}

