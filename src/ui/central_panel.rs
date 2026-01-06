use crate::actions::select_action::{SelectAction, SelectionMode};
use crate::icons_manager::IconsManager;
use crate::lang_string::{LangKeys, LangString};
use crate::path_manager::PathManager;
use crate::ui::file_widget::file_widget;
use eframe::egui;
use egui::{CursorIcon, Ui, Vec2};
use egui::{Response, ScrollArea};
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

    // Update selection mode
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

    // Which directory does the user want to go to?
    let mut new_current_path = None;

    ScrollArea::both().show_rows(ui, widget_row_height, total_widgets, |ui, row_range| {
        // For directory_content[row_range] represents the viewable entries
        for (index, entry) in directory_content[row_range].iter().enumerate() {
            let new_possible_path = entry
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .and_then(|file_name| {
                    file_row_ui(
                        ui,
                        directory_content,
                        entry,
                        &file_name.into(),
                        select_action,
                        icons_manager,
                    )
                });

            if new_possible_path.is_some() {
                new_current_path = new_possible_path;
            }
        }
    });

    new_current_path
}

fn file_row_ui(
    ui: &mut Ui,
    directory_content: &[PathBuf],
    entry: &PathBuf,
    file_name: &String,
    select_action: &mut SelectAction,
    icons_manager: &IconsManager,
) -> Option<PathBuf> {
    // Which directory does the user want to go to?
    let mut new_current_path = None;

    ui.horizontal(|ui| {
        ui.add(
            icons_manager
                .get_icon(&entry)
                .clone()
                .fit_to_exact_size(Vec2::new(32.0, 32.0)),
        );

        ui.vertical_centered_justified(|ui| {
            let file_widget_response =
                file_widget(ui, select_action.is_file_selected(entry), file_name);

            if file_widget_response.clicked() {
                select_action.select_file(entry, Some(directory_content));
            }

            if file_widget_response.double_clicked() {
                if entry.is_dir() {
                    new_current_path = Some(entry.clone());
                } else {
                    // todo(bl4ze4447): error modal to display information
                    if let Err(_) = opener::open(entry) {}
                }
            }

            file_context_menu(&file_widget_response, entry, select_action);
        });
    });

    new_current_path
}

fn file_context_menu(
    file_widget_response: &Response,
    entry: &PathBuf,
    select_action: &mut SelectAction,
) {
    file_widget_response.context_menu(|ui| {
        // If the file is not selected, clear the selection
        // and add the current file to the selection
        if !select_action.is_file_selected(entry) {
            select_action.clear_selection();
            select_action.select_file(entry, None);
        }

        if ui.button("Unselect (test)").clicked() {
            select_action.deselect_file(entry);
        }
    });
}
