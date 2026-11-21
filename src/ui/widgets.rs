use crate::models::*;
use eframe::egui::{self, Color32, CornerRadius, FontId, Margin, Pos2, Rect, RichText, Stroke, Vec2};
use std::sync::atomic::{AtomicBool, Ordering};

static DARK_MODE: AtomicBool = AtomicBool::new(true);

pub fn set_dark_mode(dark: bool) {
    DARK_MODE.store(dark, Ordering::Relaxed);
}

pub fn is_dark_mode() -> bool {
    DARK_MODE.load(Ordering::Relaxed)
}

// Dynamic Theme Palette (Dark = Obsidian War Room | Light = Imperial Parchment Strategy)
pub fn bg_canvas() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(11, 16, 26)
    } else {
        Color32::from_rgb(235, 231, 220)
    }
}

pub fn bg_panel() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(18, 25, 38)
    } else {
        Color32::from_rgb(245, 242, 234)
    }
}

pub fn bg_card() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(24, 34, 50)
    } else {
        Color32::from_rgb(253, 251, 246)
    }
}

pub fn bg_hero() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(21, 31, 48)
    } else {
        Color32::from_rgb(248, 245, 236)
    }
}

pub fn bg_inset() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(12, 18, 29)
    } else {
        Color32::from_rgb(225, 220, 206)
    }
}

pub fn border_subtle() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(47, 63, 88)
    } else {
        Color32::from_rgb(196, 188, 169)
    }
}

pub fn border_gold() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(188, 146, 56)
    } else {
        Color32::from_rgb(168, 118, 24)
    }
}

pub fn text_primary() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(238, 244, 252)
    } else {
        Color32::from_rgb(22, 31, 46)
    }
}

pub fn text_muted() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(150, 168, 194)
    } else {
        Color32::from_rgb(86, 98, 118)
    }
}

pub fn accent_blue() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(56, 189, 248)
    } else {
        Color32::from_rgb(20, 102, 184)
    }
}

pub fn accent_cyan() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(34, 211, 238)
    } else {
        Color32::from_rgb(14, 116, 144)
    }
}

pub fn accent_green() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(34, 197, 94)
    } else {
        Color32::from_rgb(21, 128, 61)
    }
}

pub fn accent_amber() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(245, 158, 11)
    } else {
        Color32::from_rgb(180, 96, 10)
    }
}

pub fn accent_gold() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(250, 204, 21)
    } else {
        Color32::from_rgb(166, 112, 12)
    }
}

pub fn accent_red() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(239, 68, 68)
    } else {
        Color32::from_rgb(185, 28, 28)
    }
}

pub fn accent_purple() -> Color32 {
    if is_dark_mode() {
        Color32::from_rgb(167, 139, 250)
    } else {
        Color32::from_rgb(109, 40, 217)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UiMetrics {
    pub screen_w: f32,
    pub screen_h: f32,
    pub scale: f32,
    pub sidebar_w: f32,
    pub flag_w: f32,
    pub flag_h: f32,
}

impl UiMetrics {
    pub fn from_screen(rect: Rect) -> Self {
        let w = rect.width().max(640.0);
        let h = rect.height().max(480.0);
        let scale = (w / 1380.0).clamp(0.80, 1.38);
        let sidebar_w = (w * 0.168).clamp(184.0, 274.0);
        let flag_h = (28.0 * scale).clamp(22.0, 44.0);
        let flag_w = flag_h * 1.55;
        Self {
            screen_w: w,
            screen_h: h,
            scale,
            sidebar_w,
            flag_w,
            flag_h,
        }
    }

    pub fn fs(&self, base: f32) -> f32 {
        (base * self.scale).clamp(10.0, base * 1.38)
    }
}

pub fn fmt_billions(bn: f64) -> String {
    let a = bn.abs();
    if a >= 1.0e15 {
        format!("${:.2}Sp", bn / 1.0e15)
    } else if a >= 1.0e12 {
        format!("${:.2}Sx", bn / 1.0e12)
    } else if a >= 1.0e9 {
        format!("${:.2}Qi", bn / 1.0e9)
    } else if a >= 1.0e6 {
        format!("${:.2}Q", bn / 1.0e6)
    } else if a >= 1.0e3 {
        format!("${:.2}T", bn / 1.0e3)
    } else {
        format!("${:.1}B", bn)
    }
}

pub fn fmt_signed_billions(bn: f64) -> String {
    let a = bn.abs();
    if a >= 1.0e15 {
        format!("{:+.2}Sp", bn / 1.0e15)
    } else if a >= 1.0e12 {
        format!("{:+.2}Sx", bn / 1.0e12)
    } else if a >= 1.0e9 {
        format!("{:+.2}Qi", bn / 1.0e9)
    } else if a >= 1.0e6 {
        format!("{:+.2}Q", bn / 1.0e6)
    } else if a >= 1.0e3 {
        format!("{:+.2}T", bn / 1.0e3)
    } else {
        format!("{:+.1}B", bn)
    }
}

pub fn card_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(bg_card())
        .stroke(Stroke::new(1.0_f32, border_subtle()))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(Margin::same(12))
}

pub fn hero_card_frame(border_col: Color32) -> egui::Frame {
    egui::Frame::new()
        .fill(bg_hero())
        .stroke(Stroke::new(1.4_f32, border_col))
        .corner_radius(CornerRadius::same(10))
        .inner_margin(Margin::same(12))
}

pub fn panel_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(bg_panel())
        .stroke(Stroke::new(1.0_f32, border_subtle()))
        .corner_radius(CornerRadius::same(7))
        .inner_margin(Margin::same(10))
}

pub fn severity_color(value: f64, high_is_good: bool) -> Color32 {
    if high_is_good {
        if value >= 65.0 {
            accent_green()
        } else if value >= 40.0 {
            accent_amber()
        } else {
            accent_red()
        }
    } else if value <= 25.0 {
        accent_green()
    } else if value <= 55.0 {
        accent_amber()
    } else {
        accent_red()
    }
}

pub fn badge(ui: &mut egui::Ui, text: &str, color: Color32) {
    badge_scaled(ui, text, color, 11.0);
}

