use eframe::{NativeOptions, egui::{self, IconData, Vec2b}};

fn main() {
    let native_options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
        .with_title("Mini Lyrics")
        .with_always_on_top()
        .with_min_inner_size(egui::Vec2::new(320.0,216.0))
        .with_inner_size(egui::Vec2::new(480.0,270.0))
        .with_title_shown(false)
        .with_titlebar_shown(false)
        .with_active(false)
        .with_title_shown(false)
        .with_titlebar_buttons_shown(false)
        .with_decorations(false)
        .with_has_shadow(false)
        .with_icon(IconData::n)
        ,
        ..Default::default()
    };
    let _ = eframe::run_native(
        "My Lyrics",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}

#[derive(Default)]
struct App {}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}
impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("test");
        });
    }
}
