use eframe::egui::panel::Side;
use eframe::egui::{
    self, Context, Id, Label, Layout, RichText, Sense, SidePanel, TopBottomPanel, Ui,
};
use eframe::emath::{Align, Align2};
use eframe::epaint::FontId;

use super::basics::{get_icon_img, logo_btn, text_small_button, ImgIcons};
use super::configs::{get_def_frame, ACCENT, SUB_HEADING_COLOR, Y_PADDING};
use super::router::Routes;
use super::Main;

/* Ui that persists across the pages of the app. Header, footer and custom widow styles */

pub fn header(ctx: &Context, frame: &mut eframe::Frame, app: &mut Main) {
    TopBottomPanel::top("header_bar").show(&ctx, |ui| {
        title_bar_ui(ui, frame, "App Tracker");
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

struct SideMenuItem {
    icon: ImgIcons,
    title: String,
    route: Routes,
}

impl SideMenuItem {
    fn new(icon: ImgIcons, title: &str, route: Routes) -> Self {
        Self {
            icon,
            title: String::from(title),
            route,
        }
    }
    fn render(&self, ui: &mut Ui, is_active: bool, on_click: impl FnOnce(&Routes) -> ()) {
        let color = if is_active { ACCENT } else { SUB_HEADING_COLOR };

        ui.add_space(15.0);

        ui.horizontal(|ui| {
            ui.add_space(5.0);
            ui.add(get_icon_img(ui.ctx(), &self.icon, Some(20.0)).bg_fill(color));
            let nav_btn = ui
                .add(
                    Label::new(RichText::new(&self.title).size(20.0).color(color))
                        .sense(Sense::click()),
                )
                .on_hover_cursor(egui::CursorIcon::PointingHand);

            ui.add_space(3.0);

            if nav_btn.clicked() {
                on_click(&self.route);
            };
        });

        ui.add_space(5.0);
    }
}

pub fn side_menu(ctx: &Context, app: &mut Main) {
    let side_menu_data: [SideMenuItem; 3] = [
        SideMenuItem::new(ImgIcons::HomeIcon, "Home", Routes::Home),
        SideMenuItem::new(ImgIcons::HomeIcon, "List", Routes::AppPage),
        SideMenuItem::new(ImgIcons::HomeIcon, "Apps", Routes::NotTrackedApps),
    ];

    SidePanel::new(Side::Left, "side_menu")
        .frame(get_def_frame(ctx))
        .min_width(100.0)
        .resizable(false)
        .default_width(150.0)
        .show_separator_line(false)
        .show(ctx, |ui| {
            side_menu_data.into_iter().for_each(|item| {
                item.render(ui, app.current_route == item.route, |r| {
                    app.change_route(r.to_owned())
                });
            });
        });
}

fn title_bar_ui(ui: &mut egui::Ui, frame: &mut eframe::Frame, title: &str) {
    let title_bar_height = 32.0;
    let title_bar_rect = {
        let mut rect = ui.max_rect();
        rect.max.y = rect.min.y + title_bar_height;
        rect
    };

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
    text_small_button(ui, "❌", None, || frame.close());

    if frame.info().window_info.maximized {
        text_small_button(ui, "🗗", None, || frame.set_maximized(false));
    } else {
        text_small_button(ui, "🗗", None, || frame.set_maximized(true))
    };

    text_small_button(ui, "🗕", None, || frame.set_minimized(true));
}