pub fn badge_scaled(ui: &mut egui::Ui, text: &str, color: Color32, font_size: f32) {
    let alpha = if is_dark_mode() { 36 } else { 24 };
    let bg = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);
    let pad_x = (font_size * 0.65).round() as i8;
    let pad_y = (font_size * 0.22).round().max(2.0) as i8;
    egui::Frame::new()
        .fill(bg)
        .stroke(Stroke::new(1.0_f32, color))
        .corner_radius(CornerRadius::same(5))
        .inner_margin(Margin::symmetric(pad_x, pad_y))
        .show(ui, |ui| {
            ui.label(RichText::new(text).color(color).size(font_size).strong());
        });
}

pub fn draw_status_flag_chip(
    ui: &mut egui::Ui,
    icon: &str,
    label: &str,
    tooltip: &str,
    color: Color32,
    scale: f32,
) {
    let alpha = if is_dark_mode() { 26 } else { 20 };
    let bg = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);
    let border = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 170);
    let pad_x = (8.0 * scale).clamp(6.0, 12.0) as i8;
    let pad_y = (4.5 * scale).clamp(3.0, 7.0) as i8;
    let resp = egui::Frame::new()
        .fill(bg)
        .stroke(Stroke::new(1.0_f32, border))
        .corner_radius(CornerRadius::same(6))
        .inner_margin(Margin::symmetric(pad_x, pad_y))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 5.0 * scale;
                let bar_h = (13.0 * scale).clamp(11.0, 18.0);
                let (bar_rect, _) =
                    ui.allocate_exact_size(Vec2::new(3.0 * scale, bar_h), egui::Sense::hover());
                ui.painter()
                    .rect_filled(bar_rect, CornerRadius::same(2), color);
                ui.label(
                    RichText::new(format!("{} {}", icon, label))
                        .color(color)
                        .size((11.2 * scale).clamp(10.0, 14.5))
                        .strong(),
                );
            });
        })
        .response;
    resp.on_hover_text(tooltip);
}

pub fn kpi_card(
    ui: &mut egui::Ui,
    title: &str,
    value: &str,
    subtitle: &str,
    accent: Color32,
    width: f32,
) {
    let scale = (width / 155.0).clamp(0.85, 1.25);
    let top_tint = Color32::from_rgba_unmultiplied(
        accent.r(),
        accent.g(),
        accent.b(),
        if is_dark_mode() { 24 } else { 32 },
    );
    egui::Frame::new()
        .fill(bg_card())
        .stroke(Stroke::new(
            1.1_f32,
            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 110),
        ))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_min_width(width);
            let card_rect = ui.max_rect();
            let glow_rect = Rect::from_min_size(
                Pos2::new(card_rect.left() - 6.0, card_rect.top() - 6.0),
                Vec2::new(width + 12.0, 3.0),
            );
            ui.painter()
                .rect_filled(glow_rect, CornerRadius::same(2), top_tint);

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let (rect, _) =
                        ui.allocate_exact_size(Vec2::new(3.5, 12.0 * scale), egui::Sense::hover());
                    ui.painter()
                        .rect_filled(rect, CornerRadius::same(2), accent);
                    ui.label(
                        RichText::new(title)
                            .color(text_muted())
                            .size((10.8 * scale).clamp(9.5, 13.0))
                            .strong(),
                    );
                });
                ui.add_space(2.0);
                ui.label(
                    RichText::new(value)
                        .color(text_primary())
                        .size((17.5 * scale).clamp(15.0, 22.0))
                        .strong(),
                );
                ui.label(
                    RichText::new(subtitle)
                        .color(accent)
                        .size((11.0 * scale).clamp(10.0, 13.5)),
                );
            });
        });
}

pub fn meter_bar(ui: &mut egui::Ui, label: &str, value: f64, high_is_good: bool) {
    let clamped = value.clamp(0.0, 100.0);
    let col = severity_color(clamped, high_is_good);
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(text_primary()).size(12.5));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(format!("{:.1}%", clamped))
                    .color(col)
                    .size(12.5)
                    .strong(),
            );
        });
    });
    let desired = Vec2::new(ui.available_width().max(80.0), 8.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(4), bg_inset());
    let fill_w = rect.width() * (clamped as f32 / 100.0);
    if fill_w > 1.0 {
        let fill_rect = Rect::from_min_size(rect.min, Vec2::new(fill_w, rect.height()));
        painter.rect_filled(fill_rect, CornerRadius::same(4), col);
        let sheen_rect = Rect::from_min_size(rect.min, Vec2::new(fill_w, rect.height() * 0.42));
        painter.rect_filled(
            sheen_rect,
            CornerRadius::same(3),
            Color32::from_rgba_unmultiplied(255, 255, 255, 42),
        );
    }
    ui.add_space(4.0);
}

pub fn signed_relation_bar(ui: &mut egui::Ui, value: f64) {
    let clamped = value.clamp(-100.0, 100.0);
    let col = if clamped >= 25.0 {
        accent_green()
    } else if clamped >= -20.0 {
        accent_amber()
    } else {
        accent_red()
    };
    let desired = Vec2::new(110.0, 9.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(4), bg_inset());
    let mid_x = rect.center().x;
    painter.line_segment(
        [Pos2::new(mid_x, rect.top()), Pos2::new(mid_x, rect.bottom())],
        Stroke::new(1.0_f32, border_subtle()),
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
    painter.rect_filled(bar_rect, CornerRadius::same(3), col);
}

pub fn draw_war_frontline_bar(
    ui: &mut egui::Ui,
    our_name: &str,
    enemy_name: &str,
    war_score: f64,
) {
    let ws = war_score.clamp(-100.0, 100.0) as f32;
    let frac = (ws + 100.0) / 200.0;
    let annex_frac = (70.0 + 100.0) / 200.0;

    ui.horizontal(|ui| {
        ui.label(
            RichText::new(format!("⚔ FRONT: {}", our_name))
                .color(accent_green())
                .size(11.0)
                .strong(),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(format!("{} 🛡", enemy_name))
                    .color(accent_red())
                    .size(11.0)
                    .strong(),
            );
        });
    });

    let desired = Vec2::new(ui.available_width().max(160.0), 14.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let p = ui.painter();

    p.rect_filled(
        rect,
        CornerRadius::same(4),
        Color32::from_rgba_unmultiplied(210, 48, 48, 125),
    );
    let split_x = rect.left() + rect.width() * frac;
    let our_rect = Rect::from_min_max(rect.min, Pos2::new(split_x, rect.bottom()));
    p.rect_filled(
        our_rect,
        CornerRadius::same(4),
        Color32::from_rgba_unmultiplied(30, 185, 90, 185),
    );
    let mid_x = rect.center().x;
    p.line_segment(
        [Pos2::new(mid_x, rect.top()), Pos2::new(mid_x, rect.bottom())],
        Stroke::new(1.0_f32, Color32::from_white_alpha(150)),
    );
    let annex_x = rect.left() + rect.width() * annex_frac;
    p.line_segment(
        [
            Pos2::new(annex_x, rect.top() - 1.0),
            Pos2::new(annex_x, rect.bottom() + 1.0),
        ],
        Stroke::new(2.0_f32, accent_gold()),
    );
    p.circle_filled(Pos2::new(split_x, rect.center().y), 5.0, text_primary());
    p.circle_stroke(
        Pos2::new(split_x, rect.center().y),
        5.0,
        Stroke::new(1.5_f32, bg_canvas()),
    );
    p.rect_stroke(
        rect,
        CornerRadius::same(4),
        Stroke::new(1.0_f32, border_subtle()),
        egui::StrokeKind::Inside,
    );
    ui.label(
        RichText::new(format!(
            "War Score: {:+.0}%  |  Gold tick = +70% Direct Annexation  |  +95% = Total Capitulation",
            war_score
        ))
        .color(if war_score >= 70.0 { accent_gold() } else { text_muted() })
        .size(10.5),
    );
}

