mod connectors;
mod core;
mod query;
mod storage;
mod ui;
mod viz;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Vizualizer",
        native_options,
        Box::new(|_cc| Ok(Box::new(ui::VizualizerApp::new()))),
    )
}
