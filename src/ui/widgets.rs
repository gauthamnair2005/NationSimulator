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
        ForeignPowerId::WorldState(n) => {
            // 100% Original Fictional Procedural Heraldry (8 geometric patterns x 12 palettes, zero real-world flags)
            let palettes: [(Color32, Color32, Color32); 12] = [
                (Color32::from_rgb(18, 92, 134), Color32::from_rgb(208, 134, 42), silver),
                (Color32::from_rgb(76, 36, 128), Color32::from_rgb(26, 148, 142), gold),
                (Color32::from_rgb(32, 62, 92), Color32::from_rgb(184, 72, 66), gold),
                (Color32::from_rgb(22, 112, 88), Color32::from_rgb(44, 48, 72), Color32::from_rgb(242, 198, 76)),
                (Color32::from_rgb(128, 42, 74), Color32::from_rgb(28, 38, 62), silver),
                (Color32::from_rgb(156, 86, 32), Color32::from_rgb(24, 86, 108), Color32::from_rgb(244, 234, 206)),
                (Color32::from_rgb(38, 46, 84), Color32::from_rgb(188, 144, 52), Color32::from_rgb(64, 196, 186)),
                (Color32::from_rgb(92, 46, 36), Color32::from_rgb(36, 116, 112), gold),
                (Color32::from_rgb(24, 82, 116), Color32::from_rgb(142, 58, 122), silver),
                (Color32::from_rgb(46, 88, 62), Color32::from_rgb(198, 116, 54), silver),
                (Color32::from_rgb(56, 32, 86), Color32::from_rgb(212, 152, 48), Color32::from_rgb(232, 240, 248)),
                (Color32::from_rgb(22, 72, 96), Color32::from_rgb(196, 84, 74), Color32::from_rgb(245, 212, 98)),
            ];
            let seed = (power.flag_seed as usize).wrapping_add((n as usize) * 13);
            let (c1, c2, c3) = palettes[seed % palettes.len()];
            let pattern = (seed / 12) % 8;
            match pattern {
                0 => {
                    // Diagonal split + centered hex crest
                    painter.rect_filled(rect, CornerRadius::same(2), c1);
                    let tri = vec![rect.left_bottom(), rect.right_bottom(), rect.right_top()];
                    painter.add(egui::Shape::convex_polygon(tri, c2, Stroke::NONE));
                    draw_hex(painter, c, h * 0.25, c3);
                }
                1 => {
                    // Hoist chevron + diamond emblem
                    painter.rect_filled(rect, CornerRadius::same(2), c1);
                    let chev = vec![rect.left_top(), Pos2::new(rect.left() + w * 0.44, c.y), rect.left_bottom()];
                    painter.add(egui::Shape::convex_polygon(chev, c2, Stroke::NONE));
                    draw_diamond(painter, Pos2::new(c.x + w * 0.14, c.y), w * 0.14, h * 0.26, c3);
                }
                2 => {
                    // Quartered banner + compass star
                    painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w * 0.5, h * 0.5)), CornerRadius::same(1), c1);
                    painter.rect_filled(Rect::from_min_size(Pos2::new(c.x, rect.top()), Vec2::new(w * 0.5, h * 0.5)), CornerRadius::same(1), c2);
                    painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), c.y), Vec2::new(w * 0.5, h * 0.5)), CornerRadius::same(1), c2);
                    painter.rect_filled(Rect::from_min_size(Pos2::new(c.x, c.y), Vec2::new(w * 0.5, h * 0.5)), CornerRadius::same(1), c1);
                    draw_compass_star(painter, c, h * 0.27, c3);
                }
                3 => {
                    // Twin outer bands + diamond ring
                    painter.rect_filled(rect, CornerRadius::same(2), c1);
                    painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w, h * 0.20)), CornerRadius::same(1), c2);
                    painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), rect.bottom() - h * 0.20), Vec2::new(w, h * 0.20)), CornerRadius::same(1), c2);
                    draw_diamond(painter, c, w * 0.20, h * 0.28, c3);
                    painter.circle_filled(c, h * 0.10, c1);
                }
                4 => {
                    // Asymmetric diagonal ribbon + hex
                    painter.rect_filled(rect, CornerRadius::same(2), c1);
                    let band = vec![
                        Pos2::new(rect.left() + w * 0.15, rect.top()),
                        Pos2::new(rect.left() + w * 0.45, rect.top()),
                        Pos2::new(rect.left() + w * 0.25, rect.bottom()),
                        rect.left_bottom(),
                    ];
                    painter.add(egui::Shape::convex_polygon(band, c2, Stroke::NONE));
                    draw_compass_star(painter, Pos2::new(c.x + w * 0.16, c.y), h * 0.24, c3);
                }
                5 => {
                    // Concentric heraldic rings + vertical pillar
                    painter.rect_filled(rect, CornerRadius::same(2), c1);
                    painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left() + w * 0.36, rect.top()), Vec2::new(w * 0.28, h)), CornerRadius::ZERO, c2);
                    painter.circle_stroke(c, h * 0.28, Stroke::new(1.6_f32, c3));
                    draw_diamond(painter, c, h * 0.14, h * 0.14, c3);
                }
                6 => {
                    // Twin opposing triangles + hex core
                    painter.rect_filled(rect, CornerRadius::same(2), c1);
                    let t1 = vec![rect.left_top(), Pos2::new(c.x, c.y), rect.left_bottom()];
                    let t2 = vec![rect.right_top(), Pos2::new(c.x, c.y), rect.right_bottom()];
                    painter.add(egui::Shape::convex_polygon(t1, c2, Stroke::NONE));
                    painter.add(egui::Shape::convex_polygon(t2, c2, Stroke::NONE));
                    draw_hex(painter, c, h * 0.22, c3);
                }
                _ => {
                    // Horizontal horizon split + radiant peak
                    painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w, h * 0.52)), CornerRadius::same(1), c1);
                    painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), c.y), Vec2::new(w, h * 0.50)), CornerRadius::same(1), c2);
                    let peak = vec![
                        Pos2::new(c.x, rect.top() + h * 0.16),
                        Pos2::new(c.x + w * 0.22, rect.bottom() - h * 0.16),
                        Pos2::new(c.x - w * 0.22, rect.bottom() - h * 0.16),
                    ];
                    painter.add(egui::Shape::convex_polygon(peak, c3, Stroke::NONE));
                }
            }
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

