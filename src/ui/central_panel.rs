use crate::actions::Actions;
use crate::actions::select_action::{SelectionMode, SelectionResult};
use crate::icons_manager::IconsManager;
use crate::lang_string::{LangKeys, LangString};
use crate::path_manager::PathManager;
use crate::ui::file_widget::file_widget;
use eframe::egui;
use egui::{Align, CursorIcon, Key, Pos2, Rect, Ui, Vec2};
use egui::{Response, ScrollArea};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::error_modal::ErrorModal;

pub fn show(
    ui: &mut Ui,
    lang_string: &LangString,
    general_error_modal: &mut ErrorModal,
    path_manager: &mut PathManager,
    actions: &mut Actions,
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
        if let Err(err) = path_manager.fill_directory_content() {
            general_error_modal.set_title_and_caption(&"Fill Directory Content Error".to_string(), &err.to_string());
            general_error_modal.set_visible(true);
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
        lang_string,
        general_error_modal,
        &path_manager.directory_content,
        actions,
        &icons_manager,
    ) {
        path_manager.update_current_directory(&new_current_path);
        actions.select_action.clear_selection();
    }
}

fn directory_builder(
    ui: &mut Ui,
    lang_string: &LangString,
    general_error_modal: &mut ErrorModal,
    directory_content: &[PathBuf],
    actions: &mut Actions,
    icons_manager: &IconsManager,
) -> Option<PathBuf> {
    let total_widgets = directory_content.len();
    let widget_row_height = ui.spacing().interact_size.y * 1.65;

    // todo: this is kind of ugly
    static SCROLL_OFFSET_Y: AtomicU32 = AtomicU32::new(0);

    // Update selection mode
    ui.ctx().input(|input_state| {
        // On Windows and Linux, set this to the same value as ctrl.
        // On Mac, this should be set whenever one of the ⌘ Command keys is down (same as mac_cmd)
        let new_mode = match (input_state.modifiers.shift, input_state.modifiers.command) {
            (true, _) => SelectionMode::Ranged,
            (false, true) => SelectionMode::Multiple,
            (false, false) => SelectionMode::Single,
        };

        actions.select_action.mode = new_mode;

        // This check prevents skipping file_widgets since multiple frames can be run,
        // even if the key_press is really short
        if actions.select_action.key_select.was_pressed {
            if input_state.key_down(actions.select_action.key_select.key) {
                return;
            }

            actions.select_action.key_select.was_pressed = false;
        }

        // Select a file_widget by key press (A-Z)
        let key_a_u8 = Key::A as u8;
        let key_z_u8 = Key::Z as u8;
        for key in input_state.keys_down.iter() {
            let key_u8 = *key as u8;

            if key_a_u8 <= key_u8 && key_u8 <= key_z_u8 {
                // IMPORTANT: egui Key::A (46) to ASCII 'A' (65) requires an offset of 19.
                // This is fragile if the egui Key enum order changes.
                let key_enum_to_ascii_offset = 19;
                let key_uppercase_char = (key_enum_to_ascii_offset + key_u8) as char;

                let start_idx = directory_content
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, f)| actions.select_action.files.contains(*f))
                    .map(|(idx, _)| idx + 1)
                    .unwrap_or(0);

                for idx in start_idx..directory_content.len() {
                    let file = &directory_content[idx];
                    let file_stem = file.file_stem().and_then(|stem| stem.to_str());
                    if let Some(file_stem) = file_stem {
                        if file_stem.to_uppercase().starts_with(key_uppercase_char) {
                            actions.select_action.select_file_by_key(file, *key);
                            break;
                        }
                    }
                }

                break;
            }
        }
    });

    // Which directory does the user want to go to?
    let mut new_current_path = None;

    let scroll_offset_y = ScrollArea::both()
        .show_rows(ui, widget_row_height, total_widgets, |ui, row_range| {
            // For directory_content[row_range] represents the viewable entries
            for entry in directory_content[row_range].iter() {
                let new_possible_path = entry
                    .file_name()
                    .and_then(|file_name| file_name.to_str())
                    .and_then(|file_name| {
                        file_row_ui(
                            ui,
                            general_error_modal,
                            directory_content,
                            entry,
                            &file_name.into(),
                            actions,
                            icons_manager,
                        )
                    });

                if new_possible_path.is_some() {
                    new_current_path = new_possible_path;
                }
            }

            if actions.select_action.key_select.scroll_to_widget
                && let SelectionResult::Single(path) =
                    actions.select_action.get_selected_files(lang_string)
            {
                let entry = directory_content
                    .iter()
                    .enumerate()
                    .find(|(_, entry)| entry.as_path() == path.as_path());
                if let Some((idx, _)) = entry {
                    let bits = SCROLL_OFFSET_Y.load(Ordering::Relaxed);
                    scroll_to_file_widget(ui, idx + 1, widget_row_height, f32::from_bits(bits));
                    actions.select_action.key_select.scroll_to_widget = false;
                }
            }
        })
        .state
        .offset
        .y;

    SCROLL_OFFSET_Y.store(scroll_offset_y.to_bits(), Ordering::Relaxed);

    new_current_path
}

fn scroll_to_file_widget(
    ui: &mut Ui,
    file_widget_idx: usize,
    file_widget_height: f32,
    scroll_offset_y: f32,
) {
    let spacing = ui.spacing().item_spacing.y;
    let y = file_widget_idx as f32 * (file_widget_height + spacing);
    let target_rect = Rect {
        min: Pos2::new(0.0, y - scroll_offset_y),
        max: Pos2::new(0.0, y + file_widget_height - scroll_offset_y),
    };

    ui.scroll_to_rect(target_rect, Some(Align::TOP));
}

fn file_row_ui(
    ui: &mut Ui,
    general_error_modal: &mut ErrorModal,
    directory_content: &[PathBuf],
    entry: &PathBuf,
    file_name: &String,
    actions: &mut Actions,
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
                file_widget(ui, actions.select_action.is_file_selected(entry), file_name);

            if file_widget_response.clicked() {
                actions
                    .select_action
                    .select_file(entry, Some(directory_content));
            }

            if file_widget_response.double_clicked() {
                if entry.is_dir() {
                    new_current_path = Some(entry.clone());
                } else {
                    if let Err(err) = opener::open(entry) {
                        general_error_modal.set_title_and_caption(&"Opener Error".to_string(), &err.to_string());
                        general_error_modal.set_visible(true);
                    }
                }
            }

            file_context_menu(&file_widget_response, entry, actions);
        });
    });

    new_current_path
}

fn file_context_menu(file_widget_response: &Response, entry: &PathBuf, actions: &mut Actions) {
    file_widget_response.context_menu(|ui| {
        // If the file is not selected, clear the selection
        // and add the current file to the selection
        if !actions.select_action.is_file_selected(entry) {
            actions.select_action.clear_selection();
            actions.select_action.select_file(entry, None);
        }

        if ui.button("Unselect (test)").clicked() {
            actions.select_action.deselect_file(entry);
        }
    });
}
