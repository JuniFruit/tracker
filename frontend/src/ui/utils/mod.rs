use eframe::{egui::Margin, epaint::Color32};

/// returns [`Color32`] shade of the rgb color.
/// Param factor specifies intesity of a shade. Range between -1 <= 0 <= 1;
/// If range exceeds limits returns the color from passed tuple.
pub fn shade_color(rgb: (u8, u8, u8, u8), factor: f32) -> Color32 {
    if factor > 1.0 || factor < -1.0 {
        return Color32::from_rgb(rgb.0, rgb.1, rgb.2);
    }
    let mut r = rgb.0 as f32;
    let mut g = rgb.1 as f32;
    let mut b = rgb.2 as f32;

    if factor < 0.0 {
        let factor = factor + 1.0;
        r *= factor;
        g *= factor;
        b *= factor;
    } else {
        r = (255.0 - r) * factor + r;
        g = (255.0 - g) * factor + g;
        b = (255.0 - g) * factor + b;
    }

    Color32::from_rgb(r as u8, g as u8, b as u8)
}

/// Returns [`Margin`] with left and right values specified.
pub fn get_mx(left: f32, right: f32) -> Margin {
    Margin {
        left,
        right,
        top: 0.0,
        bottom: 0.0,
    }
}

/// Returns [`Margin`] with top and bottom values specified.
pub fn get_my(top: f32, bottom: f32) -> Margin {
    Margin {
        left: 0.0,
        right: 0.0,
        top,
        bottom,
    }
}

pub fn format_time(secs: u64) -> String {
    if secs > (60 * 60) {
        return format!("{} hours", secs / (60 * 60));
    }
    if secs > 60 {
        return format!("{} minutes", secs / 60);
    }

    return format!("{} seconds", secs);
}
