use crate::models::*;
use crate::ui::widgets::*;
use eframe::egui::{self, Color32, CornerRadius, FontId, Margin, Pos2, Rect, RichText, Stroke, Vec2};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CinematicKind {
    ElectionVictory,
    Revolution,
    MilitaryTakeover,
    SacredAscension,
    CorporateAscent,
    FoundingNewHomeland,
}

impl CinematicKind {
    pub fn for_preset(state: &NationState) -> Self {
        match state.regime {
            RegimeType::LiberalDemocracy | RegimeType::SocialDemocracy => Self::ElectionVictory,
            RegimeType::OnePartyCommunist => Self::Revolution,
            RegimeType::MilitaryDictatorship | RegimeType::AbsoluteAutocracy => {
                Self::MilitaryTakeover
            }
            RegimeType::TheocraticRepublic => Self::SacredAscension,
            RegimeType::OligarchicRepublic => Self::CorporateAscent,
        }
    }

    pub fn duration_secs(&self) -> f32 {
        match self {
            Self::FoundingNewHomeland => 8.5,
            _ => 7.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CinematicState {
    pub kind: CinematicKind,
    pub started_at: Instant,
}

impl CinematicState {
    pub fn new(kind: CinematicKind) -> Self {
        Self {
            kind,
            started_at: Instant::now(),
        }
    }

    pub fn restart(&mut self) {
        self.started_at = Instant::now();
    }
}

pub enum CinematicAction {
    Continue,
    FinishToDashboard,
    Replay,
}

fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    Color32::from_rgba_unmultiplied(
        lerp_f32(a.r() as f32, b.r() as f32, t) as u8,
        lerp_f32(a.g() as f32, b.g() as f32, t) as u8,
        lerp_f32(a.b() as f32, b.b() as f32, t) as u8,
        lerp_f32(a.a() as f32, b.a() as f32, t) as u8,
    )
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn render_cinematic_screen(
    ctx: &egui::Context,
    state: &NationState,
    cin: &CinematicState,
    metrics: &UiMetrics,
) -> CinematicAction {
    // Request smooth 60fps animation frames
    ctx.request_repaint();

    let elapsed = cin.started_at.elapsed().as_secs_f32();
    let total = cin.kind.duration_secs();
    let progress = (elapsed / total).clamp(0.0, 1.0);

    let mut action = CinematicAction::Continue;

    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(Color32::from_rgb(7, 10, 17)).inner_margin(Margin::same(18)))
        .show(ctx, |ui| {
            let avail = ui.available_size();
            let canvas_h = (avail.y * 0.62).clamp(260.0, 560.0);
            let canvas_w = avail.x.max(400.0);

            // 1. ANIMATED CINEMATIC STAGE CANVAS
            let (rect, _) = ui.allocate_exact_size(Vec2::new(canvas_w, canvas_h), egui::Sense::hover());
            let p = ui.painter_at(rect);

            match cin.kind {
                CinematicKind::ElectionVictory => {
                    paint_election_scene(&p, rect, state, elapsed, progress);
                }
                CinematicKind::Revolution => {
                    paint_revolution_scene(&p, rect, state, elapsed, progress);
                }
                CinematicKind::MilitaryTakeover => {
                    paint_military_scene(&p, rect, state, elapsed, progress);
                }
                CinematicKind::SacredAscension => {
                    paint_sacred_scene(&p, rect, state, elapsed, progress);
                }
                CinematicKind::CorporateAscent => {
                    paint_corporate_scene(&p, rect, state, elapsed, progress);
                }
                CinematicKind::FoundingNewHomeland => {
                    paint_founding_homeland_scene(&p, rect, state, elapsed, progress);
                }
            }

            // Cinematic letterbox bars top & bottom
            let bar_h = (canvas_h * 0.055).clamp(14.0, 28.0);
            p.rect_filled(
                Rect::from_min_size(rect.min, Vec2::new(rect.width(), bar_h)),
                CornerRadius::ZERO,
                Color32::from_rgb(5, 7, 12),
            );
            p.rect_filled(
                Rect::from_min_size(Pos2::new(rect.left(), rect.bottom() - bar_h), Vec2::new(rect.width(), bar_h)),
                CornerRadius::ZERO,
                Color32::from_rgb(5, 7, 12),
            );
            p.rect_stroke(
                rect,
                CornerRadius::same(8),
                Stroke::new(1.5_f32, Color32::from_rgb(188, 146, 56)),
                egui::StrokeKind::Inside,
            );

            ui.add_space(14.0);

            // 2. EMOTIONAL NARRATIVE CAPTION CARD (Multi-beat story beats)
            let (chapter_tag, headline, narrative, accent) =
                cinematic_story_beat(cin.kind, state, elapsed);

            egui::Frame::new()
                .fill(Color32::from_rgb(15, 22, 34))
                .stroke(Stroke::new(1.2_f32, accent))
                .corner_radius(CornerRadius::same(10))
                .inner_margin(Margin::symmetric(20, 14))
                .show(ui, |ui| {
                    ui.set_min_width(canvas_w - 40.0);
                    ui.vertical_centered(|ui| {
                        badge_scaled(ui, chapter_tag, accent, metrics.fs(10.5));
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(headline)
                                .color(Color32::from_rgb(245, 248, 255))
                                .size(metrics.fs(20.0))
                                .strong(),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(narrative)
                                .color(Color32::from_rgb(188, 204, 226))
                                .size(metrics.fs(13.5)),
                        );
                    });
                });

            ui.add_space(12.0);

            // 3. TIMELINE PROGRESS & INTERACTIVE CONTROLS
            let prog_rect_size = Vec2::new(canvas_w, 6.0);
            let (prog_rect, _) = ui.allocate_exact_size(prog_rect_size, egui::Sense::hover());
            ui.painter().rect_filled(prog_rect, CornerRadius::same(3), Color32::from_rgb(24, 34, 50));
            let fill = Rect::from_min_size(prog_rect.min, Vec2::new(prog_rect.width() * progress, 6.0));
            ui.painter().rect_filled(fill, CornerRadius::same(3), accent);

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("↻ Replay Cinematic")
                                .color(Color32::from_rgb(180, 198, 220))
                                .size(metrics.fs(12.0)),
                        )
                        .min_size(egui::vec2(140.0 * metrics.scale, 34.0 * metrics.scale)),
                    )
                    .clicked()
                {
                    action = CinematicAction::Replay;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let ready = progress >= 0.96;
                    let btn_text = if ready {
                        "👑 BEGIN YOUR REIGN ▶▶"
                    } else {
                        "▶ ENTER COMMAND ROOM"
                    };
                    let btn_col = if ready {
                        Color32::from_rgb(250, 204, 21)
                    } else {
                        Color32::from_rgb(56, 189, 248)
                    };
                    let btn = egui::Button::new(
                        RichText::new(btn_text)
                            .color(btn_col)
                            .size(metrics.fs(13.5))
                            .strong(),
                    )
                    .fill(if ready {
                        Color32::from_rgb(46, 38, 16)
                    } else {
                        Color32::from_rgb(20, 38, 60)
                    })
                    .stroke(Stroke::new(1.5_f32, btn_col))
                    .min_size(egui::vec2(220.0 * metrics.scale, 38.0 * metrics.scale));

                    if ui.add(btn).clicked() {
                        action = CinematicAction::FinishToDashboard;
                    }
                });
            });
        });

    action
}

