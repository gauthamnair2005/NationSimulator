use crate::models::*;
use eframe::egui::{self, Color32, CornerRadius, FontId, Margin, Pos2, Rect, RichText, Stroke, Vec2};

pub const BG_CANVAS: Color32 = Color32::from_rgb(11, 16, 26);
pub const BG_PANEL: Color32 = Color32::from_rgb(18, 25, 38);
pub const BG_CARD: Color32 = Color32::from_rgb(24, 34, 50);
pub const BG_HERO: Color32 = Color32::from_rgb(21, 31, 48);
pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(47, 63, 88);
pub const BORDER_GOLD: Color32 = Color32::from_rgb(180, 140, 58);

pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(238, 244, 252);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(150, 168, 194);

pub const ACCENT_BLUE: Color32 = Color32::from_rgb(56, 189, 248);
pub const ACCENT_CYAN: Color32 = Color32::from_rgb(34, 211, 238);
pub const ACCENT_GREEN: Color32 = Color32::from_rgb(34, 197, 94);
pub const ACCENT_AMBER: Color32 = Color32::from_rgb(245, 158, 11);
pub const ACCENT_GOLD: Color32 = Color32::from_rgb(250, 204, 21);
pub const ACCENT_RED: Color32 = Color32::from_rgb(239, 68, 68);
pub const ACCENT_PURPLE: Color32 = Color32::from_rgb(167, 139, 250);

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
        let sidebar_w = (w * 0.168).clamp(182.0, 272.0);
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
    if bn.abs() >= 1000.0 {
        format!("${:.2}T", bn / 1000.0)
    } else {
        format!("${:.1}B", bn)
    }
}

pub fn fmt_signed_billions(bn: f64) -> String {
    if bn.abs() >= 1000.0 {
        format!("{:+.2}T", bn / 1000.0)
    } else {
        format!("{:+.1}B", bn)
    }
}

pub fn card_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(BG_CARD)
        .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(Margin::same(12))
}

pub fn hero_card_frame(border_col: Color32) -> egui::Frame {
    egui::Frame::new()
        .fill(BG_HERO)
        .stroke(Stroke::new(1.3_f32, border_col))
        .corner_radius(CornerRadius::same(10))
        .inner_margin(Margin::same(12))
}

pub fn panel_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(BG_PANEL)
        .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
        .corner_radius(CornerRadius::same(7))
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
    badge_scaled(ui, text, color, 11.0);
}

pub fn badge_scaled(ui: &mut egui::Ui, text: &str, color: Color32, font_size: f32) {
    let bg = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 36);
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
    let bg = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 26);
    let border = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 160);
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
    let top_tint = Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 18);
    egui::Frame::new()
        .fill(BG_CARD)
        .stroke(Stroke::new(
            1.0_f32,
            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 95),
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
                            .color(TEXT_MUTED)
                            .size((10.8 * scale).clamp(9.5, 13.0))
                            .strong(),
                    );
                });
                ui.add_space(2.0);
                ui.label(
                    RichText::new(value)
                        .color(TEXT_PRIMARY)
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
        ui.label(RichText::new(label).color(TEXT_PRIMARY).size(12.5));
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
    painter.rect_filled(rect, CornerRadius::same(4), Color32::from_rgb(13, 19, 30));
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
        ACCENT_GREEN
    } else if clamped >= -20.0 {
        ACCENT_AMBER
    } else {
        ACCENT_RED
    };
    let desired = Vec2::new(110.0, 9.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(4), Color32::from_rgb(13, 19, 30));
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
    painter.rect_filled(bar_rect, CornerRadius::same(3), col);
}