fn draw_diamond(painter: &egui::Painter, center: Pos2, rx: f32, ry: f32, color: Color32) {
    let pts = vec![
        Pos2::new(center.x, center.y - ry),
        Pos2::new(center.x + rx, center.y),
        Pos2::new(center.x, center.y + ry),
        Pos2::new(center.x - rx, center.y),
    ];
    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

fn draw_hex(painter: &egui::Painter, center: Pos2, r: f32, color: Color32) {
    let mut pts = Vec::with_capacity(6);
    for i in 0..6 {
        let a = (i as f32) * std::f32::consts::FRAC_PI_3;
        pts.push(Pos2::new(center.x + r * a.cos(), center.y + r * a.sin()));
    }
    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

fn draw_compass_star(painter: &egui::Painter, center: Pos2, outer_r: f32, color: Color32) {
    let inner_r = outer_r * 0.34;
    let mut pts = Vec::with_capacity(8);
    for i in 0..8 {
        let angle = -std::f32::consts::FRAC_PI_2 + (i as f32) * std::f32::consts::FRAC_PI_4;
        let r = if i % 2 == 0 { outer_r } else { inner_r };
        pts.push(Pos2::new(
            center.x + r * angle.cos(),
            center.y + r * angle.sin(),
        ));
    }
    for i in 0..8 {
        painter.add(egui::Shape::convex_polygon(
            vec![center, pts[i], pts[(i + 1) % 8]],
            color,
            Stroke::NONE,
        ));
    }
}

fn paint_flag_finish(painter: &egui::Painter, rect: Rect, gold_border: bool) {
    let sheen = vec![
        rect.left_top(),
        Pos2::new(rect.left() + rect.width() * 0.50, rect.top()),
        Pos2::new(rect.left() + rect.width() * 0.22, rect.bottom()),
        rect.left_bottom(),
    ];
    painter.add(egui::Shape::convex_polygon(
        sheen,
        Color32::from_rgba_unmultiplied(255, 255, 255, 24),
        Stroke::NONE,
    ));
    let shade = Rect::from_min_max(
        Pos2::new(rect.left() + rect.width() * 0.64, rect.top()),
        Pos2::new(rect.left() + rect.width() * 0.78, rect.bottom()),
    );
    painter.rect_filled(
        shade,
        CornerRadius::ZERO,
        Color32::from_rgba_unmultiplied(0, 0, 0, 22),
    );
    let border_col = if gold_border {
        Color32::from_rgb(240, 192, 48)
    } else {
        border_subtle()
    };
    painter.rect_stroke(
        rect,
        CornerRadius::same(3),
        Stroke::new(if gold_border { 1.6_f32 } else { 1.1_f32 }, border_col),
        egui::StrokeKind::Inside,
    );
}

pub fn draw_nation_flag_from_spec(
    ui: &mut egui::Ui,
    regime: RegimeType,
    economy: EconomicSystem,
    religion: ReligionPolicy,
    god_mode: bool,
    hegemon: bool,
    width: f32,
    height: f32,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::click());
    paint_nation_flag_at(
        ui.painter(),
        rect,
        regime,
        economy,
        religion,
        god_mode,
        hegemon,
    );
    resp
}

/// 100% Original Fictional Heraldry for Player Nations (no resemblance to real-world flags)
pub fn paint_nation_flag_at(
    painter: &egui::Painter,
    rect: Rect,
    regime: RegimeType,
    economy: EconomicSystem,
    religion: ReligionPolicy,
    god_mode: bool,
    hegemon: bool,
) {
    let w = rect.width();
    let h = rect.height();
    let c = rect.center();
    let gold = Color32::from_rgb(245, 198, 42);
    let silver = Color32::from_rgb(236, 242, 248);

    match regime {
        RegimeType::LiberalDemocracy => {
            // Sapphire & Silver diagonal split with Twin Compass Crest
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(24, 92, 168));
            let diag = vec![rect.right_top(), rect.right_bottom(), Pos2::new(rect.left() + w * 0.28, rect.bottom())];
            painter.add(egui::Shape::convex_polygon(diag, Color32::from_rgb(15, 148, 136), Stroke::NONE));
            draw_diamond(painter, c, w * 0.20, h * 0.34, silver);
            draw_compass_star(painter, c, h * 0.24, gold);
        }
        RegimeType::SocialDemocracy => {
            // Deep Teal & Amber chevron banner with Golden Hexagon
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(15, 98, 118));
            let chev = vec![rect.left_top(), Pos2::new(c.x, c.y), rect.left_bottom()];
            painter.add(egui::Shape::convex_polygon(chev, Color32::from_rgb(217, 132, 26), Stroke::NONE));
            draw_hex(painter, Pos2::new(c.x + w * 0.14, c.y), h * 0.24, silver);
            draw_compass_star(painter, Pos2::new(c.x + w * 0.14, c.y), h * 0.16, gold);
        }
        RegimeType::OnePartyCommunist => {
            // Deep Terracotta-Crimson & Obsidian diagonal gear-diamond banner
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(156, 26, 48));
            let corner = vec![rect.left_bottom(), rect.right_bottom(), rect.right_top()];
            painter.add(egui::Shape::convex_polygon(corner, Color32::from_rgb(32, 34, 46), Stroke::NONE));
            draw_hex(painter, c, h * 0.29, gold);
            draw_diamond(painter, c, h * 0.16, h * 0.16, Color32::from_rgb(156, 26, 48));
        }
        RegimeType::MilitaryDictatorship => {
            // Gunmetal Slate & Bronze twin-chevron with Steel Shield
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(44, 54, 68));
            let chev = vec![rect.left_top(), Pos2::new(rect.left() + w * 0.44, c.y), rect.left_bottom()];
            painter.add(egui::Shape::convex_polygon(chev, Color32::from_rgb(176, 88, 32), Stroke::NONE));
            draw_diamond(painter, Pos2::new(c.x + w * 0.12, c.y), w * 0.16, h * 0.28, gold);
        }
        RegimeType::AbsoluteAutocracy => {
            // Royal Amethyst & Obsidian with Imperial Gold Band & Compass Crest
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w, h * 0.5)), CornerRadius::same(2), Color32::from_rgb(92, 32, 142));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), c.y), Vec2::new(w, h * 0.5)), CornerRadius::same(2), Color32::from_rgb(20, 26, 40));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), c.y - h * 0.08), Vec2::new(w, h * 0.16)), CornerRadius::ZERO, gold);
            draw_compass_star(painter, c, h * 0.28, silver);
        }
        RegimeType::TheocraticRepublic => {
            // Deep Ochre-Bronze & Ivory geometric sunburst (no real-world religious symbols)
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(142, 96, 28));
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w * 0.26, h)), CornerRadius::same(2), silver);
            draw_compass_star(painter, Pos2::new(rect.left() + w * 0.62, c.y), h * 0.30, gold);
            painter.circle_filled(Pos2::new(rect.left() + w * 0.62, c.y), h * 0.11, Color32::from_rgb(28, 88, 128));
        }
        RegimeType::OligarchicRepublic => {
            // Midnight Navy & Emerald-Gold Lozenge Banner
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(18, 38, 76));
            draw_diamond(painter, c, w * 0.34, h * 0.38, gold);
            draw_hex(painter, c, h * 0.20, Color32::from_rgb(16, 168, 128));
        }
    }

    if economy == EconomicSystem::WarEconomy {
        painter.circle_filled(Pos2::new(rect.right() - w * 0.14, rect.top() + h * 0.22), h * 0.10, Color32::from_rgb(239, 68, 68));
    } else if religion == ReligionPolicy::TheocraticLaw && regime != RegimeType::TheocraticRepublic {
        painter.circle_stroke(c, h * 0.31, Stroke::new(1.3_f32, gold));
    }

    if god_mode || hegemon {
        painter.circle_stroke(c, h * 0.37, Stroke::new(1.6_f32, gold));
    }

    paint_flag_finish(painter, rect, god_mode || hegemon);
}

