use crate::gui::{FlightLogApp, waypoint::Waypoint, wpt_solver};
use eframe::egui::{self, Button, DragValue, Label, TextEdit};

/// Draws the editable list of waypoints and their parameters.
pub fn draw(app: &mut FlightLogApp, ui: &mut egui::Ui) {
    ui.columns(13, |columns| {
        draw_header_row(columns);

        // Store auto_solve and angle_range in locals since we can't borrow app multiple times
        let auto_solve = app.auto_solve;
        let angle_range = app.angle_range.clone();
        let mut drop_target = None;

        for (i, wpt) in app.flp.waypoints.iter_mut().enumerate() {
            draw_waypoint_row(columns, wpt, i, auto_solve, &angle_range, &mut drop_target);
        }

        // Apply any delete operations after iteration
        if let Some(idx) = drop_target {
            app.drop = idx;
        }
    });
}

/// Draws the header row with column labels
fn draw_header_row(columns: &mut [egui::Ui]) {
    let headers = [
        "WPT",
        "Course",
        "Distance",
        "Altitude",
        "IAS",
        "W. Speed",
        "W. Dir",
        "TAS",
        "TH",
        "GS",
        "Flight time",
        "",
        "",
    ];

    for (col, header) in columns.iter_mut().zip(headers) {
        col.add(Label::new(header));
    }
}

/// Draws a single waypoint row
fn draw_waypoint_row(
    columns: &mut [egui::Ui],
    wpt: &mut Waypoint,
    index: usize,
    auto_solve: bool,
    angle_range: &std::ops::RangeInclusive<f64>,
    drop_target: &mut Option<usize>,
) {
    // Waypoint name
    columns[0].add(TextEdit::singleline(&mut wpt.name));

    // Main parameters
    columns[1].add(drag_value(
        &mut wpt.course,
        0.5,
        0,
        Some(angle_range.clone()),
    ));
    columns[2].add(drag_value(&mut wpt.distance, 0.05, 1, None));
    columns[3].add(drag_value(&mut wpt.altitude, 100.0, 0, None));
    columns[4].add(drag_value(&mut wpt.indicated_air_speed, 0.5, 0, None));
    columns[5].add(drag_value(&mut wpt.wind_speed, 0.5, 0, None));
    columns[6].add(drag_value(
        &mut wpt.wind_direction,
        0.5,
        0,
        Some(angle_range.clone()),
    ));

    // Auto-solve dependent parameters
    let enabled = !auto_solve;
    columns[7].add_enabled(enabled, drag_value(&mut wpt.true_air_speed, 0.5, 0, None));
    columns[8].add_enabled(
        enabled,
        drag_value(&mut wpt.true_heading, 0.5, 0, Some(angle_range.clone())),
    );
    columns[9].add_enabled(enabled, drag_value(&mut wpt.ground_speed, 0.5, 0, None));
    columns[10].add_enabled(enabled, flight_time_drag_value(&mut wpt.flight_time));

    // Solve button
    if auto_solve {
        // Show disabled solve button and auto-solve
        columns[11].add_enabled(false, Button::new("Solve"));
        wpt_solver::solve(wpt);
    } else {
        // Show enabled solve button
        if columns[11].button("Solve").clicked() {
            wpt_solver::solve(wpt);
        }
    }

    // Delete button - only show for non-first waypoints
    if index == 0 {
        // First waypoint - show empty space
        columns[12].add(Label::new(""));
    } else {
        // Non-first waypoint - show delete button
        if columns[12].button("Delete").clicked() {
            *drop_target = Some(index);
        }
    }
}

/// Creates a configured drag value
fn drag_value(
    value: &mut f64,
    speed: f64,
    decimals: usize,
    range: Option<std::ops::RangeInclusive<f64>>,
) -> DragValue {
    let mut dv = DragValue::new(value).speed(speed).max_decimals(decimals);
    if let Some(range) = range {
        dv = dv.range(range);
    }
    dv
}

/// Special drag value for flight time (hh:mm:ss format)
fn flight_time_drag_value(value: &mut f64) -> DragValue {
    DragValue::new(value)
        .speed(2)
        .range(0.0..=86399.0) // Max 24 hours
        .custom_formatter(|n, _| {
            let n = n as i32;
            let hours = n / 3600;
            let mins = (n / 60) % 60;
            let secs = n % 60;
            format!("{hours:02}:{mins:02}:{secs:02}")
        })
        .custom_parser(|s| {
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() == 3 {
                parts[0]
                    .parse::<i32>()
                    .and_then(|h| {
                        parts[1].parse::<i32>().and_then(|m| {
                            parts[2]
                                .parse::<i32>()
                                .map(|s| ((h * 3600) + (m * 60) + s) as f64)
                        })
                    })
                    .ok()
            } else {
                None
            }
        })
}