pub fn draw_war_frontline_bar(
    ui: &mut egui::Ui,
    our_name: &str,
    enemy_name: &str,
    war_score: f64,
) {
    let ws = war_score.clamp(-100.0, 100.0) as f32;
    let frac = (ws + 100.0) / 200.0; // 0.0 = enemy victory, 0.5 = stalemate, 1.0 = our victory
    let annex_frac = (70.0 + 100.0) / 200.0;

    ui.horizontal(|ui| {
        ui.label(
            RichText::new(format!("⚔ FRONT: {}", our_name))
                .color(ACCENT_GREEN)
                .size(11.0)
                .strong(),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(format!("{} 🛡", enemy_name))
                    .color(ACCENT_RED)
                    .size(11.0)
                    .strong(),
            );
        });
    });

    let desired = Vec2::new(ui.available_width().max(160.0), 14.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let p = ui.painter();

    // Enemy side background
    p.rect_filled(
        rect,
        CornerRadius::same(4),
        Color32::from_rgba_unmultiplied(220, 50, 50, 110),
    );
    // Our territory push
    let split_x = rect.left() + rect.width() * frac;
    let our_rect = Rect::from_min_max(rect.min, Pos2::new(split_x, rect.bottom()));
    p.rect_filled(
        our_rect,
        CornerRadius::same(4),
        Color32::from_rgba_unmultiplied(34, 197, 94, 175),
    );
    // Center 0 marker
    let mid_x = rect.center().x;
    p.line_segment(
        [Pos2::new(mid_x, rect.top()), Pos2::new(mid_x, rect.bottom())],
        Stroke::new(1.0_f32, Color32::from_white_alpha(120)),
    );
    // +70 Annexation threshold marker
    let annex_x = rect.left() + rect.width() * annex_frac;
    p.line_segment(
        [
            Pos2::new(annex_x, rect.top() - 1.0),
            Pos2::new(annex_x, rect.bottom() + 1.0),
        ],
        Stroke::new(2.0_f32, ACCENT_GOLD),
    );
    // Frontline spearhead indicator
    p.circle_filled(Pos2::new(split_x, rect.center().y), 5.0, TEXT_PRIMARY);
    p.circle_stroke(
        Pos2::new(split_x, rect.center().y),
        5.0,
        Stroke::new(1.5_f32, BG_CANVAS),
    );
    // Frame border
    p.rect_stroke(
        rect,
        CornerRadius::same(4),
        Stroke::new(1.0_f32, BORDER_SUBTLE),
        egui::StrokeKind::Inside,
    );
    ui.label(
        RichText::new(format!(
            "War Score: {:+.0}%  |  Gold tick = +70% Direct Annexation threshold  |  +95% = Total Capitulation",
            war_score
        ))
        .color(if war_score >= 70.0 { ACCENT_GOLD } else { TEXT_MUTED })
        .size(10.5),
    );
}

fn draw_star_5(painter: &egui::Painter, center: Pos2, outer_r: f32, color: Color32) {
    let inner_r = outer_r * 0.42;
    let mut pts = Vec::with_capacity(10);
    for i in 0..10 {
        let angle = -std::f32::consts::FRAC_PI_2 + (i as f32) * std::f32::consts::PI / 5.0;
        let r = if i % 2 == 0 { outer_r } else { inner_r };
        pts.push(Pos2::new(
            center.x + r * angle.cos(),
            center.y + r * angle.sin(),
        ));
    }
    // Draw triangles from center so concave star fills cleanly in egui
    for i in 0..10 {
        let a = pts[i];
        let b = pts[(i + 1) % 10];
        painter.add(egui::Shape::convex_polygon(
            vec![center, a, b],
            color,
            Stroke::NONE,
        ));
    }
}