pub fn draw_foreign_flag(
    ui: &mut egui::Ui,
    power: &ForeignPower,
    width: f32,
    height: f32,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
    paint_foreign_flag_at(ui.painter(), rect, power);
    resp
}

/// 100% Original Fictional Flags for all 18 World Powers + Emergent/Rebel Nations
pub fn paint_foreign_flag_at(painter: &egui::Painter, rect: Rect, power: &ForeignPower) {
    let w = rect.width();
    let h = rect.height();
    let c = rect.center();
    let gold = Color32::from_rgb(245, 198, 42);
    let silver = Color32::from_rgb(236, 242, 248);

    match power.id {
        ForeignPowerId::AtlanticRepublic => {
            // Aethelgard: Cobalt & Silver geometric chevron + Twin Gold Compass
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(22, 66, 138));
            let tri = vec![rect.left_top(), Pos2::new(c.x, c.y), rect.left_bottom()];
            painter.add(egui::Shape::convex_polygon(tri, silver, Stroke::NONE));
            draw_compass_star(painter, Pos2::new(rect.left() + w * 0.20, c.y), h * 0.20, Color32::from_rgb(22, 66, 138));
            draw_compass_star(painter, Pos2::new(c.x + w * 0.22, c.y), h * 0.22, gold);
        }
        ForeignPowerId::PeoplesUnion => {
            // Volkgrad: Burgundy & Charcoal diagonal + Golden Hex Gear
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(138, 28, 52));
            let low = vec![rect.left_bottom(), rect.right_bottom(), rect.right_top()];
            painter.add(egui::Shape::convex_polygon(low, Color32::from_rgb(36, 40, 52), Stroke::NONE));
            draw_hex(painter, c, h * 0.27, gold);
        }
        ForeignPowerId::EurasianFederation => {
            // Borealis: Deep Indigo & Bronze split + Silver Twin-Diamond
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w, h * 0.5)), CornerRadius::same(1), Color32::from_rgb(30, 42, 92));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), c.y), Vec2::new(w, h * 0.5)), CornerRadius::same(1), Color32::from_rgb(138, 82, 42));
            draw_diamond(painter, Pos2::new(c.x - w * 0.14, c.y), w * 0.11, h * 0.26, silver);
            draw_diamond(painter, Pos2::new(c.x + w * 0.14, c.y), w * 0.11, h * 0.26, gold);
        }
        ForeignPowerId::CrescentCoalition => {
            // Qadira Sun Dominion: Amber Ochre & Deep Sapphire Sunburst
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(186, 118, 28));
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w, h * 0.22)), CornerRadius::same(1), Color32::from_rgb(20, 54, 108));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), rect.bottom() - h * 0.22), Vec2::new(w, h * 0.22)), CornerRadius::same(1), Color32::from_rgb(20, 54, 108));
            draw_compass_star(painter, c, h * 0.26, silver);
        }
        ForeignPowerId::BorderlandRepublic => {
            // Kestrel Frontier: Teal & Slate vertical split + Silver Fortress Diamond
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w * 0.5, h)), CornerRadius::same(1), Color32::from_rgb(18, 128, 124));
            painter.rect_filled(Rect::from_min_size(Pos2::new(c.x, rect.top()), Vec2::new(w * 0.5, h)), CornerRadius::same(1), Color32::from_rgb(54, 66, 86));
            draw_diamond(painter, c, w * 0.18, h * 0.30, silver);
            painter.circle_filled(c, h * 0.10, gold);
        }
        ForeignPowerId::GlobalMonetaryCouncil => {
            // GMC: Teal-Indigo with Golden Octagonal Lattice
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(18, 86, 146));
            painter.circle_stroke(c, h * 0.30, Stroke::new(1.5_f32, gold));
            draw_diamond(painter, c, h * 0.22, h * 0.22, silver);
            painter.circle_filled(c, h * 0.08, Color32::from_rgb(18, 86, 146));
        }
        ForeignPowerId::BharatUnion => {
            // Indravarta Technate: Deep Bronze, Teal & Amber Geometric Lotus-Hex
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(16, 104, 112));
            let band = vec![rect.left_top(), Pos2::new(rect.left() + w * 0.38, rect.top()), Pos2::new(rect.left() + w * 0.18, rect.bottom()), rect.left_bottom()];
            painter.add(egui::Shape::convex_polygon(band, Color32::from_rgb(204, 118, 36), Stroke::NONE));
            draw_hex(painter, Pos2::new(c.x + w * 0.10, c.y), h * 0.26, gold);
            draw_compass_star(painter, Pos2::new(c.x + w * 0.10, c.y), h * 0.16, Color32::from_rgb(16, 104, 112));
        }
        ForeignPowerId::NordicConfederation => {
            // Valheim Frost League: Arctic Cyan & Violet Aurora Banner + Silver Ice Star
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w, h * 0.52)), CornerRadius::same(1), Color32::from_rgb(38, 146, 196));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), c.y), Vec2::new(w, h * 0.50)), CornerRadius::same(1), Color32::from_rgb(76, 46, 138));
            draw_compass_star(painter, c, h * 0.30, silver);
        }
        ForeignPowerId::PacificSyndicate => {
            // Thalassia Maritime: Deep Sea-Teal & Coral Wave + Golden Trident Diamond
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(12, 82, 114));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), rect.bottom() - h * 0.28), Vec2::new(w, h * 0.28)), CornerRadius::same(1), Color32::from_rgb(218, 96, 82));
            draw_diamond(painter, Pos2::new(c.x, c.y - h * 0.08), w * 0.18, h * 0.26, gold);
        }
        ForeignPowerId::AndeanFederation => {
            // Solterra Cordillera: Terracotta & Forest Emerald Diagonal + Gold Hex
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(176, 76, 44));
            let lower = vec![rect.left_bottom(), rect.right_bottom(), rect.right_top()];
            painter.add(egui::Shape::convex_polygon(lower, Color32::from_rgb(24, 118, 76), Stroke::NONE));
            draw_hex(painter, c, h * 0.26, gold);
        }
        ForeignPowerId::SaharaAlliance => {
            // Zahara Equatorial: Deep Plum-Violet & Copper Triband + Ivory Shield
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(96, 36, 118));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), c.y - h * 0.16), Vec2::new(w, h * 0.32)), CornerRadius::ZERO, Color32::from_rgb(198, 108, 42));
            draw_diamond(painter, c, w * 0.16, h * 0.30, silver);
        }
        ForeignPowerId::AlpineRepublic => {
            // Montclair Summit: Slate Silver & Royal Indigo + Golden Crystal Peak
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(56, 72, 104));
            let peak = vec![Pos2::new(c.x, rect.top() + h * 0.14), Pos2::new(rect.right() - w * 0.18, rect.bottom() - h * 0.14), Pos2::new(rect.left() + w * 0.18, rect.bottom() - h * 0.14)];
            painter.add(egui::Shape::convex_polygon(peak, silver, Stroke::NONE));
            draw_diamond(painter, Pos2::new(c.x, c.y + h * 0.08), w * 0.09, h * 0.16, gold);
        }
        ForeignPowerId::VesperaEmpire => {
            // Grand Duchy of Vespera: Midnight Velvet & Rose-Gold Crest
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(48, 22, 68));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left() + w * 0.38, rect.top()), Vec2::new(w * 0.24, h)), CornerRadius::ZERO, Color32::from_rgb(196, 134, 86));
            draw_compass_star(painter, c, h * 0.26, silver);
        }
        ForeignPowerId::KharosDesertLeague => {
            // Kharos Oasis: Warm Sand-Gold & Oasis Cyan Geometric Band
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(206, 158, 62));
            let chev = vec![rect.right_top(), Pos2::new(c.x, c.y), rect.right_bottom()];
            painter.add(egui::Shape::convex_polygon(chev, Color32::from_rgb(18, 132, 148), Stroke::NONE));
            draw_hex(painter, Pos2::new(rect.left() + w * 0.32, c.y), h * 0.22, Color32::from_rgb(38, 44, 62));
        }
        ForeignPowerId::CaledonArchipelago => {
            // Caledon: Deep Marine & Jade Quartered Banner
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w * 0.5, h * 0.5)), CornerRadius::same(1), Color32::from_rgb(18, 92, 138));
            painter.rect_filled(Rect::from_min_size(Pos2::new(c.x, rect.top()), Vec2::new(w * 0.5, h * 0.5)), CornerRadius::same(1), Color32::from_rgb(34, 158, 126));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), c.y), Vec2::new(w * 0.5, h * 0.5)), CornerRadius::same(1), Color32::from_rgb(34, 158, 126));
            painter.rect_filled(Rect::from_min_size(Pos2::new(c.x, c.y), Vec2::new(w * 0.5, h * 0.5)), CornerRadius::same(1), Color32::from_rgb(18, 92, 138));
            draw_diamond(painter, c, w * 0.14, h * 0.22, gold);
        }
        ForeignPowerId::DrakoriaState => {
            // Iron Republic of Drakoria: Obsidian & Crimson Iron Chevron + Amber Hex
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(26, 28, 36));
            let chev = vec![rect.left_top(), Pos2::new(rect.left() + w * 0.48, c.y), rect.left_bottom()];
            painter.add(egui::Shape::convex_polygon(chev, Color32::from_rgb(172, 24, 38), Stroke::NONE));
            draw_hex(painter, Pos2::new(c.x + w * 0.16, c.y), h * 0.23, Color32::from_rgb(245, 158, 11));
        }
        ForeignPowerId::LuminaRepublic => {
            // Commonwealth of Lumina: Bright Mint-Teal & Royal Indigo + Radiant Star
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(26, 168, 152));
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w * 0.34, h)), CornerRadius::same(1), Color32::from_rgb(42, 48, 118));
            draw_compass_star(painter, Pos2::new(rect.left() + w * 0.64, c.y), h * 0.28, silver);
        }
        ForeignPowerId::OakhavenUnion => {
            // Oakhaven Heartland: Deep Forest & Harvest Gold + Copper Diamond
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w, h * 0.58)), CornerRadius::same(1), Color32::from_rgb(32, 108, 62));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), rect.top() + h * 0.58), Vec2::new(w, h * 0.42)), CornerRadius::same(1), Color32::from_rgb(212, 162, 42));
            draw_diamond(painter, Pos2::new(c.x, rect.top() + h * 0.42), w * 0.16, h * 0.26, silver);
        }
        ForeignPowerId::Emergent(n) => {
            let palettes: [(Color32, Color32, Color32); 6] = [
                (Color32::from_rgb(22, 130, 120), Color32::from_rgb(236, 242, 248), gold),
                (Color32::from_rgb(124, 52, 28), gold, Color32::from_rgb(26, 30, 42)),
                (Color32::from_rgb(67, 56, 202), silver, Color32::from_rgb(16, 185, 129)),
                (Color32::from_rgb(156, 32, 54), Color32::from_rgb(24, 28, 36), Color32::from_rgb(245, 158, 11)),
                (Color32::from_rgb(15, 118, 110), Color32::from_rgb(244, 206, 72), Color32::from_rgb(30, 58, 138)),
                (Color32::from_rgb(109, 40, 217), silver, Color32::from_rgb(239, 68, 68)),
            ];
            let (c1, c2, c3) = if power.is_rebel_state {
                (Color32::from_rgb(153, 27, 27), Color32::from_rgb(18, 20, 26), Color32::from_rgb(245, 158, 11))
            } else {
                palettes[((power.flag_seed as usize) + (n as usize)) % palettes.len()]
            };
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w, h * 0.5)), CornerRadius::same(1), c1);
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), c.y), Vec2::new(w, h * 0.5)), CornerRadius::same(1), c2);
            let chevron = vec![rect.left_top(), Pos2::new(rect.left() + w * 0.40, c.y), rect.left_bottom()];
            painter.add(egui::Shape::convex_polygon(chevron, c3, Stroke::NONE));
            draw_compass_star(painter, Pos2::new(c.x + w * 0.14, c.y), h * 0.22, if power.is_rebel_state { Color32::from_rgb(239, 68, 68) } else { c3 });
        }
    }

    if power.annexed {
        painter.rect_filled(
            rect,
            CornerRadius::same(2),
            Color32::from_rgba_unmultiplied(10, 18, 28, 125),
        );
        painter.line_segment(
            [rect.left_bottom(), rect.right_top()],
            Stroke::new(2.5_f32, gold),
        );
        draw_compass_star(painter, c, h * 0.25, gold);
    } else if power.puppet_state {
        let badge_c = Pos2::new(rect.right() - w * 0.16, rect.top() + h * 0.24);
        painter.circle_filled(badge_c, h * 0.15, accent_purple());
        painter.circle_stroke(badge_c, h * 0.15, Stroke::new(1.0_f32, gold));
    }

    paint_flag_finish(painter, rect, power.annexed || power.puppet_state);
}