fn cinematic_story_beat(
    kind: CinematicKind,
    state: &NationState,
    t: f32,
) -> (&'static str, String, String, Color32) {
    match kind {
        CinematicKind::ElectionVictory => {
            if t < 2.4 {
                (
                    "🗳 ELECTION NIGHT — LIVE BALLOT COUNT",
                    format!("Citizens Across {} Head to the Polls", state.nation_name),
                    "Polling stations close nationwide as millions gather in illuminated city plazas awaiting the sovereign verdict of the voters...".into(),
                    Color32::from_rgb(56, 189, 248),
                )
            } else if t < 5.0 {
                (
                    "🎉 BREAKING: HISTORIC ELECTORAL VICTORY!",
                    format!("{} {} Wins a Decisive National Mandate!", state.leader_title, state.leader_name),
                    format!(
                        "Confetti rains across the capital avenues! With overwhelming popular support, {}'s democratic coalition sweeps parliament.",
                        state.nation_name
                    ),
                    Color32::from_rgb(34, 197, 94),
                )
            } else {
                (
                    "🏛 INAUGURATION & OATH OF OFFICE",
                    format!("Long Live the Republic of {}", state.nation_name),
                    format!(
                        "{} {} takes the constitutional oath beneath the rising national banner. The future of the nation now rests in your hands.",
                        state.leader_title, state.leader_name
                    ),
                    Color32::from_rgb(250, 204, 21),
                )
            }
        }
        CinematicKind::Revolution => {
            if t < 2.4 {
                (
                    "🔥 DAWN OF THE REVOLUTION",
                    format!("Factory Sirens & Red Banners Sweep {}", state.nation_name),
                    "Across steelworks, rail hubs, and barracks, united workers' councils declare the end of the old order as searchlights pierce the dawn sky...".into(),
                    Color32::from_rgb(239, 68, 68),
                )
            } else if t < 5.0 {
                (
                    "⚡ THE CITADEL FALLS — REVOLUTION TRIUMPHANT!",
                    "The Vanguard Seizes the Palace of State!".into(),
                    format!(
                        "Cheering crowds tear down old statues as revolutionary brigades secure the capital of {}. A sweeping new socialist epoch ignites.",
                        state.nation_name
                    ),
                    Color32::from_rgb(245, 158, 11),
                )
            } else {
                (
                    "★ PROCLAMATION OF THE PEOPLE'S STATE",
                    format!("{} {} Addresses the Victorious Masses", state.leader_title, state.leader_name),
                    "Beneath a crimson dawn and rising industrial banners, the Central Committee entrusts supreme command of the revolution to you.".into(),
                    Color32::from_rgb(250, 204, 21),
                )
            }
        }
        CinematicKind::MilitaryTakeover => {
            if t < 2.5 {
                (
                    "⚔ OPERATION IRON DAWN",
                    format!("Armored Columns Converge on the Capital of {}", state.nation_name),
                    "Before sunrise, elite mechanized divisions lock down broadcast towers, bridges, and ministries in a swift, disciplined maneuver...".into(),
                    Color32::from_rgb(245, 158, 11),
                )
            } else if t < 5.0 {
                (
                    "🛡 ORDER RESTORED BY SUPREME DECREE",
                    "The High Command Assumes Sovereign Authority!".into(),
                    format!(
                        "Martial trumpets echo across the parade square as generals pledge unbroken loyalty to {} {}.",
                        state.leader_title, state.leader_name
                    ),
                    Color32::from_rgb(239, 68, 68),
                )
            } else {
                (
                    "👑 SUPREME EXECUTIVE COMMAND",
                    format!("{} {} Takes the Reins of {}", state.leader_title, state.leader_name, state.nation_name),
                    "Strength, discipline, and iron sovereignty now define the state. Lead your armed forces to dominance.".into(),
                    Color32::from_rgb(250, 204, 21),
                )
            }
        }
        CinematicKind::SacredAscension => {
            if t < 2.5 {
                (
                    "✦ THE SACRED CONVOCATION",
                    format!("Lanterns & Hymns Rise Across {}", state.nation_name),
                    "From ancient domes to mountain sanctuaries, scholars and faithful assemblies convene to proclaim a new era of moral stewardship...".into(),
                    Color32::from_rgb(167, 139, 250),
                )
            } else if t < 5.0 {
                (
                    "☀ MANDATE OF THE HIGH COUNCIL",
                    format!("{} {} Confirmed as Supreme Guardian!", state.leader_title, state.leader_name),
                    "Golden rays break above the Grand Citadel as the Clerical Council seals the sacred charter of governance.".into(),
                    Color32::from_rgb(250, 204, 21),
                )
            } else {
                (
                    "🏛 GUARDIANSHIP BEGINS",
                    format!("A New Chapter Dawns for {}", state.nation_name),
                    "Balance devotion, modernization, and security as you guide your sacred republic through a turbulent world.".into(),
                    Color32::from_rgb(34, 197, 94),
                )
            }
        }
        CinematicKind::CorporateAscent => {
            if t < 2.5 {
                (
                    "📈 THE GRAND EXCHANGE CONVENES",
                    format!("Global Markets Surge Across {}", state.nation_name),
                    "Skyscrapers blaze with emerald tickers as industrial titans and financial houses assemble for the Chancellor's keynote...".into(),
                    Color32::from_rgb(34, 211, 238),
                )
            } else if t < 5.0 {
                (
                    "💎 ARCHITECT OF PROSPERITY",
                    format!("{} {} Ascends to Supreme Chancellery!", state.leader_title, state.leader_name),
                    "Capital pours into the harbor district as a bold free-market administration takes command of the federation.".into(),
                    Color32::from_rgb(34, 197, 94),
                )
            } else {
                (
                    "🏛 THE FEDERATION AWAITS",
                    format!("Lead {} to Unrivaled Economic Might", state.nation_name),
                    "Harness commerce, innovation, and strategic leverage — while keeping inequality and rivals in check.".into(),
                    Color32::from_rgb(250, 204, 21),
                )
            }
        }
        CinematicKind::FoundingNewHomeland => {
            if t < 2.4 {
                (
                    "🌅 CHAPTER I — TERRA NULLIUS (UNCLAIMED LAND)",
                    "An Untouched Frontier at the Edge of the Map".into(),
                    "Before today, there were no borders here — only quiet misty valleys, wild rivers, and endless empty plains waiting for a dream...".into(),
                    Color32::from_rgb(56, 189, 248),
                )
            } else if t < 4.9 {
                (
                    "🚩 CHAPTER II — PLANTING THE FIRST BANNER",
                    format!("{} {} Claims This Homeland for {}!", state.leader_title, state.leader_name, state.nation_name),
                    "As golden sunrise breaks over the ridge, the first pioneers drive your sovereign standard deep into virgin soil. The wind catches your flag for the very first time.".into(),
                    Color32::from_rgb(250, 204, 21),
                )
            } else if t < 7.0 {
                (
                    "🏛 CHAPTER III — FROM EMPTY EARTH, A CAPITAL RISES",
                    format!("Borders Drawn • First Lights Ignite Across {}", state.nation_name),
                    "Stone by stone, shelters become avenues, harbors open to the horizon, and a free people swear allegiance to a country that did not exist yesterday.".into(),
                    Color32::from_rgb(34, 197, 94),
                )
            } else {
                (
                    "👑 A NEW NATION IS BORN!",
                    format!("Welcome to History, {}!", state.nation_name),
                    format!(
                        "From unclaimed wilderness to a sovereign state — {} {} now stands at the helm of {}. Write a legend the world will never forget.",
                        state.leader_title, state.leader_name, state.nation_name
                    ),
                    Color32::from_rgb(250, 204, 21),
                )
            }
        }
    }
}