fn paint_flag_finish(painter: &egui::Painter, rect: Rect, gold_border: bool) {
    // Diagonal satin fabric sheen
    let sheen = vec![
        rect.left_top(),
        Pos2::new(rect.left() + rect.width() * 0.52, rect.top()),
        Pos2::new(rect.left() + rect.width() * 0.22, rect.bottom()),
        rect.left_bottom(),
    ];
    painter.add(egui::Shape::convex_polygon(
        sheen,
        Color32::from_rgba_unmultiplied(255, 255, 255, 22),
        Stroke::NONE,
    ));
    // Subtle fold shadow
    let shade = Rect::from_min_max(
        Pos2::new(rect.left() + rect.width() * 0.62, rect.top()),
        Pos2::new(rect.left() + rect.width() * 0.78, rect.bottom()),
    );
    painter.rect_filled(
        shade,
        CornerRadius::ZERO,
        Color32::from_rgba_unmultiplied(0, 0, 0, 26),
    );
    let border_col = if gold_border {
        ACCENT_GOLD
    } else {
        Color32::from_rgb(130, 150, 180)
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

    // Base field determined by Regime
    match regime {
        RegimeType::LiberalDemocracy => {
            // Classic horizontal tricolor: Deep Azure / Crisp White / Royal Crimson
            let s1 = Rect::from_min_size(rect.min, Vec2::new(w, h / 3.0));
            let s2 = Rect::from_min_size(Pos2::new(rect.left(), rect.top() + h / 3.0), Vec2::new(w, h / 3.0));
            let s3 = Rect::from_min_size(Pos2::new(rect.left(), rect.top() + 2.0 * h / 3.0), Vec2::new(w, h / 3.0));
            painter.rect_filled(s1, CornerRadius::same(2), Color32::from_rgb(29, 78, 216));
            painter.rect_filled(s2, CornerRadius::ZERO, Color32::from_rgb(241, 245, 249));
            painter.rect_filled(s3, CornerRadius::same(2), Color32::from_rgb(190, 24, 60));
            draw_star_5(painter, c, h * 0.20, Color32::from_rgb(234, 179, 8));
        }
        RegimeType::SocialDemocracy => {
            // Nordic cross: Cobalt Blue field with Golden-Yellow Cross
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(14, 116, 144));
            let cross_w = h * 0.22;
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(rect.left(), c.y - cross_w * 0.5), Vec2::new(w, cross_w)),
                CornerRadius::ZERO,
                ACCENT_GOLD,
            );
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(rect.left() + w * 0.30, rect.top()), Vec2::new(cross_w, h)),
                CornerRadius::ZERO,
                ACCENT_GOLD,
            );
        }
        RegimeType::OnePartyCommunist => {
            // Bold Socialist Crimson banner with golden vanguard star & canton stripe
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(185, 28, 28));
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(rect.left(), rect.bottom() - h * 0.16), Vec2::new(w, h * 0.16)),
                CornerRadius::same(1),
                Color32::from_rgb(234, 179, 8),
            );
            draw_star_5(
                painter,
                Pos2::new(rect.left() + w * 0.28, rect.top() + h * 0.42),
                h * 0.27,
                ACCENT_GOLD,
            );
        }
        RegimeType::MilitaryDictatorship => {
            // Armored Slate / Olive & Crimson chevron banner
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(38, 56, 46));
            let diag = vec![
                rect.left_top(),
                Pos2::new(rect.left() + w * 0.45, c.y),
                rect.left_bottom(),
            ];
            painter.add(egui::Shape::convex_polygon(
                diag,
                Color32::from_rgb(153, 27, 27),
                Stroke::NONE,
            ));
            let shield = vec![
                Pos2::new(c.x + w * 0.10, c.y - h * 0.24),
                Pos2::new(c.x + w * 0.24, c.y),
                Pos2::new(c.x + w * 0.10, c.y + h * 0.24),
                Pos2::new(c.x - w * 0.04, c.y),
            ];
            painter.add(egui::Shape::convex_polygon(shield, ACCENT_GOLD, Stroke::NONE));
        }
        RegimeType::AbsoluteAutocracy => {
            // Imperial Royal Purple & Obsidian split with golden crown diamond
            painter.rect_filled(
                Rect::from_min_size(rect.min, Vec2::new(w, h * 0.5)),
                CornerRadius::same(2),
                Color32::from_rgb(88, 28, 135),
            );
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(rect.left(), c.y), Vec2::new(w, h * 0.5)),
                CornerRadius::same(2),
                Color32::from_rgb(23, 28, 42),
            );
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(rect.left(), c.y - h * 0.07), Vec2::new(w, h * 0.14)),
                CornerRadius::ZERO,
                ACCENT_GOLD,
            );
            draw_star_5(painter, c, h * 0.24, ACCENT_GOLD);
        }
        RegimeType::TheocraticRepublic => {
            // Sacred Emerald & Ivory vertical panels with Golden Crescent/Sun
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(6, 95, 70));
            painter.rect_filled(
                Rect::from_min_size(rect.min, Vec2::new(w * 0.26, h)),
                CornerRadius::same(2),
                Color32::from_rgb(241, 245, 249),
            );
            let emblem_c = Pos2::new(rect.left() + w * 0.62, c.y);
            painter.circle_filled(emblem_c, h * 0.24, ACCENT_GOLD);
            painter.circle_filled(
                Pos2::new(emblem_c.x + h * 0.09, emblem_c.y - h * 0.03),
                h * 0.19,
                Color32::from_rgb(6, 95, 70),
            );
            draw_star_5(
                painter,
                Pos2::new(emblem_c.x + h * 0.16, emblem_c.y - h * 0.02),
                h * 0.11,
                ACCENT_GOLD,
            );
        }
        RegimeType::OligarchicRepublic => {
            // Sovereign Navy with Golden Rhombus & Emerald Core
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(15, 36, 74));
            let diamond = vec![
                Pos2::new(c.x, rect.top() + h * 0.14),
                Pos2::new(rect.right() - w * 0.18, c.y),
                Pos2::new(c.x, rect.bottom() - h * 0.14),
                Pos2::new(rect.left() + w * 0.18, c.y),
            ];
            painter.add(egui::Shape::convex_polygon(diamond, ACCENT_GOLD, Stroke::NONE));
            painter.circle_filled(c, h * 0.16, Color32::from_rgb(16, 185, 129));
        }
    }

    // Subtle economic/religious accent pip in upper-right when distinct
    if economy == EconomicSystem::WarEconomy {
        painter.circle_filled(
            Pos2::new(rect.right() - w * 0.14, rect.top() + h * 0.22),
            h * 0.10,
            ACCENT_RED,
        );
    } else if religion == ReligionPolicy::TheocraticLaw && regime != RegimeType::TheocraticRepublic {
        painter.circle_stroke(
            c,
            h * 0.30,
            Stroke::new(1.3_f32, ACCENT_GOLD),
        );
    }

    // Golden Mandate / Hegemon radiant laurel ring
    if god_mode || hegemon {
        painter.circle_stroke(c, h * 0.36, Stroke::new(1.5_f32, ACCENT_GOLD));
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

pub fn paint_foreign_flag_at(painter: &egui::Painter, rect: Rect, power: &ForeignPower) {
    let w = rect.width();
    let h = rect.height();
    let c = rect.center();

    match power.id {
        ForeignPowerId::AtlanticRepublic => {
            // 5 horizontal stripes + deep navy canton with golden star
            let stripe_h = h / 5.0;
            for i in 0..5 {
                let col = if i % 2 == 0 {
                    Color32::from_rgb(185, 28, 45)
                } else {
                    Color32::from_rgb(241, 245, 249)
                };
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(rect.left(), rect.top() + i as f32 * stripe_h), Vec2::new(w, stripe_h + 0.5)),
                    CornerRadius::ZERO,
                    col,
                );
            }
            let canton = Rect::from_min_size(rect.min, Vec2::new(w * 0.44, h * 0.60));
            painter.rect_filled(canton, CornerRadius::same(1), Color32::from_rgb(23, 52, 118));
            draw_star_5(painter, canton.center(), h * 0.16, Color32::from_rgb(250, 204, 21));
        }
        ForeignPowerId::PeoplesUnion => {
            // Crimson field with prominent gold star + 3 arc stars
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(190, 20, 32));
            draw_star_5(
                painter,
                Pos2::new(rect.left() + w * 0.25, rect.top() + h * 0.38),
                h * 0.24,
                ACCENT_GOLD,
            );
            for &(dx, dy) in &[(0.48, 0.22), (0.54, 0.42), (0.47, 0.60)] {
                draw_star_5(
                    painter,
                    Pos2::new(rect.left() + w * dx, rect.top() + h * dy),
                    h * 0.09,
                    ACCENT_GOLD,
                );
            }
        }
        ForeignPowerId::EurasianFederation => {
            // White / Royal Blue / Crimson horizontal tricolor with gold crest
            let sh = h / 3.0;
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w, sh)), CornerRadius::same(1), Color32::from_rgb(241, 245, 249));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), rect.top() + sh), Vec2::new(w, sh)), CornerRadius::ZERO, Color32::from_rgb(29, 78, 216));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), rect.top() + 2.0 * sh), Vec2::new(w, sh)), CornerRadius::same(1), Color32::from_rgb(185, 28, 28));
            painter.circle_filled(c, h * 0.15, ACCENT_GOLD);
        }
        ForeignPowerId::CrescentCoalition => {
            // Rich Emerald & Black hoist with Golden Crescent
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(5, 110, 74));
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w * 0.22, h)), CornerRadius::same(1), Color32::from_rgb(22, 26, 36));
            let ec = Pos2::new(rect.left() + w * 0.58, c.y);
            painter.circle_filled(ec, h * 0.25, ACCENT_GOLD);
            painter.circle_filled(Pos2::new(ec.x + h * 0.09, ec.y - h * 0.02), h * 0.20, Color32::from_rgb(5, 110, 74));
            draw_star_5(painter, Pos2::new(ec.x + h * 0.17, ec.y - h * 0.02), h * 0.11, ACCENT_GOLD);
        }
        ForeignPowerId::BorderlandRepublic => {
            // Sky Azure over Golden Wheat with crisp white mountain chevron
            painter.rect_filled(Rect::from_min_size(rect.min, Vec2::new(w, h * 0.55)), CornerRadius::same(1), Color32::from_rgb(14, 116, 185));
            painter.rect_filled(Rect::from_min_size(Pos2::new(rect.left(), rect.top() + h * 0.55), Vec2::new(w, h * 0.45)), CornerRadius::same(1), Color32::from_rgb(217, 155, 22));
            let peak = vec![
                Pos2::new(c.x, rect.top() + h * 0.20),
                Pos2::new(c.x + w * 0.22, rect.bottom() - h * 0.15),
                Pos2::new(c.x - w * 0.22, rect.bottom() - h * 0.15),
            ];
            painter.add(egui::Shape::convex_polygon(peak, Color32::from_rgb(241, 245, 249), Stroke::NONE));
        }
        ForeignPowerId::GlobalMonetaryCouncil => {
            // UN-inspired Royal Teal/Blue with Golden Globe Rings
            painter.rect_filled(rect, CornerRadius::same(2), Color32::from_rgb(18, 86, 150));
            painter.circle_stroke(c, h * 0.29, Stroke::new(1.5_f32, ACCENT_GOLD));
            painter.circle_stroke(c, h * 0.16, Stroke::new(1.1_f32, Color32::from_rgb(241, 245, 249)));
            painter.line_segment(
                [Pos2::new(c.x - h * 0.28, c.y), Pos2::new(c.x + h * 0.28, c.y)],
                Stroke::new(1.0_f32, ACCENT_GOLD),
            );
            painter.line_segment(
                [Pos2::new(c.x, c.y - h * 0.28), Pos2::new(c.x, c.y + h * 0.28)],
                Stroke::new(1.0_f32, ACCENT_GOLD),
            );
        }
    }

    if power.annexed {
        // Conquest dark veil + golden diagonal ribbon
        painter.rect_filled(
            rect,
            CornerRadius::same(2),
            Color32::from_rgba_unmultiplied(10, 18, 28, 120),
        );
        painter.line_segment(
            [rect.left_bottom(), rect.right_top()],
            Stroke::new(2.5_f32, ACCENT_GOLD),
        );
        draw_star_5(painter, c, h * 0.24, ACCENT_GOLD);
    } else if power.puppet_state {
        // Purple/gold client-state corner mark
        let badge_c = Pos2::new(rect.right() - w * 0.16, rect.top() + h * 0.24);
        painter.circle_filled(badge_c, h * 0.15, ACCENT_PURPLE);
        painter.circle_stroke(badge_c, h * 0.15, Stroke::new(1.0_f32, ACCENT_GOLD));
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
        BORDER_GOLD
    } else {
        BORDER_SUBTLE
    })
    .show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new("🌍 STRATEGIC GEOPOLITICAL THEATER MAP")
                    .color(ACCENT_GOLD)
                    .size(metrics.fs(13.5))
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                badge_scaled(
                    ui,
                    &format!(
                        "EMPIRE: {} ANNEXED • {} CLIENT",
                        state.annexed_count(),
                        state.puppet_count()
                    ),
                    if state.annexed_count() > 0 { ACCENT_GOLD } else { ACCENT_BLUE },
                    metrics.fs(10.5),
                );
                if state.active_wars_count() > 0 {
                    badge_scaled(
                        ui,
                        &format!("⚔ {} ACTIVE WAR(S)", state.active_wars_count()),
                        ACCENT_RED,
                        metrics.fs(10.5),
                    );
                }
            });
        });
        ui.add_space(6.0);

        let avail_w = ui.available_width().max(320.0);
        let map_h = (avail_w * 0.29).clamp(205.0, 320.0);
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(avail_w, map_h), egui::Sense::click());
        let p = ui.painter();

        // Tactical ocean/grid canvas
        p.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(9, 15, 25));
        let grid_col = Color32::from_rgba_unmultiplied(56, 189, 248, 18);
        let cols = 10;
        let rows = 5;
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

        // Radar range rings centered on player's sovereign capital
        let center = Pos2::new(rect.left() + rect.width() * 0.50, rect.top() + rect.height() * 0.52);
        let base_r = (map_h * 0.22).clamp(36.0, 68.0);
        p.circle_stroke(
            center,
            base_r * 1.45,
            Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(56, 189, 248, 30)),
        );
        p.circle_stroke(
            center,
            base_r * 2.25,
            Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(56, 189, 248, 20)),
        );

        // Positions for each foreign power (normalized inside map rect)
        let node_pos = |id: ForeignPowerId| -> Pos2 {
            let (nx, ny) = match id {
                ForeignPowerId::AtlanticRepublic => (0.14, 0.30),
                ForeignPowerId::PeoplesUnion => (0.86, 0.28),
                ForeignPowerId::EurasianFederation => (0.76, 0.76),
                ForeignPowerId::CrescentCoalition => (0.22, 0.78),
                ForeignPowerId::BorderlandRepublic => (0.50, 0.17),
                ForeignPowerId::GlobalMonetaryCouncil => (0.50, 0.86),
            };
            Pos2::new(rect.left() + rect.width() * nx, rect.top() + rect.height() * ny)
        };

        let flag_h = (map_h * 0.105).clamp(18.0, 29.0);
        let flag_w = flag_h * 1.55;

        // Draw connecting strategic lanes first
        for fp in &state.foreign_powers {
            let target_pos = node_pos(fp.id);
            let (line_col, width): (Color32, f32) = if fp.annexed {
                (Color32::from_rgba_unmultiplied(250, 204, 21, 185), 2.4)
            } else if fp.at_war {
                (ACCENT_RED, 2.8)
            } else if fp.puppet_state {
                (ACCENT_PURPLE, 2.2)
            } else if fp.defense_alliance {
                (ACCENT_GREEN, 2.0)
            } else if fp.trade_pact {
                (ACCENT_BLUE, 1.6)
            } else if fp.sanctioning_us || fp.we_sanction_them {
                (Color32::from_rgba_unmultiplied(245, 158, 11, 120), 1.3)
            } else {
                (Color32::from_rgba_unmultiplied(120, 145, 175, 55), 1.0)
            };
            p.line_segment([center, target_pos], Stroke::new(width, line_col));

            // Midpoint battle/status indicator
            if fp.at_war && !fp.annexed {
                let mid = Pos2::new((center.x + target_pos.x) * 0.5, (center.y + target_pos.y) * 0.5);
                p.circle_filled(mid, 10.0, Color32::from_rgb(127, 29, 29));
                p.circle_stroke(mid, 10.0, Stroke::new(1.2_f32, ACCENT_RED));
                p.text(
                    mid,
                    egui::Align2::CENTER_CENTER,
                    format!("{:+.0}", fp.war_score),
                    FontId::proportional((9.5 * metrics.scale).clamp(8.5, 11.5)),
                    TEXT_PRIMARY,
                );
            }
        }

        // Draw player's Sovereign Nation hub at center
        let empire_growth = 1.0 + (state.annexed_count() as f32) * 0.12;
        let hub_r = (base_r * 0.85 * empire_growth).min(map_h * 0.34);
        let hub_fill = if state.flags.ledger_rebalance || state.is_global_hegemon() {
            Color32::from_rgba_unmultiplied(250, 204, 21, 30)
        } else {
            Color32::from_rgba_unmultiplied(56, 189, 248, 28)
        };
        let hub_stroke = if state.flags.ledger_rebalance || state.is_global_hegemon() {
            ACCENT_GOLD
        } else {
            ACCENT_BLUE
        };
        p.circle_filled(center, hub_r, hub_fill);
        p.circle_stroke(center, hub_r, Stroke::new(1.8_f32, hub_stroke));

        let our_flag_rect = Rect::from_center_size(
            Pos2::new(center.x, center.y - 7.0),
            Vec2::new(flag_w * 1.25, flag_h * 1.25),
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
            Pos2::new(center.x, our_flag_rect.bottom() + 9.0),
            egui::Align2::CENTER_CENTER,
            &state.nation_name,
            FontId::proportional((11.5 * metrics.scale).clamp(10.0, 14.0)),
            if state.flags.ledger_rebalance { ACCENT_GOLD } else { TEXT_PRIMARY },
        );

        // Draw each Foreign Power node with drawn flag & click detection
        let hover_pos = ui.input(|i| i.pointer.hover_pos());
        let click_pos = if response.clicked() {
            response.interact_pointer_pos()
        } else {
            None
        };
        let mut tooltip_text: Option<String> = None;

        for fp in &state.foreign_powers {
            let pos = node_pos(fp.id);
            let card_w = (flag_w + 78.0 * metrics.scale).clamp(112.0, 186.0);
            let card_h = (flag_h + 18.0).clamp(36.0, 52.0);
            let node_rect = Rect::from_center_size(pos, Vec2::new(card_w, card_h));
            let hovered = hover_pos.map(|hp| node_rect.contains(hp)).unwrap_or(false);

            let border_col = if fp.annexed {
                ACCENT_GOLD
            } else if fp.at_war {
                ACCENT_RED
            } else if fp.puppet_state {
                ACCENT_PURPLE
            } else if hovered {
                ACCENT_CYAN
            } else {
                BORDER_SUBTLE
            };
            p.rect_filled(
                node_rect,
                CornerRadius::same(6),
                if hovered {
                    Color32::from_rgb(30, 44, 66)
                } else {
                    Color32::from_rgba_unmultiplied(17, 25, 39, 235)
                },
            );
            p.rect_stroke(
                node_rect,
                CornerRadius::same(6),
                Stroke::new(if hovered || fp.at_war || fp.annexed { 1.5_f32 } else { 1.0_f32 }, border_col),
                egui::StrokeKind::Inside,
            );

            let f_rect = Rect::from_min_size(
                Pos2::new(node_rect.left() + 6.0, node_rect.center().y - flag_h * 0.5),
                Vec2::new(flag_w, flag_h),
            );
            paint_foreign_flag_at(p, f_rect, fp);

            let text_x = f_rect.right() + 6.0;
            p.text(
                Pos2::new(text_x, node_rect.center().y - 6.5),
                egui::Align2::LEFT_CENTER,
                &fp.short_name,
                FontId::proportional((10.5 * metrics.scale).clamp(9.5, 13.0)),
                TEXT_PRIMARY,
            );
            let (sub_txt, sub_col) = if fp.annexed {
                ("★ ANNEXED".to_string(), ACCENT_GOLD)
            } else if fp.at_war {
                (format!("⚔ WAR {:+.0}", fp.war_score), ACCENT_RED)
            } else if fp.puppet_state {
                ("👑 CLIENT".to_string(), ACCENT_PURPLE)
            } else if fp.defense_alliance {
                (format!("🛡 ALLY {:+.0}", fp.relation), ACCENT_GREEN)
            } else {
                (format!("Rel {:+.0}", fp.relation), severity_color(fp.relation + 50.0, true))
            };
            p.text(
                Pos2::new(text_x, node_rect.center().y + 7.5),
                egui::Align2::LEFT_CENTER,
                sub_txt,
                FontId::proportional((9.5 * metrics.scale).clamp(8.5, 11.5)),
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
            Stroke::new(1.0_f32, BORDER_SUBTLE),
            egui::StrokeKind::Inside,
        );

        if let Some(tip) = tooltip_text {
            response.on_hover_text(tip);
        }
    });

    clicked_power
}

