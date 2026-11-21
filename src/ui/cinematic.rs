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
            Self::FoundingNewHomeland => 10.5,
            _ => 9.5,
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

fn flash_pulse(t: f32, center: f32, width: f32) -> f32 {
    let d = (t - center).abs();
    if d >= width {
        0.0
    } else {
        let f = 1.0 - d / width;
        f * f
    }
}

fn typewriter_slice(text: &str, elapsed_in_beat: f32, chars_per_sec: f32) -> String {
    let total_chars = text.chars().count();
    let show = ((elapsed_in_beat.max(0.0) * chars_per_sec) as usize).min(total_chars);
    text.chars().take(show).collect()
}

fn paint_firework_burst(
    p: &egui::Painter,
    origin: Pos2,
    local_t: f32,
    max_r: f32,
    color: Color32,
) {
    if !(0.0..=1.6).contains(&local_t) {
        return;
    }
    let expand = smoothstep(0.0, 0.9, local_t);
    let fade = 1.0 - smoothstep(0.55, 1.55, local_t);
    let alpha = (fade * 235.0) as u8;
    if alpha < 4 {
        return;
    }
    let rays = 14;
    for i in 0..rays {
        let a = (i as f32 / rays as f32) * std::f32::consts::TAU + (origin.x * 0.01);
        let r_out = max_r * expand;
        let r_in = (r_out - 14.0).max(0.0);
        let grav = local_t * local_t * 14.0;
        let p0 = Pos2::new(origin.x + r_in * a.cos(), origin.y + r_in * a.sin() + grav * 0.5);
        let p1 = Pos2::new(origin.x + r_out * a.cos(), origin.y + r_out * a.sin() + grav);
        p.line_segment(
            [p0, p1],
            Stroke::new(1.8_f32, Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)),
        );
        p.circle_filled(
            p1,
            2.1,
            Color32::from_rgba_unmultiplied(255, 250, 220, alpha),
        );
    }
}

fn paint_volumetric_rays(
    p: &egui::Painter,
    source: Pos2,
    rect: Rect,
    t: f32,
    color: Color32,
    intensity: f32,
) {
    if intensity <= 0.01 {
        return;
    }
    let ray_count = 9;
    let max_len = rect.width().max(rect.height()) * 1.1;
    for i in 0..ray_count {
        let base_a = -std::f32::consts::PI * 0.95
            + (i as f32 / (ray_count - 1) as f32) * std::f32::consts::PI * 0.90;
        let sway = (t * 0.35 + i as f32 * 0.8).sin() * 0.04;
        let a1 = base_a + sway - 0.035;
        let a2 = base_a + sway + 0.035;
        let poly = vec![
            source,
            Pos2::new(source.x + max_len * a1.cos(), source.y + max_len * a1.sin()),
            Pos2::new(source.x + max_len * a2.cos(), source.y + max_len * a2.sin()),
        ];
        let a = (24.0 * intensity * (0.7 + 0.3 * (t * 1.3 + i as f32).sin())) as u8;
        p.add(egui::Shape::convex_polygon(
            poly,
            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), a),
            Stroke::NONE,
        ));
    }
}

fn paint_waving_nation_flag(
    p: &egui::Painter,
    base_rect: Rect,
    state: &NationState,
    t: f32,
) {
    // Draw main flag with gentle wind-lift motion + rippling translucent wind-highlight strips
    let lift = (t * 3.8).sin() * 2.4;
    let moved = base_rect.translate(Vec2::new(0.0, lift));
    paint_nation_flag_at(
        p,
        moved,
        state.regime,
        state.economy,
        state.religion_policy,
        state.flags.ledger_rebalance,
        state.is_global_hegemon(),
    );
    // Traveling wind ripple bands across the fabric
    let strips = 8;
    let sw = moved.width() / strips as f32;
    for i in 0..strips {
        let phase = (t * 5.2 - i as f32 * 0.7).sin();
        let r = Rect::from_min_size(
            Pos2::new(moved.left() + i as f32 * sw, moved.top()),
            Vec2::new(sw + 0.5, moved.height()),
        );
        if phase > 0.0 {
            p.rect_filled(
                r,
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(255, 255, 255, (phase * 28.0) as u8),
            );
        } else {
            p.rect_filled(
                r,
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(0, 0, 0, (-phase * 34.0) as u8),
            );
        }
    }
}

