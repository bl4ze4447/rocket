use eframe::egui;
use egui::{Image, ImageSource, Ui, Vec2};
use egui::ScrollArea;
use crate::lang_string::{LangKeys, LangString};
use crate::path_manager::PathManager;
use crate::ui::file_widget::file_widget;

pub fn show(ui: &mut Ui, lang_string: &LangString, path_manager: &mut PathManager, folder_img: &ImageSource, file_img: &ImageSource) {
    if path_manager.update_folder_content {
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

    directory_builder(ui, lang_string, path_manager, folder_img, file_img);
}

fn directory_builder(ui: &mut Ui, lang_string: &LangString, path_manager: &mut PathManager, folder_img: &ImageSource, file_img: &ImageSource) {
    let total_widgets = path_manager.directory_content.len();
    let widget_row_height = ui.spacing().interact_size.y * 2.0;

    ScrollArea::both().show_rows(ui, widget_row_height, total_widgets, |ui, row_range| {
        let viewable_content = path_manager.directory_content[row_range].to_vec();
        for entry in &viewable_content {
            let file_name = entry.file_name();
            if let Some(file_name) = file_name {
                ui.horizontal(|ui| {
                    if entry.is_dir() {
                        ui.add(Image::new(folder_img.clone()).fit_to_exact_size(Vec2::new(32.0, 32.0)));
                    } else {
                        ui.add(Image::new(file_img.clone()).fit_to_exact_size(Vec2::new(32.0, 32.0)));
                    }

                    ui.vertical_centered_justified(|ui| {
                        if file_widget(ui, false, &file_name.to_string_lossy().to_string()).double_clicked() {
                            if entry.is_dir() {
                                path_manager.update_current_directory(entry);
                            }
                            else if entry.is_file() {
                                // todo(bl4ze4447): error modal to display information
                                if let Err(err) = opener::open(entry) {
                                }
                            }
                        }
                    });
                });
            }
        }
    });
}