fn paint_sky_gradient(p: &egui::Painter, rect: Rect, top: Color32, mid: Color32, bottom: Color32) {
    let steps = 16;
    let step_h = rect.height() / steps as f32;
    for i in 0..steps {
        let f = i as f32 / (steps - 1) as f32;
        let col = if f < 0.5 {
            lerp_color(top, mid, f * 2.0)
        } else {
            lerp_color(mid, bottom, (f - 0.5) * 2.0)
        };
        let r = Rect::from_min_size(
            Pos2::new(rect.left(), rect.top() + i as f32 * step_h),
            Vec2::new(rect.width(), step_h + 1.0),
        );
        p.rect_filled(r, CornerRadius::ZERO, col);
    }
}

/// SPECIAL EMOTIONAL SCENE: Founding a Brand-New Nation on Empty Unclaimed Land!
fn paint_founding_homeland_scene(
    p: &egui::Painter,
    rect: Rect,
    state: &NationState,
    t: f32,
    _progress: f32,
) {
    let dawn_t = smoothstep(0.5, 5.5, t);
    let top_sky = lerp_color(Color32::from_rgb(8, 14, 30), Color32::from_rgb(24, 62, 118), dawn_t);
    let mid_sky = lerp_color(Color32::from_rgb(20, 32, 58), Color32::from_rgb(196, 114, 62), dawn_t);
    let bot_sky = lerp_color(Color32::from_rgb(38, 52, 74), Color32::from_rgb(248, 202, 108), dawn_t);
    paint_sky_gradient(p, rect, top_sky, mid_sky, bot_sky);

    let cx = rect.center().x;
    let horizon_y = rect.top() + rect.height() * 0.62;

    // Fading morning stars in Phase 1
    let star_alpha = ((1.0 - smoothstep(0.5, 4.0, t)) * 210.0) as u8;
    if star_alpha > 4 {
        for i in 0..36 {
            let sx = rect.left() + ((i * 97 + 23) % 100) as f32 / 100.0 * rect.width();
            let sy = rect.top() + ((i * 53 + 11) % 52) as f32 / 100.0 * rect.height();
            let twinkle = 0.6 + 0.4 * (t * 3.0 + i as f32).sin();
            p.circle_filled(
                Pos2::new(sx, sy),
                1.4,
                Color32::from_rgba_unmultiplied(235, 245, 255, (star_alpha as f32 * twinkle) as u8),
            );
        }
    }

    // Rising Golden Sun over the Empty Frontier
    let sun_rise = smoothstep(1.2, 5.2, t);
    let sun_y = lerp_f32(horizon_y + 45.0, horizon_y - rect.height() * 0.14, sun_rise);
    let sun_r = (rect.height() * 0.11).clamp(24.0, 52.0);
    if sun_rise > 0.01 {
        // Warm sun halo rings
        for k in 1..=3 {
            p.circle_filled(
                Pos2::new(cx, sun_y),
                sun_r * (1.0 + k as f32 * 0.55),
                Color32::from_rgba_unmultiplied(252, 208, 86, (32 / k) as u8),
            );
        }
        p.circle_filled(Pos2::new(cx, sun_y), sun_r, Color32::from_rgb(255, 232, 148));
    }

    // Distant Untouched Mountain Ridges
    let mtn_col = lerp_color(Color32::from_rgb(20, 28, 42), Color32::from_rgb(44, 58, 76), dawn_t);
    for &(px, pw, ph) in &[
        (0.12, 0.28, 0.19),
        (0.35, 0.30, 0.22),
        (0.66, 0.32, 0.20),
        (0.88, 0.26, 0.18),
    ] {
        let peak = vec![
            Pos2::new(rect.left() + rect.width() * px, horizon_y - rect.height() * ph),
            Pos2::new(rect.left() + rect.width() * (px + pw * 0.55), horizon_y + 10.0),
            Pos2::new(rect.left() + rect.width() * (px - pw * 0.55), horizon_y + 10.0),
        ];
        p.add(egui::Shape::convex_polygon(peak, mtn_col, Stroke::NONE));
    }

    // Vast Empty Unclaimed Plains / Valley Floor
    let land_col = lerp_color(Color32::from_rgb(16, 30, 28), Color32::from_rgb(28, 68, 54), dawn_t);
    p.rect_filled(
        Rect::from_min_max(Pos2::new(rect.left(), horizon_y), rect.max),
        CornerRadius::ZERO,
        land_col,
    );

    // Expanding Golden Sovereign Border Rings across the Unclaimed Earth (Phase 2-4)
    let border_grow = smoothstep(2.6, 6.4, t);
    if border_grow > 0.01 {
        let max_rx = rect.width() * 0.44 * border_grow;
        let max_ry = rect.height() * 0.16 * border_grow;
        let ground_c = Pos2::new(cx, horizon_y + rect.height() * 0.17);
        for ring in 0..2 {
            let mul = 1.0 - ring as f32 * 0.35;
            let mut ring_pts = Vec::with_capacity(49);
            for i in 0..=48 {
                let a = (i as f32 / 48.0) * std::f32::consts::TAU;
                ring_pts.push(Pos2::new(
                    ground_c.x + max_rx * mul * a.cos(),
                    ground_c.y + max_ry * mul * a.sin(),
                ));
            }
            p.add(egui::Shape::line(
                ring_pts,
                Stroke::new(
                    2.0_f32,
                    Color32::from_rgba_unmultiplied(250, 204, 21, (190.0 * border_grow) as u8),
                ),
            ));
        }
    }

    // Rising First Capital Skyline out of Empty Wilderness (Phase 3-4)
    let city_rise = smoothstep(4.2, 6.8, t);
    if city_rise > 0.01 {
        let buildings: [(f32, f32, f32); 11] = [
            (0.24, 0.025, 0.07),
            (0.28, 0.032, 0.11),
            (0.33, 0.022, 0.08),
            (0.38, 0.038, 0.14),
            (0.43, 0.026, 0.09),
            (0.57, 0.026, 0.10),
            (0.61, 0.036, 0.15),
            (0.66, 0.025, 0.09),
            (0.71, 0.030, 0.12),
            (0.75, 0.022, 0.07),
            (0.19, 0.022, 0.06),
        ];
        for (i, &(bx, bw, bh)) in buildings.iter().enumerate() {
            let local_rise = smoothstep(4.2 + (i as f32 * 0.12), 6.2 + (i as f32 * 0.10), t);
            let h_now = rect.height() * bh * local_rise;
            if h_now > 1.0 {
                let b_rect = Rect::from_min_max(
                    Pos2::new(rect.left() + rect.width() * bx, horizon_y + 12.0 - h_now),
                    Pos2::new(rect.left() + rect.width() * (bx + bw), horizon_y + 12.0),
                );
                p.rect_filled(b_rect, CornerRadius::same(2), Color32::from_rgb(18, 28, 42));
                // Warm pioneer city window lights
                if local_rise > 0.55 {
                    let wy = b_rect.top() + 6.0;
                    p.circle_filled(
                        Pos2::new(b_rect.center().x, wy),
                        2.0,
                        Color32::from_rgba_unmultiplied(255, 224, 120, 210),
                    );
                }
            }
        }
    }

    // Central Hilltop Mound & Flagpole Planting + Raising the Custom National Flag!
    let hill = vec![
        Pos2::new(cx - rect.width() * 0.15, rect.bottom()),
        Pos2::new(cx, horizon_y + rect.height() * 0.08),
        Pos2::new(cx + rect.width() * 0.15, rect.bottom()),
    ];
    p.add(egui::Shape::convex_polygon(hill, Color32::from_rgb(20, 48, 40), Stroke::NONE));

    let pole_plant = smoothstep(1.8, 2.7, t);
    if pole_plant > 0.01 {
        let pole_base = Pos2::new(cx, horizon_y + rect.height() * 0.14);
        let pole_h = rect.height() * 0.42 * pole_plant;
        let pole_top = Pos2::new(cx, pole_base.y - pole_h);
        p.line_segment([pole_base, pole_top], Stroke::new(3.2_f32, Color32::from_rgb(232, 224, 198)));
        p.circle_filled(pole_top, 4.5, Color32::from_rgb(250, 204, 21));

        // Hoist the Custom National Flag smoothly from base to peak!
        let hoist = smoothstep(2.5, 4.8, t);
        if hoist > 0.01 {
            let fh = (rect.height() * 0.16).clamp(38.0, 68.0);
            let fw = fh * 1.55;
            let wave = (t * 4.2).sin() * 2.5;
            let flag_top_y = lerp_f32(pole_base.y - fh - 6.0, pole_top.y + 4.0, hoist) + wave;
            let flag_rect = Rect::from_min_size(Pos2::new(cx + 3.0, flag_top_y), Vec2::new(fw, fh));
            paint_nation_flag_at(
                p,
                flag_rect,
                state.regime,
                state.economy,
                state.religion_policy,
                false,
                false,
            );
        }
    }

    // Floating Golden Pioneer Lanterns / Hope Particles
    let lantern_fade = smoothstep(3.2, 5.5, t);
    if lantern_fade > 0.01 {
        for i in 0..24 {
            let x_frac = ((i * 73 + 17) % 94 + 3) as f32 / 100.0;
            let y_base = (horizon_y + 30.0) - ((t * (14.0 + (i % 5) as f32 * 4.0) + i as f32 * 19.0) % (rect.height() * 0.65));
            let sway = (t * 2.0 + i as f32).sin() * 8.0;
            p.circle_filled(
                Pos2::new(rect.left() + rect.width() * x_frac + sway, y_base),
                2.2,
                Color32::from_rgba_unmultiplied(255, 226, 128, (185.0 * lantern_fade) as u8),
            );
        }
    }

    // Watermark "TERRA NULLIUS -> [NATION]" title overlay on canvas
    let label_alpha = (smoothstep(4.6, 6.2, t) * 245.0) as u8;
    if label_alpha > 8 {
        p.text(
            Pos2::new(cx, rect.top() + rect.height() * 0.17),
            egui::Align2::CENTER_CENTER,
            format!("FOUNDED: {}", state.nation_name.to_uppercase()),
            FontId::proportional((rect.height() * 0.072).clamp(18.0, 32.0)),
            Color32::from_rgba_unmultiplied(255, 238, 178, label_alpha),
        );
    }
}

