use eframe::egui::{self, Checkbox};

use crate::gui::{FlightLogApp, waypoint::Waypoint};

/// Draws the bottom panel with controls for adding a new waypoint and toggling auto-solve.
pub fn draw(app: &mut FlightLogApp, ui: &mut egui::Ui) {
    // Layout the elements horizontally (side by side)
    ui.horizontal(|ui| {
        // --- "Add Waypoint" button ---
        if ui.button("Add Waypoint").clicked() {
            // Create a new waypoint with default values
            let mut next_wpt = Waypoint::new();

            // If there is at least one waypoint, copy relevant values from the last one
            if let Some(last_wpt) = app.flp.waypoints.last() {
                next_wpt.altitude = last_wpt.altitude;
                next_wpt.wind_direction = last_wpt.wind_direction;
                next_wpt.wind_speed = last_wpt.wind_speed;
                next_wpt.indicated_air_speed = last_wpt.indicated_air_speed;
                next_wpt.true_air_speed = last_wpt.true_air_speed;
            }

            // Add the new waypoint to the flight plan
            app.flp.waypoints.push(next_wpt);
        }

        // --- Auto-solve checkbox ---
        // Toggles automatic solving of flight parameters for all waypoints
        ui.add(Checkbox::new(&mut app.auto_solve, "Auto-solve"));
    });
}
