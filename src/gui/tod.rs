use eframe::egui::{self, DragValue, Label, TextEdit, Window};

use crate::gui::FlightLogApp;

pub(crate) fn draw(app: &mut FlightLogApp, ctx: &egui::Context) {
    // Helper for editable widgets (label + widget)
    fn draw_editable_column<T: egui::Widget>(col: &mut egui::Ui, label: &str, widget: T) {
        col.add(Label::new(label));
        col.add(widget);
    }

    // Helper for read-only columns (label + disabled widget)
    fn draw_readonly_column(col: &mut egui::Ui, label: &str, text: String) {
        col.add(Label::new(label));
        col.add_enabled(false, TextEdit::singleline(&mut text.clone()));
        // clone car TextEdit requires mutable string ref
    }

    let tod = (app.altitude - app.des_altitude) / (app.aoa.to_radians().tan() * 6076.12);
    let desc_rate = app.ground_speed * 101.0 * app.aoa.to_radians().tan();

    Window::new("Top Of Descent")
        .default_size([600.0, 600.0])
        .open(&mut app.show_tod)
        .show(ctx, |ui| {
            ui.columns(6, |columns| {
                draw_editable_column(
                    &mut columns[0],
                    "Altitude",
                    DragValue::new(&mut app.altitude).speed(100).max_decimals(0),
                );
                draw_editable_column(
                    &mut columns[1],
                    "Desired Alt.",
                    DragValue::new(&mut app.des_altitude),
                );
                draw_editable_column(
                    &mut columns[2],
                    "Gnd Speed",
                    DragValue::new(&mut app.ground_speed),
                );
                draw_editable_column(
                    &mut columns[3],
                    "AoA",
                    DragValue::new(&mut app.aoa).speed(0.01).max_decimals(3),
                );

                draw_readonly_column(&mut columns[4], "ToD", format!("{:.1} nm", tod));
                draw_readonly_column(
                    &mut columns[5],
                    "Desc. rate",
                    format!("{:.0} ft/s", desc_rate),
                );
            });
        });
}
