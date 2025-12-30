use std::path::PathBuf;
use eframe::egui;
use egui::{Button, Image, ImageSource, Label, TextStyle, Ui, Vec2, WidgetText};
use crate::path_manager::PathManager;

pub fn show(ui: &mut Ui, path_manager: &mut PathManager, folder_img: &ImageSource) {
    let available_space = ui.available_size();

    ui.allocate_ui_with_layout(
        available_space,
        egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
        |ui| {
            ui.horizontal(|ui| {
                ui.add(Image::from(folder_img.clone()).fit_to_exact_size(Vec2::new(64.0, 64.0)));

                let mut current_path = PathBuf::new();
                let path = path_manager.current_path.clone();
                let path_components = path.components();
                let parts: Vec<_> = path_components.map(|component| {
                    component.as_os_str()
                }).collect();

                for (path_idx, path_part) in parts.iter().enumerate() {
                    current_path.push(path_part);

                    if path_idx + 1 == parts.len() {
                        ui.add(
                            Label::new(
                                WidgetText::from(path_part.to_string_lossy())
                                    .text_style(TextStyle::Heading)
                            )
                        );
                    }
                    else if ui.add(
                        Button::new(
                            WidgetText::from(path_part.to_string_lossy())
                                .text_style(TextStyle::Heading))
                    ).clicked() {
                        path_manager.update_current_directory(&current_path);
                    }

                    if current_path.parent().is_some() {
                        ui.label("/");
                    }
                }
            });
        },
    );
}