/// DEMOCRATIC ELECTION VICTORY CINEMATIC
fn paint_election_scene(
    p: &egui::Painter,
    rect: Rect,
    state: &NationState,
    t: f32,
    _progress: f32,
) {
    paint_sky_gradient(
        p,
        rect,
        Color32::from_rgb(10, 20, 44),
        Color32::from_rgb(18, 42, 86),
        Color32::from_rgb(28, 62, 112),
    );
    let cx = rect.center().x;
    let floor_y = rect.top() + rect.height() * 0.74;

    // Sweeping celebratory spotlight beams
    for k in 0..4 {
        let base_x = rect.left() + rect.width() * (0.18 + k as f32 * 0.21);
        let sway = (t * 1.4 + k as f32 * 1.7).sin() * rect.width() * 0.14;
        let beam = vec![
            Pos2::new(base_x, floor_y),
            Pos2::new(base_x + sway - 36.0, rect.top()),
            Pos2::new(base_x + sway + 36.0, rect.top()),
        ];
        p.add(egui::Shape::convex_polygon(
            beam,
            Color32::from_rgba_unmultiplied(120, 210, 255, 24),
            Stroke::NONE,
        ));
    }

    // Animated Live Election Vote-Tally Bars (Left & Right)
    let tally = smoothstep(0.3, 3.2, t);
    let our_pct = lerp_f32(34.0, 64.8, tally);
    let opp_pct = lerp_f32(38.0, 35.2, tally);
    let bar_area_w = (rect.width() * 0.22).clamp(130.0, 240.0);
    let bx = rect.left() + 28.0;
    let by = rect.top() + rect.height() * 0.22;

    p.rect_filled(
        Rect::from_min_size(Pos2::new(bx - 10.0, by - 22.0), Vec2::new(bar_area_w + 20.0, 88.0)),
        CornerRadius::same(7),
        Color32::from_rgba_unmultiplied(10, 18, 32, 200),
    );
    p.text(
        Pos2::new(bx, by - 10.0),
        egui::Align2::LEFT_CENTER,
        "LIVE ELECTION TALLY",
        FontId::proportional(11.0),
        Color32::from_rgb(56, 189, 248),
    );
    p.rect_filled(
        Rect::from_min_size(Pos2::new(bx, by + 6.0), Vec2::new(bar_area_w * (our_pct / 100.0), 18.0)),
        CornerRadius::same(3),
        Color32::from_rgb(34, 197, 94),
    );
    p.text(
        Pos2::new(bx + 4.0, by + 15.0),
        egui::Align2::LEFT_CENTER,
        format!("{} {:.1}%", state.leader_name, our_pct),
        FontId::proportional(11.0),
        Color32::WHITE,
    );
    p.rect_filled(
        Rect::from_min_size(Pos2::new(bx, by + 32.0), Vec2::new(bar_area_w * (opp_pct / 100.0), 16.0)),
        CornerRadius::same(3),
        Color32::from_rgb(100, 116, 139),
    );
    p.text(
        Pos2::new(bx + 4.0, by + 40.0),
        egui::Align2::LEFT_CENTER,
        format!("Opposition {:.1}%", opp_pct),
        FontId::proportional(10.5),
        Color32::WHITE,
    );

    // Rising National Flag in Center Spotlight
    let flag_rise = smoothstep(1.2, 3.6, t);
    let fh = (rect.height() * 0.22).clamp(48.0, 88.0);
    let fw = fh * 1.55;
    let fy = lerp_f32(floor_y, rect.center().y - fh * 0.65, flag_rise);
    let frect = Rect::from_center_size(Pos2::new(cx, fy + fh * 0.5), Vec2::new(fw, fh));
    paint_nation_flag_at(p, frect, state.regime, state.economy, state.religion_policy, false, false);

    // Celebratory Confetti Rain after Victory Declared (t > 2.2)
    let conf_t = smoothstep(2.0, 2.8, t);
    if conf_t > 0.01 {
        let colors = [
            Color32::from_rgb(250, 204, 21),
            Color32::from_rgb(56, 189, 248),
            Color32::from_rgb(34, 197, 94),
            Color32::from_rgb(248, 250, 252),
        ];
        for i in 0..48 {
            let x = rect.left() + ((i * 83 + 19) % 98 + 1) as f32 / 100.0 * rect.width()
                + (t * 2.5 + i as f32).sin() * 14.0;
            let y = rect.top() + ((t * (55.0 + (i % 6) as f32 * 14.0) + i as f32 * 31.0) % rect.height());
            let col = colors[i % colors.len()];
            p.rect_filled(
                Rect::from_center_size(Pos2::new(x, y), Vec2::new(5.5, 3.2)),
                CornerRadius::same(1),
                Color32::from_rgba_unmultiplied(col.r(), col.g(), col.b(), (220.0 * conf_t) as u8),
            );
        }
    }

    // Cheering Crowd Silhouettes Along Bottom Plaza
    p.rect_filled(
        Rect::from_min_max(Pos2::new(rect.left(), floor_y), rect.max),
        CornerRadius::ZERO,
        Color32::from_rgb(10, 16, 28),
    );
    let crowd_n = 36;
    for i in 0..crowd_n {
        let px = rect.left() + (i as f32 + 0.5) / crowd_n as f32 * rect.width();
        let cheer = ((t * 5.5 + i as f32 * 1.1).sin().abs()) * 6.0 * conf_t;
        p.circle_filled(Pos2::new(px, floor_y - 8.0 - cheer), 7.5, Color32::from_rgb(14, 22, 38));
    }

    if t > 2.4 {
        p.text(
            Pos2::new(cx, rect.top() + rect.height() * 0.14),
            egui::Align2::CENTER_CENTER,
            "VICTORY AT THE POLLS!",
            FontId::proportional((rect.height() * 0.075).clamp(18.0, 32.0)),
            Color32::from_rgb(250, 204, 21),
        );
    }
}