pub fn draw_world_theater_map(
    ui: &mut egui::Ui,
    state: &NationState,
    metrics: &UiMetrics,
) -> Option<ForeignPowerId> {
    let mut clicked_power: Option<ForeignPowerId> = None;

    hero_card_frame(if state.flags.ledger_rebalance || state.is_global_hegemon() {
        border_gold()
    } else {
        border_subtle()
    })
    .show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new("🌍 STRATEGIC WORLD THEATER CHART")
                    .color(accent_gold())
                    .size(metrics.fs(13.5))
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                badge_scaled(
                    ui,
                    &format!(
                        "EMPIRE: {} ANNEXED • {} CLIENT • {} NATIONS",
                        state.annexed_count(),
                        state.puppet_count(),
                        state.foreign_powers.len()
                    ),
                    if state.annexed_count() > 0 { accent_gold() } else { accent_blue() },
                    metrics.fs(10.2),
                );
                if state.active_wars_count() > 0 {
                    badge_scaled(
                        ui,
                        &format!("⚔ {} ACTIVE WAR(S)", state.active_wars_count()),
                        accent_red(),
                        metrics.fs(10.2),
                    );
                }
            });
        });
        ui.add_space(6.0);

        let avail_w = ui.available_width().max(360.0);
        let total_nodes = state.foreign_powers.len().max(1);
        let map_h = (avail_w * if total_nodes > 12 { 0.42 } else { 0.34 }).clamp(290.0, 460.0);
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(avail_w, map_h), egui::Sense::click());
        let p = ui.painter();

        // Map background adapts to Dark (Tactical Radar) vs Light (Parchment Nautical Chart)
        let map_bg = if is_dark_mode() {
            Color32::from_rgb(9, 15, 25)
        } else {
            Color32::from_rgb(228, 222, 206)
        };
        let grid_col = if is_dark_mode() {
            Color32::from_rgba_unmultiplied(56, 189, 248, 20)
        } else {
            Color32::from_rgba_unmultiplied(120, 102, 74, 34)
        };
        p.rect_filled(rect, CornerRadius::same(8), map_bg);

        let cols = 12;
        let rows = 6;
        for i in 1..cols {
            let x = rect.left() + rect.width() * (i as f32 / cols as f32);
            p.line_segment(
                [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                Stroke::new(0.7_f32, grid_col),
            );
        }
        for j in 1..rows {
            let y = rect.top() + rect.height() * (j as f32 / rows as f32);
            p.line_segment(
                [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                Stroke::new(0.7_f32, grid_col),
            );
        }

        let center = Pos2::new(rect.left() + rect.width() * 0.50, rect.top() + rect.height() * 0.50);
        let base_r = (map_h * 0.145).clamp(28.0, 52.0);
        p.circle_stroke(center, base_r * 1.9, Stroke::new(1.0_f32, grid_col));
        p.circle_stroke(center, base_r * 3.1, Stroke::new(1.0_f32, grid_col));

        // Dual-Ring Layout: Inner Ring (first 6) + Outer Ring (remaining 12+) so 18+ nations never overlap!
        let inner_count = total_nodes.min(6);
        let outer_count = total_nodes.saturating_sub(inner_count);
        let inner_rx = (rect.width() * 0.255).max(95.0);
        let inner_ry = (rect.height() * 0.255).max(62.0);
        let outer_rx = (rect.width() * 0.435).max(145.0);
        let outer_ry = (rect.height() * 0.425).max(104.0);

        let node_pos_idx = |idx: usize| -> Pos2 {
            if idx < inner_count {
                let angle = -std::f32::consts::FRAC_PI_2
                    + (idx as f32 / inner_count.max(1) as f32) * std::f32::consts::TAU;
                Pos2::new(center.x + inner_rx * angle.cos(), center.y + inner_ry * angle.sin())
            } else {
                let oidx = idx - inner_count;
                let stagger = std::f32::consts::PI / outer_count.max(1) as f32;
                let angle = -std::f32::consts::FRAC_PI_2
                    + stagger
                    + (oidx as f32 / outer_count.max(1) as f32) * std::f32::consts::TAU;
                Pos2::new(center.x + outer_rx * angle.cos(), center.y + outer_ry * angle.sin())
            }
        };

        let flag_h = (map_h * 0.056).clamp(14.0, 22.0);
        let flag_w = flag_h * 1.55;

        // Draw connecting lanes
        for (idx, fp) in state.foreign_powers.iter().enumerate() {
            let target_pos = node_pos_idx(idx);
            let (line_col, width): (Color32, f32) = if fp.annexed {
                (Color32::from_rgba_unmultiplied(234, 179, 8, 175), 2.0)
            } else if fp.at_war {
                (accent_red(), 2.5)
            } else if fp.puppet_state {
                (accent_purple(), 1.9)
            } else if fp.defense_alliance {
                (accent_green(), 1.7)
            } else if fp.trade_pact {
                (accent_blue(), 1.4)
            } else {
                (
                    if is_dark_mode() {
                        Color32::from_rgba_unmultiplied(120, 145, 175, 42)
                    } else {
                        Color32::from_rgba_unmultiplied(110, 96, 76, 52)
                    },
                    0.85,
                )
            };
            p.line_segment([center, target_pos], Stroke::new(width, line_col));

            if fp.at_war && !fp.annexed {
                let mid = Pos2::new((center.x + target_pos.x) * 0.5, (center.y + target_pos.y) * 0.5);
                p.circle_filled(mid, 9.5, Color32::from_rgb(138, 24, 28));
                p.circle_stroke(mid, 9.5, Stroke::new(1.2_f32, accent_red()));
                p.text(
                    mid,
                    egui::Align2::CENTER_CENTER,
                    format!("{:+.0}", fp.war_score),
                    FontId::proportional(8.8),
                    Color32::WHITE,
                );
            }
        }

        // Player Sovereign Capital Hub at center
        let hub_r = (base_r * 0.92).min(map_h * 0.18);
        let hub_stroke = if state.flags.ledger_rebalance || state.is_global_hegemon() {
            accent_gold()
        } else {
            accent_blue()
        };
        let hub_fill = Color32::from_rgba_unmultiplied(
            hub_stroke.r(),
            hub_stroke.g(),
            hub_stroke.b(),
            if is_dark_mode() { 34 } else { 42 },
        );
        p.circle_filled(center, hub_r, hub_fill);
        p.circle_stroke(center, hub_r, Stroke::new(1.8_f32, hub_stroke));

        let our_flag_rect = Rect::from_center_size(
            Pos2::new(center.x, center.y - 6.0),
            Vec2::new(flag_w * 1.4, flag_h * 1.4),
        );
        paint_nation_flag_at(
            p,
            our_flag_rect,
            state.regime,
            state.economy,
            state.religion_policy,
            state.flags.ledger_rebalance,
            state.is_global_hegemon(),
        );
        p.text(
            Pos2::new(center.x, our_flag_rect.bottom() + 8.5),
            egui::Align2::CENTER_CENTER,
            &state.nation_name,
            FontId::proportional((11.0 * metrics.scale).clamp(9.5, 13.5)),
            if state.flags.ledger_rebalance { accent_gold() } else { text_primary() },
        );

        let hover_pos = ui.input(|i| i.pointer.hover_pos());
        let click_pos = if response.clicked() {
            response.interact_pointer_pos()
        } else {
            None
        };
        let mut tooltip_text: Option<String> = None;

        for (idx, fp) in state.foreign_powers.iter().enumerate() {
            let pos = node_pos_idx(idx);
            let card_w = (flag_w + 62.0 * metrics.scale).clamp(88.0, 146.0);
            let card_h = (flag_h + 13.0).clamp(28.0, 39.0);
            let node_rect = Rect::from_center_size(pos, Vec2::new(card_w, card_h));
            let hovered = hover_pos.map(|hp| node_rect.contains(hp)).unwrap_or(false);

            let border_col = if fp.annexed {
                accent_gold()
            } else if fp.at_war || fp.is_rebel_state {
                accent_red()
            } else if fp.puppet_state {
                accent_purple()
            } else if hovered {
                accent_cyan()
            } else {
                border_subtle()
            };
            let node_bg = if hovered {
                if is_dark_mode() {
                    Color32::from_rgb(32, 46, 68)
                } else {
                    Color32::from_rgb(255, 252, 244)
                }
            } else if is_dark_mode() {
                Color32::from_rgba_unmultiplied(18, 26, 40, 240)
            } else {
                Color32::from_rgba_unmultiplied(250, 247, 238, 242)
            };
            p.rect_filled(node_rect, CornerRadius::same(6), node_bg);
            p.rect_stroke(
                node_rect,
                CornerRadius::same(6),
                Stroke::new(if hovered || fp.at_war || fp.annexed { 1.5_f32 } else { 1.0_f32 }, border_col),
                egui::StrokeKind::Inside,
            );

            let f_rect = Rect::from_min_size(
                Pos2::new(node_rect.left() + 4.5, node_rect.center().y - flag_h * 0.5),
                Vec2::new(flag_w, flag_h),
            );
            paint_foreign_flag_at(p, f_rect, fp);

            let text_x = f_rect.right() + 4.5;
            p.text(
                Pos2::new(text_x, node_rect.center().y - 5.0),
                egui::Align2::LEFT_CENTER,
                &fp.short_name,
                FontId::proportional((9.2 * metrics.scale).clamp(8.2, 11.4)),
                text_primary(),
            );
            let (sub_txt, sub_col) = if fp.annexed {
                ("★ ANNEXED".to_string(), accent_gold())
            } else if fp.is_rebel_state && fp.at_war {
                (format!("🔥 {:+.0}", fp.war_score), accent_red())
            } else if fp.at_war {
                (format!("⚔ {:+.0}", fp.war_score), accent_red())
            } else if fp.puppet_state {
                ("👑 CLIENT".to_string(), accent_purple())
            } else if fp.defense_alliance {
                (format!("🛡 {:+.0}", fp.relation), accent_green())
            } else {
                (format!("{:+.0}", fp.relation), severity_color(fp.relation + 50.0, true))
            };
            p.text(
                Pos2::new(text_x, node_rect.center().y + 6.0),
                egui::Align2::LEFT_CENTER,
                sub_txt,
                FontId::proportional((8.4 * metrics.scale).clamp(7.8, 10.5)),
                sub_col,
            );

            if hovered {
                tooltip_text = Some(format!(
                    "{} ({})\nStatus: {}\nMilitary: {:.0} | Economy: {:.0} | Relations: {:+.0}\nClick to open Military & Geopolitics",
                    fp.name,
                    fp.ideology,
                    if fp.annexed {
                        "Annexed Province"
                    } else if fp.at_war {
                        "AT WAR"
                    } else if fp.puppet_state {
                        "Loyal Client State"
                    } else if fp.defense_alliance {
                        "Defense Ally"
                    } else {
                        "Independent Power"
                    },
                    fp.military_power,
                    fp.economic_power,
                    fp.relation
                ));
            }
            if let Some(cp) = click_pos {
                if node_rect.contains(cp) {
                    clicked_power = Some(fp.id);
                }
            }
        }

        p.rect_stroke(
            rect,
            CornerRadius::same(8),
            Stroke::new(1.0_f32, border_subtle()),
            egui::StrokeKind::Inside,
        );

        if let Some(tip) = tooltip_text {
            response.on_hover_text(tip);
        }
    });

    clicked_power
}

