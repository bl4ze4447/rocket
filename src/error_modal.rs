use egui::{Context, Window};

pub struct ErrorModal {
    pub title: String,
    pub caption: String,
    pub visible: bool,
}

impl ErrorModal {

    pub fn new() -> Self {
        Self {
            title: String::new(),
            caption: String::new(),
            visible: false,
        }
    }

    pub fn set_caption(&mut self, caption: &String) {
        self.caption = caption.clone();
    }

    pub fn set_title(&mut self, title: &String) {
        self.title = title.clone();
    }

    pub fn set_title_and_caption(&mut self, title: &String, caption: &String) {
        self.set_caption(caption);
        self.set_title(title);
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn render(&mut self, ctx: &Context) {
        if !self.visible {
            return;
        }

        Window::new(&self.title).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(&self.caption);
                ui.separator();
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Ok").clicked() {
                        self.visible = false;
                    }
                });
            });
        });
    }
}
