use crate::actions::select_action::SelectAction;
use crate::icons_manager::IconsManager;
use crate::lang_string::{LangKeys, LangString};
use chrono::{DateTime, Local};
use eframe::egui;
use egui::{Image, Ui, Vec2};
use std::path::Path;

pub fn show(
    ui: &mut Ui,
    lang_string: &LangString,
    select_action: &SelectAction,
    icons_manager: &IconsManager,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            if select_action.files.is_empty() {
                ui.label(lang_string.get(LangKeys::NothingSelected));
                return;
            }

            for file in &select_action.files {
                generate_file_info_group(ui, lang_string, file, icons_manager);
            }
        })
    });
}

fn generate_file_info_group(
    ui: &mut Ui,
    lang_string: &LangString,
    file: &Path,
    icons_manager: &IconsManager,
) {
    match file.metadata() {
        Ok(metadata) => {
            if let Some(file_name) = file.file_name()
                && let Some(file_name) = file_name.to_str()
            {
                ui.group(|ui| {
                    ui.add(
                        Image::new(icons_manager.get_icon(&file.into()).clone())
                            .fit_to_exact_size(Vec2::new(128.0, 128.0)),
                    );
                    ui.heading(file_name);

                    ui.group(|ui| {
                        if let Some(extension) = file.extension()
                            && let Some(extension) = extension.to_str()
                            && metadata.is_file()
                        {
                            ui.label(lang_string.get(LangKeys::Extension) + extension);
                        }

                        let bytes = metadata.len();
                        if metadata.is_file() {
                            ui.label(convert_bytes_size_to_human(bytes));
                        }

                        const DATE_TIME_FORMAT: &str = "%d/%m/%Y %T";
                        let mut date_time: DateTime<Local>;

                        if let Ok(time) = metadata.created() {
                            date_time = time.into();
                            ui.label(
                                lang_string.get(LangKeys::CreatedAt)
                                    + date_time.format(DATE_TIME_FORMAT).to_string().as_str(),
                            );
                        }
                        if let Ok(time) = metadata.accessed() {
                            date_time = time.into();
                            ui.label(
                                lang_string.get(LangKeys::AccessedAt)
                                    + date_time.format(DATE_TIME_FORMAT).to_string().as_str(),
                            );
                        }
                        if let Ok(time) = metadata.modified() {
                            date_time = time.into();
                            ui.label(
                                lang_string.get(LangKeys::ModifiedAt)
                                    + date_time.format(DATE_TIME_FORMAT).to_string().as_str(),
                            );
                        }
                    });
                });
            }
        }

        Err(e) => {
            // todo(bl4ze4447):
            println!("{}", e);
        }
    }
}

pub fn convert_bytes_size_to_human(bytes: u64) -> String {
    const B: u64 = 1;
    const KIB: u64 = 1024 * B;
    const MIB: u64 = KIB * KIB;
    const GIB: u64 = MIB * KIB;
    const TIB: u64 = GIB * KIB;

    match bytes {
        b if b < KIB => format!("{:.2} B", bytes),
        b if b < MIB => format!("{:.2} KiB", bytes / KIB),
        b if b < GIB => format!("{:.2} MiB", bytes / MIB),
        b if b < TIB => format!("{:.2} GiB", bytes / GIB),
        _ => format!("{:.2} TiB", bytes / TIB),
    }
}
