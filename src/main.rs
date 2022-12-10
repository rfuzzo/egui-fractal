fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "fuzzy egui app",
        options,
        Box::new(|cc| Box::new(egui_fractal::WrapApp::new(cc))),
    );
}
