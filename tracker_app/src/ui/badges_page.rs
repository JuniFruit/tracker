use eframe::{
    egui::{Label, Layout, RichText, ScrollArea, Separator, Ui},
    emath::Align,
};
use tracker_core::{
    store::apps_store::use_apps_store,
    tracking::badges::{Badge, BadgeRank},
};

use super::{
    configs::{ACCENT, ADDITIONAL, ADDITIONAL_2, ERROR_COLOR, HEADING_COLOR, SUB_HEADING_COLOR},
    utils::shade_color,
};

pub struct BadgesPage {
    list: Vec<AppItem>,
}

impl BadgesPage {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ui.add_space(5.0);
        ui.vertical_centered(|ui| ui.heading("Earned badges"));
        ui.add(Separator::default().spacing(20.0));
        self.make_list();
        let is_loading = use_apps_store()
            .lock()
            .unwrap()
            .selector()
            .is_fetching_tracked;
        if is_loading {
            ui.label("Loading");
            return;
        } else if self.list.len() == 0 {
            self.render_if_empty(ui);
            return;
        };

        self.render_list(ui);
        ui.add_space(5.0);
    }

    fn make_list(&mut self) {
        if use_apps_store()
            .lock()
            .unwrap()
            .selector()
            .tracked_apps
            .len()
            != self.list.len()
        {
            self.list = vec![];
            for item in &use_apps_store().lock().unwrap().selector().tracked_apps {
                self.list
                    .push(AppItem::new(&item.display_name, &item.badges));
            }
        }
    }

    fn render_list(&mut self, ui: &mut Ui) {
        ScrollArea::new([false, true]).show(ui, |ui| {
            for item in &mut self.list {
                item.render(ui);
                ui.separator();
            }
        });
    }

    fn render_if_empty(&self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.add_space(10.0);
            ui.add(Label::new(
                RichText::new("No apps are being tracked").color(SUB_HEADING_COLOR),
            ));
        });
    }
}

struct AppItem {
    name: *const String,
    badges: *const Vec<Badge>,
    badge_list: Vec<BadgeItem>,
}

impl AppItem {
    fn new(name: *const String, badges: &Vec<Badge>) -> Self {
        Self {
            name,
            badges,
            badge_list: vec![],
        }
    }

    fn render(&mut self, ui: &mut Ui) {
        ui.add_space(5.0);
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.colored_label(
                    HEADING_COLOR,
                    format!("{}", unsafe { self.name.as_ref().unwrap() }),
                );
            });
            self.render_badges(ui);
        });
        ui.add_space(5.0);
    }

    fn render_badges(&mut self, ui: &mut Ui) {
        self.make_list();

        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            for badge_item in &self.badge_list {
                badge_item.render(ui);
            }
        });
    }

    fn make_list(&mut self) {
        let badges = unsafe { self.badges.as_ref().unwrap() };

        if self.badge_list.len() != badges.len() {
            self.badge_list = vec![];
            for badge in badges.iter() {
                self.badge_list.push(BadgeItem {
                    rank: &badge.rank,
                    description: badge.description.to_owned(),
                })
            }
        }
    }
}

struct BadgeItem {
    rank: *const BadgeRank,
    description: String,
}

impl BadgeItem {
    fn render(&self, ui: &mut Ui) {
        let (icon, bg) = unsafe {
            match *self.rank {
                BadgeRank::Initial => ("🔓", shade_color(SUB_HEADING_COLOR.to_tuple(), 0.2)),
                BadgeRank::Common => ("🕑", shade_color((0, 255, 0, 1), -0.3)),
                BadgeRank::Rare => ("⏳", ADDITIONAL),
                BadgeRank::Experienced => ("🔥", shade_color(ACCENT.to_tuple(), 0.07)),
                BadgeRank::Advanced => ("🌀", shade_color(ERROR_COLOR.to_tuple(), 0.0)),
                BadgeRank::Pro => ("🕞", ADDITIONAL_2),
                BadgeRank::Insane => ("⏰", shade_color((64, 224, 208, 1), -0.2)),
                BadgeRank::Lunatic => ("🎴", shade_color(ERROR_COLOR.to_tuple(), -0.4)),
                BadgeRank::TouchGrass => ("🎉", shade_color(ACCENT.to_tuple(), -0.3)),
                BadgeRank::Master => ("💎", shade_color(ADDITIONAL_2.to_tuple(), -0.4)),
            }
        };

        ui.add_space(2.0);
        ui.button(RichText::new(icon).size(15.0).background_color(bg))
            .on_hover_text(RichText::new(&self.description).size(10.0).color(bg));
        ui.add_space(2.0);
    }
}
