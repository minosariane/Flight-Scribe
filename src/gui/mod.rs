// Declare submodules that are part of the GUI
pub mod add_waypoint;
pub mod flightplan;
pub mod logger;
pub mod menu_bar;
pub mod tod;
pub mod waypoint;
pub mod wpt_solver;

// Import required modules and types
use crate::gui::{flightplan::FlightPlan, waypoint::Waypoint};
use eframe::egui::{self, Separator};
use std::ops::RangeInclusive;

/// The main application struct, holding state and configuration
pub struct FlightLogApp {
    flp: FlightPlan,                  // The flight plan, which holds a list of waypoints
    angle_range: RangeInclusive<f64>, // Valid range for angles (0° to 359°)
    show_tod: bool,                   // Whether the Top Of Descent panel is shown

    altitude: f64,     // Current altitude
    des_altitude: f64, // Desired altitude for descent
    ground_speed: f64, // Ground speed in knots or another unit
    aoa: f64,          // Angle of attack used for descent calculations

    auto_solve: bool, // Whether the application automatically solves the TOD
    drop: usize,      // Index of a waypoint to be removed
}

// Default values for a new instance of the app
impl Default for FlightLogApp {
    fn default() -> Self {
        Self {
            flp: FlightPlan {
                waypoints: vec![Waypoint::new()], // Start with one default waypoint
            },
            angle_range: RangeInclusive::new(0.0, 359.0),
            show_tod: false,

            altitude: 0.0,
            des_altitude: 0.0,
            ground_speed: 0.0,
            aoa: 3.33, // Default angle of descent

            auto_solve: false,

            drop: 0,
        }
    }
}

// Constructor for the app, currently just uses default values
impl FlightLogApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// Implementation of the eframe application behavior
impl eframe::App for FlightLogApp {
    /// Called every frame to update and render the GUI
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Draw the main central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // Draw the top menu bar
            menu_bar::draw(self, ui);

            // If the user has enabled the TOD view, draw it
            if self.show_tod {
                tod::draw(self, ctx);
            }

            // Scrollable panel for the logger and additional content
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(Separator::default().horizontal()); // Visual separator

                // Draw the log panel
                logger::draw(self, ui);
            });
        });

        // Bottom panel where the user can add new waypoints
        egui::TopBottomPanel::bottom("foot").show(ctx, |ui| {
            add_waypoint::draw(self, ui);
        });

        // If a waypoint is marked for deletion (drop > 0), remove it and reset the flag
        if self.drop > 0 {
            self.flp.waypoints.remove(self.drop);
            self.drop = 0;
        }
    }
}
