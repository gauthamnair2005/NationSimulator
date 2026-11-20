use eframe::egui::{self, Color32, CornerRadius, Margin, Pos2, Rect, RichText, Stroke, Vec2};

pub const BG_CANVAS: Color32 = Color32::from_rgb(13, 18, 26);
pub const BG_PANEL: Color32 = Color32::from_rgb(20, 27, 38);
pub const BG_CARD: Color32 = Color32::from_rgb(27, 36, 50);
pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(45, 58, 79);

pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(235, 241, 248);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(148, 163, 184);

pub const ACCENT_BLUE: Color32 = Color32::from_rgb(56, 189, 248);
pub const ACCENT_GREEN: Color32 = Color32::from_rgb(34, 197, 94);
pub const ACCENT_AMBER: Color32 = Color32::from_rgb(245, 158, 11);
pub const ACCENT_RED: Color32 = Color32::from_rgb(239, 68, 68);
pub const ACCENT_PURPLE: Color32 = Color32::from_rgb(167, 139, 250);

pub fn card_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(BG_CARD)
        .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
        .corner_radius(CornerRadius::same(6))
        .inner_margin(Margin::same(12))
}

pub fn panel_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(BG_PANEL)
        .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
        .corner_radius(CornerRadius::same(6))
        .inner_margin(Margin::same(10))
}

pub fn severity_color(value: f64, high_is_good: bool) -> Color32 {
    if high_is_good {
        if value >= 65.0 {
            ACCENT_GREEN
        } else if value >= 40.0 {
            ACCENT_AMBER
        } else {
            ACCENT_RED
        }
    } else if value <= 25.0 {
        ACCENT_GREEN
    } else if value <= 55.0 {
        ACCENT_AMBER
    } else {
        ACCENT_RED
    }
}

pub fn badge(ui: &mut egui::Ui, text: &str, color: Color32) {
    let bg = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 34);
    egui::Frame::new()
        .fill(bg)
        .stroke(Stroke::new(1.0_f32, color))
        .corner_radius(CornerRadius::same(4))
        .inner_margin(Margin::symmetric(7, 2))
        .show(ui, |ui| {
            ui.label(RichText::new(text).color(color).size(11.0).strong());
        });
}

pub fn kpi_card(
    ui: &mut egui::Ui,
    title: &str,
    value: &str,
    subtitle: &str,
    accent: Color32,
    width: f32,
) {
    egui::Frame::new()
        .fill(BG_CARD)
        .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
        .corner_radius(CornerRadius::same(6))
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_min_width(width);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(Vec2::new(3.0, 12.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, CornerRadius::same(1), accent);
                    ui.label(RichText::new(title).color(TEXT_MUTED).size(11.0).strong());
                });
                ui.add_space(2.0);
                ui.label(RichText::new(value).color(TEXT_PRIMARY).size(18.0).strong());
                ui.label(RichText::new(subtitle).color(accent).size(11.0));
            });
        });
}

pub fn meter_bar(ui: &mut egui::Ui, label: &str, value: f64, high_is_good: bool) {
    let clamped = value.clamp(0.0, 100.0);
    let col = severity_color(clamped, high_is_good);
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(TEXT_PRIMARY).size(12.5));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new(format!("{:.1}%", clamped)).color(col).size(12.5).strong());
        });
    });
    let desired = Vec2::new(ui.available_width().max(80.0), 7.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(3), Color32::from_rgb(16, 22, 31));
    let fill_w = rect.width() * (clamped as f32 / 100.0);
    let fill_rect = Rect::from_min_size(rect.min, Vec2::new(fill_w, rect.height()));
    painter.rect_filled(fill_rect, CornerRadius::same(3), col);
    ui.add_space(4.0);
}

