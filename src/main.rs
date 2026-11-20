mod decisions;
mod events;
mod models;
mod simulation;
mod ui;

use eframe::egui;
use ui::NationSimApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1420.0, 880.0])
            .with_min_inner_size([1080.0, 680.0])
            .with_title("SOVEREIGN // Nation Simulator Dashboard"),
        ..Default::default()
    };

    eframe::run_native(
        "SOVEREIGN // Nation Simulator Dashboard",
        options,
        Box::new(|cc| Ok(Box::new(NationSimApp::new(cc)))),
    )
}