/// COMMUNIST REVOLUTION CINEMATIC
fn paint_revolution_scene(
    p: &egui::Painter,
    rect: Rect,
    state: &NationState,
    t: f32,
    _progress: f32,
) {
    let dawn = smoothstep(0.5, 4.5, t);
    paint_sky_gradient(
        p,
        rect,
        lerp_color(Color32::from_rgb(22, 8, 12), Color32::from_rgb(78, 14, 22), dawn),
        lerp_color(Color32::from_rgb(56, 14, 18), Color32::from_rgb(168, 32, 36), dawn),
        lerp_color(Color32::from_rgb(92, 24, 18), Color32::from_rgb(232, 118, 38), dawn),
    );
    let cx = rect.center().x;
    let floor_y = rect.top() + rect.height() * 0.72;

    // Dramatic diagonal crimson searchlights
    for k in 0..3 {
        let bx = rect.left() + rect.width() * (0.22 + k as f32 * 0.28);
        let sway = (t * 1.2 + k as f32 * 2.1).cos() * rect.width() * 0.18;
        let beam = vec![
            Pos2::new(bx, floor_y),
            Pos2::new(bx + sway - 44.0, rect.top()),
            Pos2::new(bx + sway + 44.0, rect.top()),
        ];
        p.add(egui::Shape::convex_polygon(
            beam,
            Color32::from_rgba_unmultiplied(255, 196, 64, 26),
            Stroke::NONE,
        ));
    }

    // Industrial Citadel & Factory Silhouettes
    p.rect_filled(
        Rect::from_min_max(Pos2::new(rect.left(), floor_y), rect.max),
        CornerRadius::ZERO,
        Color32::from_rgb(16, 12, 16),
    );
    for &(bx, bw, bh) in &[
        (0.08, 0.08, 0.16),
        (0.18, 0.04, 0.26),
        (0.24, 0.07, 0.14),
        (0.68, 0.08, 0.17),
        (0.78, 0.04, 0.27),
        (0.84, 0.09, 0.15),
    ] {
        let r = Rect::from_min_max(
            Pos2::new(rect.left() + rect.width() * bx, floor_y - rect.height() * bh),
            Pos2::new(rect.left() + rect.width() * (bx + bw), floor_y),
        );
        p.rect_filled(r, CornerRadius::same(1), Color32::from_rgb(24, 16, 20));
    }

    // Rising Revolutionary Banner over Citadel
    let hoist = smoothstep(0.8, 3.8, t);
    let fh = (rect.height() * 0.24).clamp(50.0, 92.0);
    let fw = fh * 1.55;
    let fy = lerp_f32(floor_y + 10.0, rect.center().y - fh * 0.55, hoist);
    let frect = Rect::from_center_size(Pos2::new(cx, fy + fh * 0.5), Vec2::new(fw, fh));
    paint_nation_flag_at(p, frect, state.regime, state.economy, state.religion_policy, false, false);

    // Rising Embers & Revolution Sparks
    for i in 0..42 {
        let x = rect.left() + ((i * 71 + 13) % 98 + 1) as f32 / 100.0 * rect.width()
            + (t * 3.0 + i as f32).sin() * 10.0;
        let y = floor_y - ((t * (42.0 + (i % 5) as f32 * 14.0) + i as f32 * 23.0) % (rect.height() * 0.72));
        p.circle_filled(
            Pos2::new(x, y),
            2.2,
            Color32::from_rgba_unmultiplied(255, 200, 68, 210),
        );
    }

    if t > 2.2 {
        p.text(
            Pos2::new(cx, rect.top() + rect.height() * 0.14),
            egui::Align2::CENTER_CENTER,
            "REVOLUTION TRIUMPHANT!",
            FontId::proportional((rect.height() * 0.075).clamp(18.0, 32.0)),
            Color32::from_rgb(250, 204, 21),
        );
    }
}