pub fn compute_era_label(state: &NationState) -> &'static str {
    if state.is_global_hegemon() || state.flags.ledger_rebalance {
        "GOLDEN HEGEMONY ERA"
    } else if state.programs.space_tier >= 2 || state.programs.fighter_jet_tier >= 3 {
        "ORBITAL HYPER-AGE"
    } else if state.nuclear_status.stage() >= 3 {
        "ATOMIC DETERRENCE ERA"
    } else if state.gdp_bn >= 1000.0 {
        "SUPERPOWER ERA"
    } else {
        "SOVEREIGN DAWN ERA"
    }
}

pub fn compute_sovereign_power_score(state: &NationState) -> (f64, &'static str, Color32) {
    let score = (state.gdp_bn / 18.0).min(1200.0)
        + state.active_troops_k * 0.25
        + state.military_tech * 1.4
        + state.tech_level * 1.2
        + state.stability * 1.1
        + (state.nuclear_status.stage() as f64) * 55.0
        + (state.armed_forces.army_divisions as f64) * 2.0
        + (state.armed_forces.fighter_squadrons as f64) * 3.0
        + (state.armed_forces.destroyers as f64) * 2.5
        + (state.annexed_count() as f64) * 180.0
        + (state.puppet_count() as f64) * 110.0;

    if state.is_global_hegemon() {
        (score, "GLOBAL HEGEMON", accent_gold())
    } else if score >= 1400.0 {
        (score, "HYPERPOWER", accent_gold())
    } else if score >= 900.0 {
        (score, "SUPERPOWER", accent_purple())
    } else if score >= 520.0 {
        (score, "GREAT POWER", accent_blue())
    } else if score >= 300.0 {
        (score, "REGIONAL POWER", accent_green())
    } else {
        (score, "EMERGING STATE", accent_amber())
    }
}

