#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actions;
mod icons_manager;
mod lang_string;
mod path_manager;
mod search_manager;
mod ui;

use crate::actions::Actions;
use crate::icons_manager::IconsManager;
use crate::lang_string::LangString;
use crate::path_manager::PathManager;
use crate::search_manager::SearchManager;
use crate::ui::{
    additional_info_panel, central_panel, display_path_panel, navigation_bar_panel,
    quick_access_panel,
};
use eframe::egui;
use egui::{Context, Id};

const APP_NAME: &str = "rocket";

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            cc.egui_ctx.set_theme(egui::Theme::Dark);

            Ok(Box::<Rocket>::default())
        }),
    )
}

struct Rocket<'a> {
    lang_string: LangString,
    path_manager: PathManager,
    search_manager: SearchManager,
    actions: Actions,
    icons_manager: IconsManager<'a>,
}

impl Default for Rocket<'_> {
    fn default() -> Self {
        let lang_string: LangString = LangString::new();
        let path_manager = PathManager::new();
        let search_manager = SearchManager::new();
        let actions = Actions::new();
        let icons_manager = IconsManager::new();
        Self {
            lang_string,
            path_manager,
            search_manager,
            actions,
            icons_manager,
        }
    }
}

impl eframe::App for Rocket<'_> {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left(Id::new("quick_access"))
            .resizable(true)
            .show(ctx, |ui| {
                quick_access_panel::show(ui);
            });

        egui::TopBottomPanel::top(Id::new("navigation_bar")).show(ctx, |ui| {
            navigation_bar_panel::show(
                ui,
                &self.lang_string,
                &mut self.path_manager,
                &mut self.search_manager,
            );
        });

        egui::TopBottomPanel::bottom(Id::new("display_path")).show(ctx, |ui| {
            display_path_panel::show(ui, &mut self.path_manager, &self.icons_manager.folder_icon);
        });

        egui::SidePanel::right(Id::new("additional_info"))
            .max_width(300.0)
            .min_width(200.0)
            .show(ctx, |ui| {
                additional_info_panel::show(
                    ui,
                    &self.lang_string,
                    &self.actions.select_action,
                    &self.icons_manager,
                );
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            central_panel::show(
                ui,
                &self.lang_string,
                &mut self.path_manager,
                &mut self.actions.select_action,
                &self.icons_manager,
            );
        });
    }
}
