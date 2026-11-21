mod decisions;
mod events;
mod models;
mod simulation;
mod ui;
mod world_gen;

use eframe::egui;
use ui::NationSimApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([840.0, 560.0])
            .with_title("SOVEREIGN // Grand Strategy Nation Simulator"),
        ..Default::default()
    };

    eframe::run_native(
        "SOVEREIGN // Grand Strategy Nation Simulator",
        options,
        Box::new(|cc| Ok(Box::new(NationSimApp::new(cc)))),
    )
}
