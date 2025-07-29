mod gui;

fn main() {
    // Configure the native window options for the application
    let native_options = eframe::NativeOptions {
        // Set the initial window size to 1200x300 pixels
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([1200.0, 300.0]),
        // Use other default options
        ..Default::default()
    };

    // Start the native eframe application
    let _ = eframe::run_native(
        "Flight Scribe",   // Window title
        native_options, // Window configuration
        Box::new(|cc| {
            // Create a new application instance
            Ok(Box::new(gui::FlightLogApp::new(cc)))
        }),
    );
}