pub fn render_cinematic_screen(
    ctx: &egui::Context,
    state: &NationState,
    cin: &CinematicState,
    metrics: &UiMetrics,
) -> CinematicAction {
    ctx.request_repaint();

    let elapsed = cin.started_at.elapsed().as_secs_f32();
    let total = cin.kind.duration_secs();
    let progress = (elapsed / total).clamp(0.0, 1.0);

    let mut action = CinematicAction::Continue;

    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(Color32::from_rgb(5, 8, 14)).inner_margin(Margin::same(16)))
        .show(ctx, |ui| {
            let avail = ui.available_size();
            let canvas_h = (avail.y * 0.60).clamp(260.0, 560.0);
            let canvas_w = avail.x.max(420.0);

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

            // Dramatic Film Vignette edges
            let vig_w = rect.width() * 0.08;
            p.rect_filled(
                Rect::from_min_size(rect.min, Vec2::new(vig_w, rect.height())),
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(0, 0, 0, 68),
            );
            p.rect_filled(
                Rect::from_min_size(Pos2::new(rect.right() - vig_w, rect.top()), Vec2::new(vig_w, rect.height())),
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(0, 0, 0, 68),
            );

            // Smooth Opening Fade-From-Black + Act Transition Flash Cut
            let intro_dark = ((1.0 - smoothstep(0.0, 1.0, elapsed)) * 245.0) as u8;
            if intro_dark > 2 {
                p.rect_filled(rect, CornerRadius::ZERO, Color32::from_rgba_unmultiplied(4, 6, 12, intro_dark));
            }
            let act_flash = flash_pulse(elapsed, 2.6, 0.35).max(flash_pulse(elapsed, 5.2, 0.35));
            if act_flash > 0.02 {
                p.rect_filled(
                    rect,
                    CornerRadius::ZERO,
                    Color32::from_rgba_unmultiplied(255, 242, 200, (act_flash * 115.0) as u8),
                );
            }

            // Cinematic Letterbox Bars & Gold Archive Reticles
            let bar_h = (canvas_h * 0.068).clamp(18.0, 34.0);
            p.rect_filled(
                Rect::from_min_size(rect.min, Vec2::new(rect.width(), bar_h)),
                CornerRadius::ZERO,
                Color32::from_rgb(4, 6, 11),
            );
            p.rect_filled(
                Rect::from_min_size(Pos2::new(rect.left(), rect.bottom() - bar_h), Vec2::new(rect.width(), bar_h)),
                CornerRadius::ZERO,
                Color32::from_rgb(4, 6, 11),
            );

            // Top letterbox archive telemetry & bottom Leader Motto bar
            let blink = if ((elapsed * 2.4) as i32) % 2 == 0 { "●" } else { "○" };
            p.text(
                Pos2::new(rect.left() + 14.0, rect.top() + bar_h * 0.5),
                egui::Align2::LEFT_CENTER,
                format!("{} SOVEREIGN ARCHIVE REEL // {}", blink, state.date_label()),
                FontId::proportional((10.5 * metrics.scale).clamp(9.5, 13.0)),
                Color32::from_rgb(235, 194, 72),
            );
            p.text(
                Pos2::new(rect.right() - 14.0, rect.top() + bar_h * 0.5),
                egui::Align2::RIGHT_CENTER,
                format!("{} {} • AGE {} • {}", state.leader_title, state.leader_name, state.leader_profile.age, state.leader_profile.background.label().to_uppercase()),
                FontId::proportional((10.5 * metrics.scale).clamp(9.5, 13.0)),
                Color32::from_rgb(176, 194, 218),
            );
            if !state.leader_profile.motto.trim().is_empty() {
                p.text(
                    Pos2::new(rect.center().x, rect.bottom() - bar_h * 0.5),
                    egui::Align2::CENTER_CENTER,
                    format!("“{}”  —  {}", state.leader_profile.motto.trim(), state.leader_profile.education.trim()),
                    FontId::proportional((11.0 * metrics.scale).clamp(9.5, 13.5)),
                    Color32::from_rgb(248, 220, 128),
                );
            }

            p.rect_stroke(
                rect,
                CornerRadius::same(8),
                Stroke::new(1.6_f32, Color32::from_rgb(192, 148, 56)),
                egui::StrokeKind::Inside,
            );

            ui.add_space(12.0);

            // 2. TYPEWRITER EMOTIONAL STORY CAPTION & LEADER DOSSIER CARD
            let (chapter_tag, headline, narrative, beat_elapsed, accent) =
                cinematic_story_beat(cin.kind, state, elapsed);
            let typed_narrative = typewriter_slice(&narrative, beat_elapsed, 68.0);

            egui::Frame::new()
                .fill(Color32::from_rgb(13, 19, 31))
                .stroke(Stroke::new(1.3_f32, accent))
                .corner_radius(CornerRadius::same(10))
                .inner_margin(Margin::symmetric(20, 12))
                .show(ui, |ui| {
                    ui.set_min_width(canvas_w - 40.0);
                    ui.vertical_centered(|ui| {
                        badge_scaled(ui, chapter_tag, accent, metrics.fs(10.5));
                        ui.add_space(5.0);
                        ui.label(
                            RichText::new(headline)
                                .color(Color32::from_rgb(246, 250, 255))
                                .size(metrics.fs(20.0))
                                .strong(),
                        );
                        ui.add_space(5.0);
                        ui.label(
                            RichText::new(typed_narrative)
                                .color(Color32::from_rgb(196, 212, 232))
                                .size(metrics.fs(13.5)),
                        );
                        if !state.leader_profile.bio.trim().is_empty() && elapsed > 4.8 {
                            ui.add_space(5.0);
                            ui.label(
                                RichText::new(format!(
                                    "Dossier: {} ({}, Age {}) — {}",
                                    state.leader_name,
                                    state.leader_profile.education,
                                    state.leader_profile.age,
                                    state.leader_profile.bio.trim()
                                ))
                                .color(Color32::from_rgb(156, 174, 198))
                                .size(metrics.fs(11.5))
                                .italics(),
                            );
                        }
                    });
                });

            ui.add_space(10.0);

            // 3. TIMELINE PROGRESS BAR & CONTROLS
            let (prog_rect, _) = ui.allocate_exact_size(Vec2::new(canvas_w, 6.0), egui::Sense::hover());
            ui.painter().rect_filled(prog_rect, CornerRadius::same(3), Color32::from_rgb(24, 34, 50));
            let fill = Rect::from_min_size(prog_rect.min, Vec2::new(prog_rect.width() * progress, 6.0));
            ui.painter().rect_filled(fill, CornerRadius::same(3), accent);

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("↻ Replay Cinematic")
                                .color(Color32::from_rgb(186, 204, 226))
                                .size(metrics.fs(12.0)),
                        )
                        .min_size(egui::vec2(146.0 * metrics.scale, 35.0 * metrics.scale)),
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
                        Color32::from_rgb(48, 38, 14)
                    } else {
                        Color32::from_rgb(20, 38, 60)
                    })
                    .stroke(Stroke::new(1.6_f32, btn_col))
                    .min_size(egui::vec2(230.0 * metrics.scale, 38.0 * metrics.scale));

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
) -> (&'static str, String, String, f32, Color32) {
    let edu = state.leader_profile.education.trim();
    let age = state.leader_profile.age;
    match kind {
        CinematicKind::ElectionVictory => {
            if t < 2.6 {
                (
                    "🗳 ACT I — ELECTION NIGHT BROADCAST",
                    format!("Ballots Tallied Across Every Province of {}", state.nation_name),
                    format!(
                        "Searchlights sweep the capital plaza as 98% of districts report. At age {}, {} {} ({}) surges ahead in every battleground province...",
                        age, state.leader_title, state.leader_name, edu
                    ),
                    t,
                    Color32::from_rgb(56, 189, 248),
                )
            } else if t < 5.4 {
                (
                    "🎉 ACT II — BREAKING: HISTORIC LANDSLIDE VICTORY!",
                    format!("{} {} Wins a Resounding Mandate!", state.leader_title, state.leader_name),
                    format!(
                        "Fireworks erupt above the Capitol Dome! Camera flashes light up the night as millions chant across {} — a historic democratic triumph!",
                        state.nation_name
                    ),
                    t - 2.6,
                    Color32::from_rgb(34, 197, 94),
                )
            } else {
                (
                    "🏛 ACT III — OATH OF OFFICE",
                    format!("A New Democratic Dawn for {}", state.nation_name),
                    format!(
                        "Hand raised over the Constitution beneath the waving banner, {} {} steps forward to shape the century ahead.",
                        state.leader_title, state.leader_name
                    ),
                    t - 5.4,
                    Color32::from_rgb(250, 204, 21),
                )
            }
        }
        CinematicKind::Revolution => {
            if t < 2.6 {
                (
                    "🔥 ACT I — THE OLD ORDER SHATTERS",
                    format!("Factory Sirens & Crimson Banners Sweep {}", state.nation_name),
                    format!(
                        "Workers' councils and defiant regiments unite before dawn. Guided by {} {} (Age {}, {}), the revolution marches on the Citadel...",
                        state.leader_title, state.leader_name, age, edu
                    ),
                    t,
                    Color32::from_rgb(239, 68, 68),
                )
            } else if t < 5.4 {
                (
                    "⚡ ACT II — THE CITADEL FALLS!",
                    "Revolution Triumphant Across the Capital!".into(),
                    format!(
                        "Golden flares ignite the crimson sky as the palace gates swing wide! Crowds hoist the new banner of {} amid thunderous cheers.",
                        state.nation_name
                    ),
                    t - 2.6,
                    Color32::from_rgb(245, 158, 11),
                )
            } else {
                (
                    "★ ACT III — DAWN OF THE VANGUARD",
                    format!("{} {} Proclaims the People's Era", state.leader_title, state.leader_name),
                    "From the balcony overlooking a sea of torches and steel mills, supreme leadership of the revolution now rests in your hands.".into(),
                    t - 5.4,
                    Color32::from_rgb(250, 204, 21),
                )
            }
        }
        CinematicKind::MilitaryTakeover => {
            if t < 2.6 {
                (
                    "⚔ ACT I — OPERATION IRON DAWN",
                    format!("Armored Columns Lock Down {}", state.nation_name),
                    format!(
                        "Before sunrise, mechanized divisions secure every bridge and ministry under the command of {} {} (Age {}).",
                        state.leader_title, state.leader_name, age
                    ),
                    t,
                    Color32::from_rgb(245, 158, 11),
                )
            } else if t < 5.4 {
                (
                    "🛡 ACT II — SUPREME ORDER PROCLAIMED",
                    "The High Command Pledges Unbroken Loyalty!".into(),
                    format!(
                        "Supersonic jets thunder overhead in diamond formation as {} assumes iron authority over {}.",
                        state.leader_name, state.nation_name
                    ),
                    t - 2.6,
                    Color32::from_rgb(239, 68, 68),
                )
            } else {
                (
                    "👑 ACT III — COMMAND OF THE CITADEL",
                    format!("{} {} Takes Absolute Command", state.leader_title, state.leader_name),
                    "Discipline, deterrence, and sovereign strength now guide every decree.".into(),
                    t - 5.4,
                    Color32::from_rgb(250, 204, 21),
                )
            }
        }
        CinematicKind::SacredAscension => {
            if t < 2.6 {
                (
                    "✦ ACT I — THE GRAND CONVOCATION",
                    format!("Lanterns & Bells Echo Across {}", state.nation_name),
                    format!(
                        "Beneath starlit spires, assemblies gather to hear {} {} (Age {}, {}).",
                        state.leader_title, state.leader_name, age, edu
                    ),
                    t,
                    Color32::from_rgb(167, 139, 250),
                )
            } else if t < 5.4 {
                (
                    "☀ ACT II — THE SACRED MANDATE",
                    format!("{} {} Ascends as Supreme Guardian!", state.leader_title, state.leader_name),
                    "Golden sunbeams break above the sanctuary as the High Council confirms your stewardship.".into(),
                    t - 2.6,
                    Color32::from_rgb(250, 204, 21),
                )
            } else {
                (
                    "🏛 ACT III — ERA OF GUARDIANSHIP",
                    format!("Guide {} Through a Turbulent Century", state.nation_name),
                    "Balance faith, statecraft, and strength as history unfolds beneath your banner.".into(),
                    t - 5.4,
                    Color32::from_rgb(34, 197, 94),
                )
            }
        }
        CinematicKind::CorporateAscent => {
            if t < 2.6 {
                (
                    "📈 ACT I — THE GRAND EXCHANGE CONVENES",
                    format!("Skyscrapers Blaze Across {}", state.nation_name),
                    format!(
                        "Markets surge to record highs as investors back {} {} (Age {}, {}).",
                        state.leader_title, state.leader_name, age, edu
                    ),
                    t,
                    Color32::from_rgb(34, 211, 238),
                )
            } else if t < 5.4 {
                (
                    "💎 ACT II — ARCHITECT OF PROSPERITY",
                    format!("{} {} Ascends to Supreme Chancellery!", state.leader_title, state.leader_name),
                    "Fireworks shimmer across the financial harbor as global syndicates rally behind your administration.".into(),
                    t - 2.6,
                    Color32::from_rgb(34, 197, 94),
                )
            } else {
                (
                    "🏛 ACT III — THE GOLDEN EXCHANGE OPENS",
                    format!("Command the Powerhouse of {}", state.nation_name),
                    "Turn capital, innovation, and diplomacy into unstoppable geopolitical clout.".into(),
                    t - 5.4,
                    Color32::from_rgb(250, 204, 21),
                )
            }
        }
        CinematicKind::FoundingNewHomeland => {
            if t < 2.6 {
                (
                    "🌅 ACT I — TERRA NULLIUS (UNCLAIMED WILDERNESS)",
                    "Empty Land at the Edge of the Known World".into(),
                    "No cities. No borders. No flags. Only wind across untouched meadows, a silver river winding through misty valleys, and a silent horizon waiting for its founder...".into(),
                    t,
                    Color32::from_rgb(56, 189, 248),
                )
            } else if t < 5.2 {
                (
                    "🚩 ACT II — PLANTING THE FIRST STANDARD",
                    format!("{} {} Claims This Earth for {}!", state.leader_title, state.leader_name, state.nation_name),
                    format!(
                        "At age {}, with training in {} and a handful of pioneers, {} drives the flagpole into virgin soil. Golden sunrise breaks across the valley as your banner catches the wind for the first time.",
                        age, edu, state.leader_name
                    ),
                    t - 2.6,
                    Color32::from_rgb(250, 204, 21),
                )
            } else if t < 7.8 {
                (
                    "🏛 ACT III — FROM EMPTY SOIL, A CAPITAL RISES",
                    format!("Borders Drawn • First Lights Ignite in {}", state.nation_name),
                    "Golden sovereign borders expand across the frontier. Stone by stone, the first Capitol dome, bridges, and harbors rise from nothing as lanterns float into the dawn sky.".into(),
                    t - 5.2,
                    Color32::from_rgb(34, 197, 94),
                )
            } else {
                (
                    "👑 ACT IV — A NEW NATION ENTERS HISTORY!",
                    format!("Long Live {} — Born From Your Vision!", state.nation_name),
                    format!(
                        "Fireworks illuminate the brand-new skyline of {}. Yesterday this land had no name — today it stands sovereign under {} {}.",
                        state.nation_name, state.leader_title, state.leader_name
                    ),
                    t - 7.8,
                    Color32::from_rgb(250, 204, 21),
                )
            }
        }
    }
}

