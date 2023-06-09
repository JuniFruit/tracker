use eframe::egui::{self, Context, Id, Label, Layout, Sense, TopBottomPanel};
use eframe::emath::{Align, Align2};
use eframe::epaint::FontId;

use super::basics::icon_button;
use super::{Main, Y_PADDING};

pub fn header(ctx: &Context, frame: &mut eframe::Frame, app: &mut Main) {
    TopBottomPanel::top("header_bar").show(&ctx, |ui| {
        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = ui.max_rect();
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };

        title_bar_ui(ui, frame, title_bar_rect, "App Tracker");

        ui.add_space(5.);
        eframe::egui::menu::bar(ui, |ui| {
            // Right side elements
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                let home_btn = ui.add(Label::new("Test"));
            });

            // Left side buttons
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {});
        });
        ui.add_space(5.);
    });
}

pub fn footer(ctx: &Context) {
    TopBottomPanel::bottom("footer_bar").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(Y_PADDING + 2.0);
            ui.add(Label::new("Created by Junifruit"));
            ui.hyperlink_to("GitHub", "https://github.com/JuniFruit/JuniFruit.github.io");
            ui.add_space(Y_PADDING + 2.0);
        })
    });
}

fn title_bar_ui(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    title_bar_rect: eframe::epaint::Rect,
    title: &str,
) {
    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.left_center(),
        Align2::LEFT_CENTER,
        title,
        FontId::proportional(10.0),
        ui.style().visuals.text_color(),
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        frame.drag_window();
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 2.0;
            ui.visuals_mut().button_frame = false;
            close_maximize_minimize(ui, frame);
        });
    });
}

/// Show some close/maximize/minimize buttons for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    icon_button(ui, "❌", "Close window", || frame.close());

    if frame.info().window_info.maximized {
        icon_button(ui, "🗗", "Minimize window", || frame.set_maximized(false));
    } else {
        icon_button(ui, "🗗", "Maximize window", || frame.set_maximized(true))
    };

    icon_button(ui, "🗕", "Collapse window", || frame.set_minimized(true));
}