fn paint_military_scene(
    p: &egui::Painter,
    rect: Rect,
    state: &NationState,
    t: f32,
    _progress: f32,
) {
    paint_sky_gradient(
        p,
        rect,
        Color32::from_rgb(12, 16, 22),
        Color32::from_rgb(28, 36, 46),
        Color32::from_rgb(68, 36, 24),
    );
    let cx = rect.center().x;
    let cy = rect.center().y;

    // Tactical radar sweep
    let r = rect.height() * 0.38;
    p.circle_stroke(Pos2::new(cx, cy), r, Stroke::new(1.2_f32, Color32::from_rgba_unmultiplied(245, 158, 11, 70)));
    p.circle_stroke(Pos2::new(cx, cy), r * 0.65, Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(245, 158, 11, 50)));
    let sweep = t * 1.8;
    p.line_segment(
        [Pos2::new(cx, cy), Pos2::new(cx + r * sweep.cos(), cy + r * sweep.sin())],
        Stroke::new(2.2_f32, Color32::from_rgb(245, 158, 11)),
    );

    let fh = (rect.height() * 0.24).clamp(48.0, 90.0);
    let fw = fh * 1.55;
    let scale_in = smoothstep(0.4, 2.4, t);
    let frect = Rect::from_center_size(Pos2::new(cx, cy), Vec2::new(fw * scale_in, fh * scale_in));
    if scale_in > 0.05 {
        paint_nation_flag_at(p, frect, state.regime, state.economy, state.religion_policy, false, false);
    }
    if t > 2.0 {
        p.text(
            Pos2::new(cx, rect.top() + rect.height() * 0.14),
            egui::Align2::CENTER_CENTER,
            "SUPREME COMMAND ESTABLISHED",
            FontId::proportional((rect.height() * 0.070).clamp(17.0, 30.0)),
            Color32::from_rgb(245, 158, 11),
        );
    }
}