pub fn compute_achievements(
    state: &NationState,
) -> Vec<(&'static str, &'static str, &'static str, bool, Color32)> {
    vec![
        ("🏆", "Trillionaire", "Reach $1.00T Nominal GDP", state.gdp_bn >= 1000.0, accent_green()),
        ("💎", "Debt-Free", "Eliminate all sovereign national debt", state.national_debt_bn <= 0.1, accent_cyan()),
        ("☢", "Atomic Triad", "Complete Strategic ICBM Triad", state.nuclear_status == NuclearStatus::StrategicTriad, accent_amber()),
        ("⚔", "Conqueror", "Annex at least 1 rival country after war", state.annexed_count() >= 1, accent_red()),
        ("👑", "Global Hegemon", "Annex or vassalize all rival world powers", state.is_global_hegemon(), accent_gold()),
        ("✈", "Sky Sovereign", "Develop a 5th-Gen+ Indigenous Stealth Fighter Jet", state.programs.fighter_jet_tier >= 2, accent_cyan()),
        ("💥", "Atomic Tester", "Conduct a live nuclear weapons test", state.programs.nuke_tests_conducted >= 1, accent_red()),
        ("💰", "Arms Titan", "Sell $50B+ in defense weapons exports", state.armed_forces.total_arms_sales_bn >= 50.0, accent_green()),
        ("🔥", "Kingmaker", "Split a rival nation to form a brand-new country", state.foreign_powers.iter().any(|p| matches!(p.id, ForeignPowerId::Emergent(_)) && !p.is_rebel_state), accent_amber()),
        ("⭐", "Golden Era", "Unlock the Sovereign Golden Mandate", state.flags.ledger_rebalance, accent_gold()),
    ]
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
            ui.label(RichText::new(title).color(text_muted()).size(11.5).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let val_str = if unit_suffix == "B" {
                    format!("{} ({})", fmt_billions(latest), fmt_signed_billions(delta))
                } else {
                    format!("{:.1}{} ({:+.1})", latest, unit_suffix, delta)
                };
                ui.label(RichText::new(val_str).color(color).size(12.0).strong());
            });
        });
        ui.add_space(6.0);

        let width = ui.available_width().max(130.0);
        let height = (width * 0.24).clamp(64.0, 96.0);
        let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
        let painter = ui.painter();

        painter.rect_filled(rect, CornerRadius::same(5), bg_inset());
        let grid_line = if is_dark_mode() {
            Color32::from_rgb(28, 38, 56)
        } else {
            Color32::from_rgb(206, 200, 186)
        };
        for i in 1..4 {
            let y = rect.top() + rect.height() * (i as f32 / 4.0);
            painter.line_segment(
                [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                Stroke::new(0.6_f32, grid_line),
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
            painter.circle_filled(pts[0], 3.5, color);
        } else {
            let fill_col = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 30);
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
            painter.add(egui::Shape::line(pts.clone(), Stroke::new(2.2_f32, color)));
            if let Some(&last) = pts.last() {
                painter.circle_filled(last, 3.5, color);
            }
        }
    });
}