fn paint_sky_gradient(p: &egui::Painter, rect: Rect, top: Color32, mid: Color32, bottom: Color32) {
    let steps = 18;
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

/// SPECIAL EMOTIONAL 4-ACT EPIC: Founding a Brand-New Country on Empty Unclaimed Land!
fn paint_founding_homeland_scene(
    p: &egui::Painter,
    rect: Rect,
    state: &NationState,
    t: f32,
    _progress: f32,
) {
    let dawn_t = smoothstep(0.6, 6.2, t);
    let cam_pan = (t * 0.35).sin() * 12.0;

    let top_sky = lerp_color(Color32::from_rgb(7, 13, 28), Color32::from_rgb(22, 66, 128), dawn_t);
    let mid_sky = lerp_color(Color32::from_rgb(18, 30, 56), Color32::from_rgb(208, 118, 62), dawn_t);
    let bot_sky = lerp_color(Color32::from_rgb(36, 48, 70), Color32::from_rgb(252, 208, 114), dawn_t);
    paint_sky_gradient(p, rect, top_sky, mid_sky, bot_sky);

    let cx = rect.center().x;
    let horizon_y = rect.top() + rect.height() * 0.60;

    // Fading stars in empty wilderness twilight
    let star_alpha = ((1.0 - smoothstep(0.6, 4.4, t)) * 220.0) as u8;
    if star_alpha > 4 {
        for i in 0..44 {
            let sx = rect.left() + ((i * 97 + 23) % 100) as f32 / 100.0 * rect.width();
            let sy = rect.top() + ((i * 53 + 11) % 50) as f32 / 100.0 * rect.height();
            let twinkle = 0.55 + 0.45 * (t * 3.2 + i as f32).sin();
            p.circle_filled(
                Pos2::new(sx, sy),
                1.4,
                Color32::from_rgba_unmultiplied(238, 246, 255, (star_alpha as f32 * twinkle) as u8),
            );
        }
    }

    // Rising Golden Sun + Volumetric God Rays
    let sun_rise = smoothstep(1.4, 5.8, t);
    let sun_pos = Pos2::new(
        cx - cam_pan * 0.3,
        lerp_f32(horizon_y + 52.0, horizon_y - rect.height() * 0.16, sun_rise),
    );
    let sun_r = (rect.height() * 0.11).clamp(24.0, 52.0);
    if sun_rise > 0.01 {
        paint_volumetric_rays(p, sun_pos, rect, t, Color32::from_rgb(255, 224, 128), sun_rise);
        for k in 1..=3 {
            p.circle_filled(
                sun_pos,
                sun_r * (1.0 + k as f32 * 0.55),
                Color32::from_rgba_unmultiplied(255, 212, 92, (36 / k) as u8),
            );
        }
        p.circle_filled(sun_pos, sun_r, Color32::from_rgb(255, 236, 158));
    }

    // Drifting Soft Dawn Clouds
    for i in 0..5 {
        let cloud_x = rect.left() + (((t * (8.0 + i as f32 * 2.5) + i as f32 * 180.0) % (rect.width() + 200.0)) - 100.0);
        let cloud_y = rect.top() + rect.height() * (0.16 + (i % 3) as f32 * 0.11);
        let cw = 90.0 + (i % 3) as f32 * 35.0;
        p.rect_filled(
            Rect::from_center_size(Pos2::new(cloud_x, cloud_y), Vec2::new(cw, 14.0)),
            CornerRadius::same(7),
            Color32::from_rgba_unmultiplied(255, 236, 212, (24.0 + 26.0 * dawn_t) as u8),
        );
    }

    // Gliding Birds Silhouette across untouched peaks
    for b in 0..5 {
        let bx = rect.left() + ((t * 36.0 + b as f32 * 48.0) % (rect.width() + 60.0)) - 30.0;
        let by = rect.top() + rect.height() * (0.34 + (b as f32 * 0.025)) + (t * 4.0 + b as f32).sin() * 4.0;
        let wing = (t * 8.0 + b as f32).sin() * 3.5;
        p.line_segment([Pos2::new(bx - 6.0, by - wing), Pos2::new(bx, by)], Stroke::new(1.2_f32, Color32::from_rgb(24, 34, 48)));
        p.line_segment([Pos2::new(bx, by), Pos2::new(bx + 6.0, by - wing)], Stroke::new(1.2_f32, Color32::from_rgb(24, 34, 48)));
    }

    // Parallax Mountain Range
    let mtn_far = lerp_color(Color32::from_rgb(18, 26, 40), Color32::from_rgb(56, 68, 92), dawn_t);
    let mtn_near = lerp_color(Color32::from_rgb(14, 22, 34), Color32::from_rgb(38, 54, 72), dawn_t);
    for &(px, pw, ph, far) in &[
        (0.10, 0.30, 0.21, true),
        (0.34, 0.32, 0.24, true),
        (0.68, 0.34, 0.23, true),
        (0.90, 0.28, 0.19, true),
        (0.22, 0.26, 0.15, false),
        (0.78, 0.28, 0.16, false),
    ] {
        let shift = if far { cam_pan * 0.4 } else { cam_pan * 0.9 };
        let peak = vec![
            Pos2::new(rect.left() + rect.width() * px + shift, horizon_y - rect.height() * ph),
            Pos2::new(rect.left() + rect.width() * (px + pw * 0.55) + shift, horizon_y + 12.0),
            Pos2::new(rect.left() + rect.width() * (px - pw * 0.55) + shift, horizon_y + 12.0),
        ];
        p.add(egui::Shape::convex_polygon(peak, if far { mtn_far } else { mtn_near }, Stroke::NONE));
    }

    // Untouched Wilderness Valley + Winding Shimmering River
    let land_col = lerp_color(Color32::from_rgb(14, 28, 26), Color32::from_rgb(28, 72, 54), dawn_t);
    p.rect_filled(
        Rect::from_min_max(Pos2::new(rect.left(), horizon_y), rect.max),
        CornerRadius::ZERO,
        land_col,
    );
    let river_col = lerp_color(Color32::from_rgb(22, 48, 74), Color32::from_rgb(68, 152, 188), dawn_t);
    let river = vec![
        Pos2::new(cx - rect.width() * 0.05, horizon_y),
        Pos2::new(cx + rect.width() * 0.04, horizon_y),
        Pos2::new(cx + rect.width() * 0.22, rect.bottom()),
        Pos2::new(cx - rect.width() * 0.18, rect.bottom()),
    ];
    p.add(egui::Shape::convex_polygon(river, river_col, Stroke::NONE));
    // Sun reflection shimmer lines on river
    for r_i in 0..6 {
        let ry = horizon_y + 14.0 + r_i as f32 * ((rect.bottom() - horizon_y) / 7.0);
        let rw = 16.0 + r_i as f32 * 8.0 + (t * 4.0 + r_i as f32).sin() * 5.0;
        p.line_segment(
            [Pos2::new(cx - rw * 0.5, ry), Pos2::new(cx + rw * 0.5, ry)],
            Stroke::new(1.4_f32, Color32::from_rgba_unmultiplied(255, 232, 148, (130.0 * sun_rise) as u8)),
        );
    }

    // Expanding Golden Sovereign Border Rings across the Unclaimed Earth (Act II-IV)
    let border_grow = smoothstep(2.9, 6.8, t);
    if border_grow > 0.01 {
        let max_rx = rect.width() * 0.45 * border_grow;
        let max_ry = rect.height() * 0.17 * border_grow;
        let ground_c = Pos2::new(cx, horizon_y + rect.height() * 0.17);
        for ring in 0..2 {
            let mul = 1.0 - ring as f32 * 0.36;
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
                Stroke::new(2.2_f32, Color32::from_rgba_unmultiplied(250, 204, 21, (205.0 * border_grow) as u8)),
            ));
        }
    }

    // First Capital City & Capitol Dome Rising out of Virgin Earth (Act III-IV)
    let city_rise = smoothstep(4.8, 7.6, t);
    if city_rise > 0.01 {
        let buildings: [(f32, f32, f32); 13] = [
            (0.16, 0.022, 0.06),
            (0.20, 0.026, 0.09),
            (0.25, 0.032, 0.12),
            (0.30, 0.024, 0.09),
            (0.35, 0.042, 0.15),
            (0.41, 0.028, 0.11),
            (0.56, 0.028, 0.11),
            (0.60, 0.042, 0.16),
            (0.66, 0.026, 0.10),
            (0.71, 0.032, 0.13),
            (0.76, 0.024, 0.08),
            (0.80, 0.026, 0.07),
            (0.84, 0.020, 0.05),
        ];
        for (i, &(bx, bw, bh)) in buildings.iter().enumerate() {
            let local_rise = smoothstep(4.8 + (i as f32 * 0.11), 6.8 + (i as f32 * 0.10), t);
            let h_now = rect.height() * bh * local_rise;
            if h_now > 1.0 {
                let b_rect = Rect::from_min_max(
                    Pos2::new(rect.left() + rect.width() * bx, horizon_y + 12.0 - h_now),
                    Pos2::new(rect.left() + rect.width() * (bx + bw), horizon_y + 12.0),
                );
                p.rect_filled(b_rect, CornerRadius::same(2), Color32::from_rgb(18, 28, 44));
                if local_rise > 0.50 {
                    for w_row in 0..3 {
                        let wy = b_rect.top() + 5.0 + w_row as f32 * 8.0;
                        if wy < b_rect.bottom() - 4.0 {
                            p.circle_filled(
                                Pos2::new(b_rect.center().x, wy),
                                1.8,
                                Color32::from_rgba_unmultiplied(255, 226, 128, 220),
                            );
                        }
                    }
                }
            }
        }
    }

    // Central Founder's Hilltop + Planting Flagpole + Waving Custom National Flag!
    let hill = vec![
        Pos2::new(cx - rect.width() * 0.15, rect.bottom()),
        Pos2::new(cx, horizon_y + rect.height() * 0.07),
        Pos2::new(cx + rect.width() * 0.15, rect.bottom()),
    ];
    p.add(egui::Shape::convex_polygon(hill, Color32::from_rgb(18, 46, 38), Stroke::NONE));

    let pole_plant = smoothstep(2.2, 3.0, t);
    if pole_plant > 0.01 {
        let pole_base = Pos2::new(cx, horizon_y + rect.height() * 0.14);
        let pole_h = rect.height() * 0.44 * pole_plant;
        let pole_top = Pos2::new(cx, pole_base.y - pole_h);
        p.line_segment([pole_base, pole_top], Stroke::new(3.4_f32, Color32::from_rgb(236, 226, 198)));
        p.circle_filled(pole_top, 4.8, Color32::from_rgb(250, 204, 21));

        // Shockwave ring when pole strikes the virgin earth!
        let impact = smoothstep(2.6, 3.6, t);
        if impact > 0.01 && impact < 0.99 {
            p.circle_stroke(
                pole_base,
                8.0 + impact * 52.0,
                Stroke::new(2.0_f32, Color32::from_rgba_unmultiplied(250, 204, 21, ((1.0 - impact) * 200.0) as u8)),
            );
        }

        let hoist = smoothstep(2.8, 5.1, t);
        if hoist > 0.01 {
            let fh = (rect.height() * 0.17).clamp(40.0, 72.0);
            let fw = fh * 1.55;
            let flag_top_y = lerp_f32(pole_base.y - fh - 6.0, pole_top.y + 4.0, hoist);
            let flag_rect = Rect::from_min_size(Pos2::new(cx + 3.0, flag_top_y), Vec2::new(fw, fh));
            paint_waving_nation_flag(p, flag_rect, state, t);
        }
    }

    // Floating Golden Pioneer Sky Lanterns
    let lantern_fade = smoothstep(4.2, 6.2, t);
    if lantern_fade > 0.01 {
        for i in 0..28 {
            let x_frac = ((i * 73 + 17) % 94 + 3) as f32 / 100.0;
            let y_base = (horizon_y + 28.0) - ((t * (15.0 + (i % 5) as f32 * 4.5) + i as f32 * 21.0) % (rect.height() * 0.68));
            let sway = (t * 2.1 + i as f32).sin() * 9.0;
            p.circle_filled(
                Pos2::new(rect.left() + rect.width() * x_frac + sway, y_base),
                2.3,
                Color32::from_rgba_unmultiplied(255, 224, 120, (195.0 * lantern_fade) as u8),
            );
        }
    }

    // Celebratory Founding Fireworks in Act IV!
    if t > 7.4 {
        paint_firework_burst(p, Pos2::new(rect.left() + rect.width() * 0.24, rect.top() + rect.height() * 0.28), (t - 7.5) % 2.1, 48.0, Color32::from_rgb(250, 204, 21));
        paint_firework_burst(p, Pos2::new(rect.left() + rect.width() * 0.76, rect.top() + rect.height() * 0.26), (t - 8.0) % 2.1, 52.0, Color32::from_rgb(56, 189, 248));
        paint_firework_burst(p, Pos2::new(rect.left() + rect.width() * 0.50, rect.top() + rect.height() * 0.22), (t - 8.5) % 2.1, 46.0, Color32::from_rgb(34, 197, 94));
    }

    let label_alpha = (smoothstep(5.4, 7.0, t) * 245.0) as u8;
    if label_alpha > 8 {
        p.text(
            Pos2::new(cx, rect.top() + rect.height() * 0.16),
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
        Color32::from_rgb(8, 18, 42),
        Color32::from_rgb(16, 40, 84),
        Color32::from_rgb(28, 66, 122),
    );
    let cx = rect.center().x;
    let floor_y = rect.top() + rect.height() * 0.75;

    // Sweeping celebratory spotlight beams
    for k in 0..5 {
        let base_x = rect.left() + rect.width() * (0.14 + k as f32 * 0.18);
        let sway = (t * 1.5 + k as f32 * 1.6).sin() * rect.width() * 0.15;
        let beam = vec![
            Pos2::new(base_x, floor_y),
            Pos2::new(base_x + sway - 38.0, rect.top()),
            Pos2::new(base_x + sway + 38.0, rect.top()),
        ];
        p.add(egui::Shape::convex_polygon(
            beam,
            Color32::from_rgba_unmultiplied(120, 215, 255, 24),
            Stroke::NONE,
        ));
    }

    // Illuminated Capitol Dome Silhouette in background
    let dome_r = (rect.height() * 0.14).clamp(32.0, 68.0);
    p.circle_filled(Pos2::new(cx, floor_y - dome_r * 0.45), dome_r, Color32::from_rgb(16, 28, 52));
    p.rect_filled(
        Rect::from_center_size(Pos2::new(cx, floor_y - 16.0), Vec2::new(dome_r * 3.4, 34.0)),
        CornerRadius::same(2),
        Color32::from_rgb(14, 24, 46),
    );

    // Broadcast Live Projection Panel (Left)
    let tally = smoothstep(0.3, 3.6, t);
    let our_pct = lerp_f32(36.0, 67.4, tally);
    let opp_pct = lerp_f32(41.0, 32.6, tally);
    let bar_w = (rect.width() * 0.23).clamp(140.0, 250.0);
    let bx = rect.left() + 26.0;
    let by = rect.top() + rect.height() * 0.22;

    p.rect_filled(
        Rect::from_min_size(Pos2::new(bx - 10.0, by - 22.0), Vec2::new(bar_w + 20.0, 96.0)),
        CornerRadius::same(7),
        Color32::from_rgba_unmultiplied(8, 16, 30, 215),
    );
    p.rect_stroke(
        Rect::from_min_size(Pos2::new(bx - 10.0, by - 22.0), Vec2::new(bar_w + 20.0, 96.0)),
        CornerRadius::same(7),
        Stroke::new(1.0_f32, Color32::from_rgb(56, 189, 248)),
        egui::StrokeKind::Inside,
    );
    p.text(
        Pos2::new(bx, by - 10.0),
        egui::Align2::LEFT_CENTER,
        if t > 2.6 { "★ PROJECTED WINNER!" } else { "LIVE ELECTION DESK • 98%" },
        FontId::proportional(11.0),
        if t > 2.6 { Color32::from_rgb(250, 204, 21) } else { Color32::from_rgb(56, 189, 248) },
    );
    p.rect_filled(
        Rect::from_min_size(Pos2::new(bx, by + 6.0), Vec2::new(bar_w * (our_pct / 100.0), 20.0)),
        CornerRadius::same(3),
        Color32::from_rgb(34, 197, 94),
    );
    p.text(
        Pos2::new(bx + 5.0, by + 16.0),
        egui::Align2::LEFT_CENTER,
        format!("{} {:.1}%", state.leader_name, our_pct),
        FontId::proportional(11.0),
        Color32::WHITE,
    );
    p.rect_filled(
        Rect::from_min_size(Pos2::new(bx, by + 34.0), Vec2::new(bar_w * (opp_pct / 100.0), 17.0)),
        CornerRadius::same(3),
        Color32::from_rgb(100, 116, 139),
    );
    p.text(
        Pos2::new(bx + 5.0, by + 42.0),
        egui::Align2::LEFT_CENTER,
        format!("Opposition {:.1}%", opp_pct),
        FontId::proportional(10.5),
        Color32::WHITE,
    );

    // Rising Waving National Flag in Center Spotlight
    let flag_rise = smoothstep(1.2, 3.8, t);
    let fh = (rect.height() * 0.22).clamp(48.0, 88.0);
    let fw = fh * 1.55;
    let fy = lerp_f32(floor_y, rect.center().y - fh * 0.62, flag_rise);
    let frect = Rect::from_center_size(Pos2::new(cx, fy + fh * 0.5), Vec2::new(fw, fh));
    paint_waving_nation_flag(p, frect, state, t);

    // Victory Fireworks + Press Flashbulbs + Confetti Storm (t > 2.4)
    let conf_t = smoothstep(2.2, 3.0, t);
    if conf_t > 0.01 {
        paint_firework_burst(p, Pos2::new(rect.left() + rect.width() * 0.74, rect.top() + rect.height() * 0.28), (t - 2.6) % 2.0, 52.0, Color32::from_rgb(250, 204, 21));
        paint_firework_burst(p, Pos2::new(rect.left() + rect.width() * 0.34, rect.top() + rect.height() * 0.24), (t - 3.3) % 2.0, 46.0, Color32::from_rgb(56, 189, 248));
        paint_firework_burst(p, Pos2::new(rect.left() + rect.width() * 0.84, rect.top() + rect.height() * 0.36), (t - 4.0) % 2.0, 44.0, Color32::from_rgb(34, 197, 94));

        // Camera flashbulb pops in crowd
        for f_i in 0..8 {
            let phase = (t * 4.0 + f_i as f32 * 1.7) % 2.5;
            if phase < 0.14 {
                let fx = rect.left() + ((f_i * 113 + 37) % 90 + 5) as f32 / 100.0 * rect.width();
                let fy_p = floor_y - 6.0;
                p.circle_filled(Pos2::new(fx, fy_p), 9.0, Color32::from_rgba_unmultiplied(255, 255, 255, 190));
            }
        }

        let colors = [
            Color32::from_rgb(250, 204, 21),
            Color32::from_rgb(56, 189, 248),
            Color32::from_rgb(34, 197, 94),
            Color32::from_rgb(248, 250, 252),
        ];
        for i in 0..56 {
            let x = rect.left() + ((i * 83 + 19) % 98 + 1) as f32 / 100.0 * rect.width()
                + (t * 2.7 + i as f32).sin() * 16.0;
            let y = rect.top() + ((t * (58.0 + (i % 6) as f32 * 14.0) + i as f32 * 31.0) % rect.height());
            let col = colors[i % colors.len()];
            p.rect_filled(
                Rect::from_center_size(Pos2::new(x, y), Vec2::new(5.8, 3.2)),
                CornerRadius::same(1),
                Color32::from_rgba_unmultiplied(col.r(), col.g(), col.b(), (225.0 * conf_t) as u8),
            );
        }
    }

    // Cheering Plaza Crowd
    p.rect_filled(
        Rect::from_min_max(Pos2::new(rect.left(), floor_y), rect.max),
        CornerRadius::ZERO,
        Color32::from_rgb(9, 15, 26),
    );
    let crowd_n = 42;
    for i in 0..crowd_n {
        let px = rect.left() + (i as f32 + 0.5) / crowd_n as f32 * rect.width();
        let cheer = ((t * 6.0 + i as f32 * 1.1).sin().abs()) * 7.0 * conf_t;
        p.circle_filled(Pos2::new(px, floor_y - 7.0 - cheer), 7.5, Color32::from_rgb(13, 21, 36));
    }

    if t > 2.6 {
        p.text(
            Pos2::new(cx, rect.top() + rect.height() * 0.16),
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
    let dawn = smoothstep(0.5, 4.8, t);
    paint_sky_gradient(
        p,
        rect,
        lerp_color(Color32::from_rgb(22, 8, 12), Color32::from_rgb(84, 14, 22), dawn),
        lerp_color(Color32::from_rgb(56, 14, 18), Color32::from_rgb(176, 32, 36), dawn),
        lerp_color(Color32::from_rgb(92, 24, 18), Color32::from_rgb(238, 124, 38), dawn),
    );
    let cx = rect.center().x;
    let floor_y = rect.top() + rect.height() * 0.73;

    paint_volumetric_rays(p, Pos2::new(cx, floor_y), rect, t, Color32::from_rgb(255, 198, 72), dawn * 0.9);

    // Sweeping revolutionary searchlights
    for k in 0..4 {
        let bx = rect.left() + rect.width() * (0.16 + k as f32 * 0.22);
        let sway = (t * 1.3 + k as f32 * 2.1).cos() * rect.width() * 0.18;
        let beam = vec![
            Pos2::new(bx, floor_y),
            Pos2::new(bx + sway - 42.0, rect.top()),
            Pos2::new(bx + sway + 42.0, rect.top()),
        ];
        p.add(egui::Shape::convex_polygon(
            beam,
            Color32::from_rgba_unmultiplied(255, 200, 68, 24),
            Stroke::NONE,
        ));
    }

    // Industrial Citadel & Billowing Factory Chimneys
    p.rect_filled(
        Rect::from_min_max(Pos2::new(rect.left(), floor_y), rect.max),
        CornerRadius::ZERO,
        Color32::from_rgb(16, 11, 16),
    );
    for &(bx, bw, bh) in &[
        (0.08, 0.08, 0.16),
        (0.18, 0.04, 0.27),
        (0.24, 0.07, 0.14),
        (0.68, 0.08, 0.17),
        (0.78, 0.04, 0.28),
        (0.84, 0.09, 0.15),
    ] {
        let r = Rect::from_min_max(
            Pos2::new(rect.left() + rect.width() * bx, floor_y - rect.height() * bh),
            Pos2::new(rect.left() + rect.width() * (bx + bw), floor_y),
        );
        p.rect_filled(r, CornerRadius::same(1), Color32::from_rgb(24, 15, 20));
    }

    // Rising Revolutionary Waving Banner
    let hoist = smoothstep(0.9, 3.9, t);
    let fh = (rect.height() * 0.24).clamp(50.0, 92.0);
    let fw = fh * 1.55;
    let fy = lerp_f32(floor_y + 10.0, rect.center().y - fh * 0.55, hoist);
    let frect = Rect::from_center_size(Pos2::new(cx, fy + fh * 0.5), Vec2::new(fw, fh));
    paint_waving_nation_flag(p, frect, state, t);

    // Triumph Flares & Fireworks (t > 2.6)
    if t > 2.6 {
        paint_firework_burst(p, Pos2::new(rect.left() + rect.width() * 0.26, rect.top() + rect.height() * 0.26), (t - 2.7) % 2.1, 48.0, Color32::from_rgb(250, 204, 21));
        paint_firework_burst(p, Pos2::new(rect.left() + rect.width() * 0.74, rect.top() + rect.height() * 0.26), (t - 3.4) % 2.1, 48.0, Color32::from_rgb(239, 68, 68));
    }

    // Rising Embers & Revolution Sparks
    for i in 0..48 {
        let x = rect.left() + ((i * 71 + 13) % 98 + 1) as f32 / 100.0 * rect.width()
            + (t * 3.0 + i as f32).sin() * 12.0;
        let y = floor_y - ((t * (45.0 + (i % 5) as f32 * 14.0) + i as f32 * 23.0) % (rect.height() * 0.74));
        p.circle_filled(
            Pos2::new(x, y),
            2.3,
            Color32::from_rgba_unmultiplied(255, 204, 72, 215),
        );
    }

    if t > 2.4 {
        p.text(
            Pos2::new(cx, rect.top() + rect.height() * 0.16),
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
        Color32::from_rgb(10, 14, 20),
        Color32::from_rgb(26, 36, 48),
        Color32::from_rgb(76, 38, 22),
    );
    let cx = rect.center().x;
    let cy = rect.center().y;

    // Supersonic Jet Flyover Contrails across upper sky
    let jet_x = rect.left() + ((t * 165.0) % (rect.width() + 260.0)) - 130.0;
    let jet_y = rect.top() + rect.height() * 0.25;
    for j in -1_i32..=1 {
        let jy = jet_y + j as f32 * 14.0;
        let jx = jet_x - (j.abs() as f32) * 18.0;
        p.line_segment(
            [Pos2::new(jx - 90.0, jy), Pos2::new(jx, jy)],
            Stroke::new(1.8_f32, Color32::from_rgba_unmultiplied(235, 245, 255, 110)),
        );
        p.circle_filled(Pos2::new(jx, jy), 2.5, Color32::from_rgb(250, 204, 21));
    }

    // Tactical Radar Sweep
    let r = rect.height() * 0.38;
    p.circle_stroke(Pos2::new(cx, cy), r, Stroke::new(1.2_f32, Color32::from_rgba_unmultiplied(245, 158, 11, 75)));
    p.circle_stroke(Pos2::new(cx, cy), r * 0.65, Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(245, 158, 11, 50)));
    let sweep = t * 1.9;
    p.line_segment(
        [Pos2::new(cx, cy), Pos2::new(cx + r * sweep.cos(), cy + r * sweep.sin())],
        Stroke::new(2.2_f32, Color32::from_rgb(245, 158, 11)),
    );

    let fh = (rect.height() * 0.24).clamp(48.0, 90.0);
    let fw = fh * 1.55;
    let scale_in = smoothstep(0.4, 2.4, t);
    if scale_in > 0.05 {
        let frect = Rect::from_center_size(Pos2::new(cx, cy), Vec2::new(fw * scale_in, fh * scale_in));
        paint_waving_nation_flag(p, frect, state, t);
    }
    if t > 2.2 {
        p.text(
            Pos2::new(cx, rect.top() + rect.height() * 0.16),
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
        Color32::from_rgb(14, 10, 32),
        Color32::from_rgb(54, 36, 82),
        Color32::from_rgb(168, 118, 44),
    );
    let cx = rect.center().x;
    let cy = rect.center().y;

    paint_volumetric_rays(p, Pos2::new(cx, cy), rect, t, Color32::from_rgb(255, 224, 128), 0.9);

    let fh = (rect.height() * 0.24).clamp(48.0, 90.0);
    let fw = fh * 1.55;
    let hoist = smoothstep(0.5, 3.0, t);
    let frect = Rect::from_center_size(Pos2::new(cx, lerp_f32(rect.bottom(), cy, hoist)), Vec2::new(fw, fh));
    paint_waving_nation_flag(p, frect, state, t);

    for i in 0..24 {
        let lx = rect.left() + ((i * 79 + 11) % 94 + 3) as f32 / 100.0 * rect.width();
        let ly = rect.bottom() - ((t * 28.0 + i as f32 * 27.0) % rect.height());
        p.circle_filled(Pos2::new(lx, ly), 2.4, Color32::from_rgba_unmultiplied(255, 226, 138, 195));
    }

    if t > 2.2 {
        p.text(
            Pos2::new(cx, rect.top() + rect.height() * 0.16),
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
        Color32::from_rgb(8, 16, 32),
        Color32::from_rgb(14, 46, 76),
        Color32::from_rgb(16, 84, 92),
    );
    let cx = rect.center().x;
    let cy = rect.center().y;

    let mut pts = Vec::new();
    let segs = 32;
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
        p.add(egui::Shape::line(pts, Stroke::new(2.8_f32, Color32::from_rgb(34, 211, 238))));
    }

    let fh = (rect.height() * 0.24).clamp(48.0, 90.0);
    let fw = fh * 1.55;
    let scale_in = smoothstep(0.5, 2.6, t);
    if scale_in > 0.05 {
        let frect = Rect::from_center_size(Pos2::new(cx, cy), Vec2::new(fw * scale_in, fh * scale_in));
        paint_waving_nation_flag(p, frect, state, t);
    }
    if t > 2.8 {
        paint_firework_burst(p, Pos2::new(rect.left() + rect.width() * 0.24, rect.top() + rect.height() * 0.28), (t - 2.9) % 2.1, 46.0, Color32::from_rgb(34, 211, 238));
        paint_firework_burst(p, Pos2::new(rect.left() + rect.width() * 0.76, rect.top() + rect.height() * 0.28), (t - 3.5) % 2.1, 46.0, Color32::from_rgb(34, 197, 94));
    }
    if t > 2.2 {
        p.text(
            Pos2::new(cx, rect.top() + rect.height() * 0.16),
            egui::Align2::CENTER_CENTER,
            "THE GOLDEN EXCHANGE OPENS",
            FontId::proportional((rect.height() * 0.070).clamp(17.0, 30.0)),
            Color32::from_rgb(34, 211, 238),
        );
    }
}