fn paint_sacred_scene(
    p: &egui::Painter,
    rect: Rect,
    state: &NationState,
    t: f32,
    _progress: f32,
) {
    paint_sky_gradient(
        p,
        rect,
        Color32::from_rgb(16, 12, 34),
        Color32::from_rgb(52, 36, 78),
        Color32::from_rgb(156, 112, 42),
    );
    let cx = rect.center().x;
    let cy = rect.center().y;

    // Radiant golden rays
    for i in 0..12 {
        let a = (i as f32 / 12.0) * std::f32::consts::TAU + t * 0.2;
        let r = rect.height() * 0.44;
        p.line_segment(
            [Pos2::new(cx, cy), Pos2::new(cx + r * a.cos(), cy + r * a.sin())],
            Stroke::new(1.5_f32, Color32::from_rgba_unmultiplied(250, 212, 96, 65)),
        );
    }

    let fh = (rect.height() * 0.24).clamp(48.0, 90.0);
    let fw = fh * 1.55;
    let hoist = smoothstep(0.5, 3.0, t);
    let frect = Rect::from_center_size(Pos2::new(cx, lerp_f32(rect.bottom(), cy, hoist)), Vec2::new(fw, fh));
    paint_nation_flag_at(p, frect, state.regime, state.economy, state.religion_policy, false, false);

    if t > 2.0 {
        p.text(
            Pos2::new(cx, rect.top() + rect.height() * 0.14),
            egui::Align2::CENTER_CENTER,
            "THE SACRED MANDATE ASCENDS",
            FontId::proportional((rect.height() * 0.070).clamp(17.0, 30.0)),
            Color32::from_rgb(250, 204, 21),
        );
    }
}