pub fn signed_relation_bar(ui: &mut egui::Ui, value: f64) {
    let clamped = value.clamp(-100.0, 100.0);
    let col = if clamped >= 25.0 {
        ACCENT_GREEN
    } else if clamped >= -20.0 {
        ACCENT_AMBER
    } else {
        ACCENT_RED
    };
    let desired = Vec2::new(110.0, 8.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(3), Color32::from_rgb(16, 22, 31));
    let mid_x = rect.center().x;
    painter.line_segment(
        [Pos2::new(mid_x, rect.top()), Pos2::new(mid_x, rect.bottom())],
        Stroke::new(1.0_f32, BORDER_SUBTLE),
    );
    let frac = (clamped as f32) / 100.0;
    let bar_rect = if frac >= 0.0 {
        Rect::from_min_max(
            Pos2::new(mid_x, rect.top()),
            Pos2::new(mid_x + (rect.width() * 0.5) * frac, rect.bottom()),
        )
    } else {
        Rect::from_min_max(
            Pos2::new(mid_x + (rect.width() * 0.5) * frac, rect.top()),
            Pos2::new(mid_x, rect.bottom()),
        )
    };
    painter.rect_filled(bar_rect, CornerRadius::same(2), col);
}

pub fn sparkline_card(
    ui: &mut egui::Ui,
    title: &str,
    values: &[f64],
    unit_suffix: &str,
    color: Color32,
) {
    card_frame().show(ui, |ui| {
        let latest = values.last().copied().unwrap_or(0.0);
        let prev = if values.len() >= 2 {
            values[values.len() - 2]
        } else {
            latest
        };
        let delta = latest - prev;

        ui.horizontal(|ui| {
            ui.label(RichText::new(title).color(TEXT_MUTED).size(12.0).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("{:.1}{} ({:+.1})", latest, unit_suffix, delta))
                        .color(color)
                        .size(12.5)
                        .strong(),
                );
            });
        });
        ui.add_space(6.0);

        let height = 74.0;
        let width = ui.available_width().max(140.0);
        let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
        let painter = ui.painter();

        // Subtle chart frame & grid lines
        painter.rect_filled(rect, CornerRadius::same(4), BG_CANVAS);
        for i in 1..4 {
            let y = rect.top() + rect.height() * (i as f32 / 4.0);
            painter.line_segment(
                [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                Stroke::new(0.6_f32, Color32::from_rgb(28, 38, 52)),
            );
        }

        if values.is_empty() {
            return;
        }

        let mut min_v = values.iter().copied().fold(f64::INFINITY, f64::min);
        let mut max_v = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        if (max_v - min_v).abs() < 0.01 {
            min_v -= 1.0;
            max_v += 1.0;
        } else {
            let pad = (max_v - min_v) * 0.12;
            min_v -= pad;
            max_v += pad;
        }

        let n = values.len();
        let mut pts: Vec<Pos2> = Vec::with_capacity(n);
        for (i, &v) in values.iter().enumerate() {
            let x_frac = if n <= 1 { 1.0 } else { i as f32 / (n - 1) as f32 };
            let y_frac = ((v - min_v) / (max_v - min_v)) as f32;
            let x = rect.left() + 4.0 + x_frac * (rect.width() - 8.0);
            let y = rect.bottom() - 4.0 - y_frac * (rect.height() - 8.0);
            pts.push(Pos2::new(x, y));
        }

        if pts.len() == 1 {
            painter.circle_filled(pts[0], 3.0, color);
        } else {
            // Subtle shaded bands beneath line segments
            let fill_col = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 26);
            for win in pts.windows(2) {
                let p0 = win[0];
                let p1 = win[1];
                let poly = vec![
                    p0,
                    p1,
                    Pos2::new(p1.x, rect.bottom() - 2.0),
                    Pos2::new(p0.x, rect.bottom() - 2.0),
                ];
                painter.add(egui::Shape::convex_polygon(poly, fill_col, Stroke::NONE));
            }
            painter.add(egui::Shape::line(pts.clone(), Stroke::new(2.0_f32, color)));
            if let Some(&last) = pts.last() {
                painter.circle_filled(last, 3.0, color);
            }
        }
    });
}