#[derive(Debug, Clone)]
pub struct GlobeViewState {
    pub cam_lon: f32,
    pub cam_lat: f32,
    pub auto_spin: bool,
    pub selected: Option<ForeignPowerId>,
}

impl Default for GlobeViewState {
    fn default() -> Self {
        Self {
            cam_lon: 12.0,
            cam_lat: 16.0,
            auto_spin: true,
            selected: None,
        }
    }
}

fn project_ortho(
    lat_deg: f32,
    lon_deg: f32,
    cam_lat_deg: f32,
    cam_lon_deg: f32,
    center: Pos2,
    radius: f32,
) -> (Pos2, f32) {
    let phi = lat_deg.to_radians();
    let lam = (lon_deg - cam_lon_deg).to_radians();
    let phi0 = cam_lat_deg.to_radians();
    let cos_c = phi0.sin() * phi.sin() + phi0.cos() * phi.cos() * lam.cos();
    let x = center.x + radius * phi.cos() * lam.sin();
    let y = center.y - radius * (phi0.cos() * phi.sin() - phi0.sin() * phi.cos() * lam.cos());
    (Pos2::new(x, y), cos_c)
}

/// Interactive 3D Rotating Planetary Globe plotting all 178+ Fictional Nations, Continents & War/Empire Arcs!
pub fn draw_interactive_globe(
    ui: &mut egui::Ui,
    state: &NationState,
    globe: &mut GlobeViewState,
    metrics: &UiMetrics,
) -> Option<ForeignPowerId> {
    let mut double_clicked_open: Option<ForeignPowerId> = None;

    hero_card_frame(if state.flags.ledger_rebalance || state.is_global_hegemon() {
        border_gold()
    } else {
        border_subtle()
    })
    .show(ui, |ui| {
        // Header + Continent Camera Snap Buttons
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(format!(
                    "🌍 3D PLANETARY COMMAND GLOBE ({} SOVEREIGN NATIONS)",
                    state.foreign_powers.len() + 1
                ))
                .color(accent_gold())
                .size(metrics.fs(13.5))
                .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let spin_lbl = if globe.auto_spin { "⏸ Spin ON" } else { "▶ Spin OFF" };
                if ui.small_button(spin_lbl).clicked() {
                    globe.auto_spin = !globe.auto_spin;
                }
                for &(reg, tlat, tlon) in &[
                    ("Thalassia", -24.0_f32, 142.0_f32),
                    ("Solterra", -20.0, -64.0),
                    ("Indravara", 28.0, 98.0),
                    ("Zahara", 2.0, 20.0),
                    ("Aethelis", 44.0, 10.0),
                    ("Valdoria", 46.0, -96.0),
                ] {
                    if ui.small_button(reg).clicked() {
                        globe.cam_lat = tlat;
                        globe.cam_lon = tlon;
                        globe.auto_spin = false;
                    }
                }
            });
        });
        ui.add_space(6.0);

        let avail_w = ui.available_width().max(340.0);
        let globe_h = (avail_w * 0.42).clamp(320.0, 490.0);
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(avail_w, globe_h), egui::Sense::click_and_drag());

        // Drag directly on the globe to spin in 3D!
        if response.dragged() {
            let d = response.drag_delta();
            globe.cam_lon -= d.x * 0.42;
            globe.cam_lat = (globe.cam_lat + d.y * 0.32).clamp(-60.0, 60.0);
            globe.auto_spin = false;
        }
        if globe.cam_lon > 180.0 {
            globe.cam_lon -= 360.0;
        } else if globe.cam_lon < -180.0 {
            globe.cam_lon += 360.0;
        }

        let p = ui.painter_at(rect);

        // Deep Space / Parchment Chart Backdrop
        let space_bg = if is_dark_mode() {
            Color32::from_rgb(6, 11, 21)
        } else {
            Color32::from_rgb(228, 222, 206)
        };
        p.rect_filled(rect, CornerRadius::same(10), space_bg);

        // Subtle starfield in Dark Mode
        if is_dark_mode() {
            for i in 0..42 {
                let sx = rect.left() + ((i * 97 + 19) % 98 + 1) as f32 / 100.0 * rect.width();
                let sy = rect.top() + ((i * 53 + 11) % 96 + 2) as f32 / 100.0 * rect.height();
                p.circle_filled(Pos2::new(sx, sy), 1.1, Color32::from_rgba_unmultiplied(210, 230, 255, 80));
            }
        }

        let center = rect.center();
        let radius = (globe_h * 0.44).min(avail_w * 0.43).max(110.0);

        // Outer Planetary Atmosphere Glow
        let atmo_col = if state.flags.ledger_rebalance {
            Color32::from_rgb(250, 204, 21)
        } else if is_dark_mode() {
            Color32::from_rgb(56, 189, 248)
        } else {
            Color32::from_rgb(62, 132, 168)
        };
        for ring in 1..=3 {
            p.circle_filled(
                center,
                radius + ring as f32 * 5.5,
                Color32::from_rgba_unmultiplied(atmo_col.r(), atmo_col.g(), atmo_col.b(), (24 / ring) as u8),
            );
        }

        // 3D Shaded Ocean Sphere
        let ocean_outer = if is_dark_mode() {
            Color32::from_rgb(10, 26, 54)
        } else {
            Color32::from_rgb(142, 186, 206)
        };
        let ocean_inner = if is_dark_mode() {
            Color32::from_rgb(18, 52, 98)
        } else {
            Color32::from_rgb(176, 214, 226)
        };
        p.circle_filled(center, radius, ocean_outer);
        p.circle_filled(
            Pos2::new(center.x - radius * 0.14, center.y - radius * 0.14),
            radius * 0.78,
            ocean_inner,
        );

        // 3D Latitude & Longitude Graticule Lines
        let grat_col = if is_dark_mode() {
            Color32::from_rgba_unmultiplied(90, 175, 235, 34)
        } else {
            Color32::from_rgba_unmultiplied(54, 88, 114, 46)
        };
        for lat_step in [-60.0_f32, -30.0, 0.0, 30.0, 60.0] {
            let mut seg = Vec::new();
            for step in 0..=72 {
                let lon = -180.0 + step as f32 * 5.0;
                let (pt, z) = project_ortho(lat_step, lon, globe.cam_lat, globe.cam_lon, center, radius);
                if z > 0.0 {
                    seg.push(pt);
                } else if seg.len() >= 2 {
                    p.add(egui::Shape::line(seg.clone(), Stroke::new(0.8_f32, grat_col)));
                    seg.clear();
                } else {
                    seg.clear();
                }
            }
            if seg.len() >= 2 {
                p.add(egui::Shape::line(seg, Stroke::new(0.8_f32, grat_col)));
            }
        }
        for lon_i in -6..6 {
            let lon_val = lon_i as f32 * 30.0;
            let mut seg = Vec::new();
            for step in 0..=36 {
                let lat = -90.0 + step as f32 * 5.0;
                let (pt, z) = project_ortho(lat, lon_val, globe.cam_lat, globe.cam_lon, center, radius);
                if z > 0.0 {
                    seg.push(pt);
                } else if seg.len() >= 2 {
                    p.add(egui::Shape::line(seg.clone(), Stroke::new(0.8_f32, grat_col)));
                    seg.clear();
                } else {
                    seg.clear();
                }
            }
            if seg.len() >= 2 {
                p.add(egui::Shape::line(seg, Stroke::new(0.8_f32, grat_col)));
            }
        }

        // 6 Original Fictional Continents projected onto the 3D Sphere!
        let land_fill = if is_dark_mode() {
            Color32::from_rgba_unmultiplied(30, 88, 68, 178)
        } else {
            Color32::from_rgba_unmultiplied(154, 186, 134, 210)
        };
        let land_border = if is_dark_mode() {
            Color32::from_rgba_unmultiplied(74, 186, 146, 120)
        } else {
            Color32::from_rgba_unmultiplied(84, 118, 72, 160)
        };
        for &(cname, poly_coords) in crate::world_gen::fictional_continents() {
            let mut projected: Vec<Pos2> = Vec::new();
            let mut sum_x = 0.0_f32;
            let mut sum_y = 0.0_f32;
            let mut vis_count = 0usize;
            for &(plat, plon) in poly_coords {
                let (pt, z) = project_ortho(plat, plon, globe.cam_lat, globe.cam_lon, center, radius);
                if z > -0.05 {
                    // Clamp slightly behind limb onto sphere rim for smooth continent silhouette
                    let v = pt - center;
                    let len = v.length();
                    let clamped = if len > radius {
                        center + v * (radius / len)
                    } else {
                        pt
                    };
                    projected.push(clamped);
                    if z > 0.08 {
                        sum_x += clamped.x;
                        sum_y += clamped.y;
                        vis_count += 1;
                    }
                }
            }
            if projected.len() >= 3 {
                p.add(egui::Shape::convex_polygon(
                    projected,
                    land_fill,
                    Stroke::new(1.1_f32, land_border),
                ));
                if vis_count >= 3 {
                    let lbl_pos = Pos2::new(sum_x / vis_count as f32, sum_y / vis_count as f32);
                    p.text(
                        lbl_pos,
                        egui::Align2::CENTER_CENTER,
                        cname.to_uppercase(),
                        FontId::proportional((10.0 * metrics.scale).clamp(8.5, 12.5)),
                        if is_dark_mode() {
                            Color32::from_rgba_unmultiplied(192, 236, 214, 115)
                        } else {
                            Color32::from_rgba_unmultiplied(46, 68, 44, 140)
                        },
                    );
                }
            }
        }

        // Player Sovereign Capital on the Globe (lat=20, lon=0)
        let (cap_pos, cap_z) = project_ortho(20.0, 0.0, globe.cam_lat, globe.cam_lon, center, radius);

        // Curved 3D Strategic Arcs from Sovereign Capital to Wars, Annexed, Clients & Allies
        for fp in &state.foreign_powers {
            if !(fp.at_war || fp.annexed || fp.puppet_state || fp.defense_alliance) {
                continue;
            }
            let (tpos, tz) = project_ortho(fp.lat, fp.lon, globe.cam_lat, globe.cam_lon, center, radius);
            if cap_z <= -0.15 && tz <= -0.15 {
                continue;
            }
            let arc_col = if fp.at_war {
                accent_red()
            } else if fp.annexed {
                Color32::from_rgba_unmultiplied(250, 204, 21, 185)
            } else if fp.puppet_state {
                accent_purple()
            } else {
                accent_green()
            };
            let steps = 12;
            let mut pts = Vec::with_capacity(steps + 1);
            for s in 0..=steps {
                let f = s as f32 / steps as f32;
                let ilat = 20.0 + (fp.lat - 20.0) * f;
                let mut dlon = fp.lon - 0.0;
                if dlon > 180.0 { dlon -= 360.0; }
                if dlon < -180.0 { dlon += 360.0; }
                let ilon = dlon * f;
                let lift = 1.0 + (f * std::f32::consts::PI).sin() * 0.09;
                let (pt, z) = project_ortho(ilat, ilon, globe.cam_lat, globe.cam_lon, center, radius * lift);
                if z > 0.0 {
                    pts.push(pt);
                } else if pts.len() >= 2 {
                    p.add(egui::Shape::line(pts.clone(), Stroke::new(1.5_f32, arc_col)));
                    pts.clear();
                } else {
                    pts.clear();
                }
            }
            if pts.len() >= 2 {
                p.add(egui::Shape::line(pts, Stroke::new(1.5_f32, arc_col)));
            }
            let _ = tpos;
        }

        // Plot ALL 178+ Countries on the 3D Sphere!
        let hover_pos = ui.input(|i| i.pointer.hover_pos());
        let click_pos = if response.clicked() { response.interact_pointer_pos() } else { None };
        let dbl_pos = if response.double_clicked() { response.interact_pointer_pos() } else { None };

        let mut hovered_fp: Option<(&ForeignPower, Pos2)> = None;
        let mut best_hover_dist = 18.0_f32;
        let mut best_click_dist = 18.0_f32;
        let mut clicked_id: Option<ForeignPowerId> = None;

        // Pass 1: Draw all minor WorldState dots first, then Major/Active/Selected on top
        for pass in 0..2 {
            for fp in &state.foreign_powers {
                let is_prominent = fp.is_major_power()
                    || fp.at_war
                    || fp.annexed
                    || fp.puppet_state
                    || fp.is_rebel_state
                    || globe.selected == Some(fp.id);
                if (pass == 0 && is_prominent) || (pass == 1 && !is_prominent) {
                    continue;
                }

                let (pt, z) = project_ortho(fp.lat, fp.lon, globe.cam_lat, globe.cam_lon, center, radius);
                if z <= 0.04 {
                    continue;
                }

                let marker_col = if fp.annexed {
                    accent_gold()
                } else if fp.at_war || fp.is_rebel_state {
                    accent_red()
                } else if fp.puppet_state {
                    accent_purple()
                } else if fp.defense_alliance {
                    accent_green()
                } else if fp.trade_pact {
                    accent_blue()
                } else if fp.is_major_power() {
                    accent_cyan()
                } else {
                    Color32::from_rgb(200, 218, 238)
                };

                let base_r = if is_prominent { 4.8 } else { 2.8 };
                let r_dot = (base_r * (0.62 + 0.48 * z)).max(2.0);
                p.circle_filled(pt, r_dot, marker_col);

                if globe.selected == Some(fp.id) {
                    p.circle_stroke(pt, r_dot + 4.0, Stroke::new(1.8_f32, accent_gold()));
                }

                // Draw compact procedural vector flag + short name for prominent visible powers
                if is_prominent && z > 0.25 {
                    let fh = (11.0 * (0.75 + 0.35 * z) * metrics.scale).clamp(9.0, 15.0);
                    let fw = fh * 1.55;
                    let frect = Rect::from_min_size(Pos2::new(pt.x + 5.0, pt.y - fh * 0.5), Vec2::new(fw, fh));
                    paint_foreign_flag_at(&p, frect, fp);
                    if z > 0.45 || globe.selected == Some(fp.id) || fp.at_war {
                        p.text(
                            Pos2::new(frect.right() + 3.0, pt.y),
                            egui::Align2::LEFT_CENTER,
                            &fp.short_name,
                            FontId::proportional((8.8 * metrics.scale).clamp(8.0, 11.2)),
                            if fp.annexed { accent_gold() } else if is_dark_mode() { Color32::WHITE } else { Color32::from_rgb(16, 24, 38) },
                        );
                    }
                }

                if let Some(hp) = hover_pos {
                    let dist = pt.distance(hp);
                    if dist < best_hover_dist {
                        best_hover_dist = dist;
                        hovered_fp = Some((fp, pt));
                    }
                }
                if let Some(cp) = click_pos {
                    let dist = pt.distance(cp);
                    if dist < best_click_dist {
                        best_click_dist = dist;
                        clicked_id = Some(fp.id);
                    }
                }
                if let Some(dp) = dbl_pos {
                    if pt.distance(dp) < 18.0 {
                        double_clicked_open = Some(fp.id);
                    }
                }
            }
        }

        if let Some(cid) = clicked_id {
            globe.selected = Some(cid);
        }

        // Draw Player's Sovereign Capital Flag & Halo on top!
        if cap_z > 0.04 {
            p.circle_stroke(cap_pos, 10.0, Stroke::new(2.0_f32, accent_gold()));
            let fh = (16.0 * metrics.scale).clamp(13.0, 22.0);
            let fw = fh * 1.55;
            let frect = Rect::from_center_size(Pos2::new(cap_pos.x, cap_pos.y - fh * 0.8), Vec2::new(fw, fh));
            paint_nation_flag_at(
                &p,
                frect,
                state.regime,
                state.economy,
                state.religion_policy,
                state.flags.ledger_rebalance,
                state.is_global_hegemon(),
            );
            p.text(
                Pos2::new(cap_pos.x, cap_pos.y + 13.0),
                egui::Align2::CENTER_CENTER,
                format!("★ {}", state.nation_name),
                FontId::proportional((10.5 * metrics.scale).clamp(9.5, 13.0)),
                accent_gold(),
            );
        }

        // Crisp planetary rim stroke
        p.circle_stroke(center, radius, Stroke::new(1.8_f32, atmo_col));

        // Floating HUD Legend & Drag Hint in corners
        p.text(
            Pos2::new(rect.left() + 10.0, rect.bottom() - 12.0),
            egui::Align2::LEFT_CENTER,
            "🖱 Drag to Rotate 3D Globe  •  Click Any Nation to Inspect & Command  •  Double-Click for Full Diplomacy",
            FontId::proportional((9.5 * metrics.scale).clamp(8.5, 11.5)),
            text_muted(),
        );
        p.text(
            Pos2::new(rect.right() - 10.0, rect.top() + 12.0),
            egui::Align2::RIGHT_CENTER,
            format!("CAM {:.0}°N, {:.0}°E", globe.cam_lat, globe.cam_lon),
            FontId::proportional((9.5 * metrics.scale).clamp(8.5, 11.5)),
            accent_cyan(),
        );

        // Highlight Hovered Country on Globe + Rich Hover Tooltip
        if let Some((fp, pt)) = hovered_fp {
            p.circle_stroke(pt, 9.0, Stroke::new(1.8_f32, accent_gold()));
            let fh = 18.0;
            let fw = fh * 1.55;
            let frect = Rect::from_min_size(Pos2::new(pt.x + 10.0, pt.y - 24.0), Vec2::new(fw, fh));
            paint_foreign_flag_at(&p, frect, fp);
            response.on_hover_text(format!(
                "{} [{}]\n{}\nMilitary: {:.0} | Economy: {:.0} | Relations: {:+.0}\nStatus: {}\n(Click to select for quick actions)",
                fp.name,
                fp.region,
                fp.ideology,
                fp.military_power,
                fp.economic_power,
                fp.relation,
                if fp.annexed {
                    "★ Annexed Province"
                } else if fp.at_war {
                    "⚔ AT WAR"
                } else if fp.puppet_state {
                    "👑 Client State"
                } else if fp.defense_alliance {
                    "🛡 Defense Ally"
                } else if fp.trade_pact {
                    "Trade Partner"
                } else {
                    "Independent Nation"
                }
            ));
        }
    });

    double_clicked_open
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