fn paint_corporate_scene(
    p: &egui::Painter,
    rect: Rect,
    state: &NationState,
    t: f32,
    _progress: f32,
) {
    paint_sky_gradient(
        p,
        rect,
        Color32::from_rgb(8, 18, 34),
        Color32::from_rgb(14, 44, 72),
        Color32::from_rgb(16, 78, 86),
    );
    let cx = rect.center().x;
    let cy = rect.center().y;

    // Rising glowing market chart line across background
    let mut pts = Vec::new();
    let segs = 28;
    let reach = smoothstep(0.2, 4.0, t);
    for i in 0..=segs {
        let f = i as f32 / segs as f32;
        if f > reach {
            break;
        }
        let x = rect.left() + f * rect.width();
        let y = rect.bottom() - rect.height() * (0.18 + f * 0.56 + (f * 14.0 + t * 2.0).sin() * 0.035);
        pts.push(Pos2::new(x, y));
    }
    if pts.len() >= 2 {
        p.add(egui::Shape::line(pts, Stroke::new(2.6_f32, Color32::from_rgb(34, 211, 238))));
    }

    let fh = (rect.height() * 0.24).clamp(48.0, 90.0);
    let fw = fh * 1.55;
    let scale_in = smoothstep(0.5, 2.6, t);
    if scale_in > 0.05 {
        let frect = Rect::from_center_size(Pos2::new(cx, cy), Vec2::new(fw * scale_in, fh * scale_in));
        paint_nation_flag_at(p, frect, state.regime, state.economy, state.religion_policy, false, false);
    }
    if t > 2.0 {
        p.text(
            Pos2::new(cx, rect.top() + rect.height() * 0.14),
            egui::Align2::CENTER_CENTER,
            "THE GOLDEN EXCHANGE OPENS",
            FontId::proportional((rect.height() * 0.070).clamp(17.0, 30.0)),
            Color32::from_rgb(34, 211, 238),
        );
    }
}
