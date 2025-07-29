use eframe::egui;

use crate::gui::{FlightLogApp, waypoint::Waypoint};

/// Draws the top menu bar in the application GUI.
pub fn draw(app: &mut FlightLogApp, ui: &mut egui::Ui) {
    // Create a new menu bar at the top of the UI
    egui::MenuBar::new().ui(ui, |ui| {
        // "File" menu section
        ui.menu_button("File", |ui| {
            // "Reset" button: clears all waypoints and adds a default one
            if ui.button("Reset").clicked() {
                app.flp.waypoints.clear();
                app.flp.waypoints.push(Waypoint::default());
            }

            // "Quit" button: sends a command to close the application window
            if ui.button("Quit").clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });

        // "Compute" menu section
        ui.menu_button("Compute", |ui| {
            // "Top Of Descent" button: sets a flag to show the Top Of Descent (TOD)
            if ui.button("Top Of Descent").clicked() {
                app.show_tod = true;
            }
        });
    });
}