pub fn compute_sovereign_power_score(state: &NationState) -> (f64, &'static str, Color32) {
    let score = (state.gdp_bn / 18.0).min(1200.0)
        + state.active_troops_k * 0.25
        + state.military_tech * 1.4
        + state.tech_level * 1.2
        + state.stability * 1.1
        + (state.nuclear_status.stage() as f64) * 55.0
        + (state.annexed_count() as f64) * 180.0
        + (state.puppet_count() as f64) * 110.0;

    if state.is_global_hegemon() {
        (score, "GLOBAL HEGEMON", ACCENT_GOLD)
    } else if score >= 1400.0 {
        (score, "HYPERPOWER", ACCENT_GOLD)
    } else if score >= 900.0 {
        (score, "SUPERPOWER", ACCENT_PURPLE)
    } else if score >= 520.0 {
        (score, "GREAT POWER", ACCENT_BLUE)
    } else if score >= 300.0 {
        (score, "REGIONAL POWER", ACCENT_GREEN)
    } else {
        (score, "EMERGING STATE", ACCENT_AMBER)
    }
}

pub fn compute_achievements(state: &NationState) -> Vec<(&'static str, &'static str, &'static str, bool, Color32)> {
    vec![
        (
            "🏆",
            "Trillionaire",
            "Reach $1.00T Nominal GDP",
            state.gdp_bn >= 1000.0,
            ACCENT_GREEN,
        ),
        (
            "💎",
            "Debt-Free",
            "Eliminate all sovereign national debt",
            state.national_debt_bn <= 0.1,
            ACCENT_CYAN,
        ),
        (
            "☢",
            "Atomic Triad",
            "Complete Strategic ICBM Triad",
            state.nuclear_status == NuclearStatus::StrategicTriad,
            ACCENT_AMBER,
        ),
        (
            "⚔",
            "Conqueror",
            "Annex at least 1 rival country after war",
            state.annexed_count() >= 1,
            ACCENT_RED,
        ),
        (
            "👑",
            "Global Hegemon",
            "Annex or vassalize all 5 rival powers",
            state.is_global_hegemon(),
            ACCENT_GOLD,
        ),
        (
            "🕊",
            "Tax-Free Haven",
            "Operate with 0% national taxes",
            state.taxes.income_tax <= 0.01
                && state.taxes.corporate_tax <= 0.01
                && state.taxes.tariffs <= 0.01
                && state.taxes.wealth_tax <= 0.01,
            ACCENT_BLUE,
        ),
        (
            "🛡",
            "Iron Mandate",
            "Maintain 85%+ Stability & 0% Coup Risk",
            state.stability >= 85.0 && state.coup_risk <= 1.0,
            ACCENT_PURPLE,
        ),
        (
            "⭐",
            "Golden Era",
            "Unlock the Sovereign Golden Mandate",
            state.flags.ledger_rebalance,
            ACCENT_GOLD,
        ),
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
            ui.label(RichText::new(title).color(TEXT_MUTED).size(11.5).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("{:.1}{} ({:+.1})", latest, unit_suffix, delta))
                        .color(color)
                        .size(12.0)
                        .strong(),
                );
            });
        });
        ui.add_space(6.0);

        let width = ui.available_width().max(130.0);
        let height = (width * 0.24).clamp(64.0, 96.0);
        let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
        let painter = ui.painter();

        // Subtle chart frame & grid lines
        painter.rect_filled(rect, CornerRadius::same(5), BG_CANVAS);
        for i in 1..4 {
            let y = rect.top() + rect.height() * (i as f32 / 4.0);
            painter.line_segment(
                [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                Stroke::new(0.6_f32, Color32::from_rgb(28, 38, 56)),
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
