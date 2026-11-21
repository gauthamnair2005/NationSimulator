pub mod tabs;
pub mod widgets;

use crate::events::resolve_event_choice;
use crate::models::*;
use crate::simulation::{advance_quarter, rebalance_sovereign_ledger};
use crate::ui::tabs::*;
use crate::ui::widgets::*;
use eframe::egui::{self, Color32, CornerRadius, Margin, RichText, Stroke};
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppScreen {
    Setup,
    Dashboard,
}

pub struct NationSimApp {
    screen: AppScreen,
    state: NationState,
    active_tab: DashboardTab,
    log_filter: Option<LogCategory>,
    status_toast: Option<(String, Instant)>,

    // Auto-play & scaling controls
    auto_play: bool,
    sim_speed: u8, // 1, 2, or 4
    ui_zoom: f32,  // 0.9, 1.0, 1.15, 1.25
    last_tick: Instant,
    header_taps: u8,
    last_tap: Instant,

    // Custom Nation Builder fields
    custom_name: String,
    custom_leader: String,
    custom_title: String,
    custom_regime: RegimeType,
    custom_economy: EconomicSystem,
    custom_religion: ReligionPolicy,
}

impl NationSimApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_theme(&cc.egui_ctx);
        Self {
            screen: AppScreen::Setup,
            state: NationState::from_preset(ScenarioPreset::AureliaDemocracy),
            active_tab: DashboardTab::Overview,
            log_filter: None,
            status_toast: None,
            auto_play: false,
            sim_speed: 1,
            ui_zoom: 1.0,
            last_tick: Instant::now(),
            header_taps: 0,
            last_tap: Instant::now(),
            custom_name: "Republic of Novaria".into(),
            custom_leader: "Soren Lindqvist".into(),
            custom_title: "President".into(),
            custom_regime: RegimeType::LiberalDemocracy,
            custom_economy: EconomicSystem::MixedEconomy,
            custom_religion: ReligionPolicy::SecularPluralism,
        }
    }

    fn auto_interval_ms(&self) -> u64 {
        match self.sim_speed {
            4 => 320,
            2 => 720,
            _ => 1400,
        }
    }

    fn save_path() -> PathBuf {
        PathBuf::from("sovereign_save.json")
    }

    fn set_toast(&mut self, msg: impl Into<String>) {
        self.status_toast = Some((msg.into(), Instant::now()));
    }

    fn trigger_easter_egg_tap(&mut self) {
        if self.last_tap.elapsed() > Duration::from_secs(3) {
            self.header_taps = 0;
        }
        self.header_taps += 1;
        self.last_tap = Instant::now();
        if self.header_taps >= 5 {
            self.header_taps = 0;
            rebalance_sovereign_ledger(&mut self.state);
            self.set_toast("Sovereign Golden Mandate active: debt cleared, +$1.00T surplus, 0% taxes unlocked & Golden Era incidents enabled!");
        }
    }

    fn save_game(&mut self) {
        match serde_json::to_string_pretty(&self.state) {
            Ok(json) => match std::fs::write(Self::save_path(), json) {
                Ok(_) => self.set_toast("Saved state to sovereign_save.json"),
                Err(e) => self.set_toast(format!("Save failed: {}", e)),
            },
            Err(e) => self.set_toast(format!("Serialization error: {}", e)),
        }
    }

    fn load_game(&mut self) {
        match std::fs::read_to_string(Self::save_path()) {
            Ok(json) => match serde_json::from_str::<NationState>(&json) {
                Ok(loaded) => {
                    self.state = loaded;
                    self.screen = AppScreen::Dashboard;
                    self.auto_play = false;
                    self.set_toast("Loaded saved nation from sovereign_save.json");
                }
                Err(e) => self.set_toast(format!("Invalid save file: {}", e)),
            },
            Err(_) => self.set_toast("No sovereign_save.json found in workspace"),
        }
    }
}

fn configure_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = BG_PANEL;
    visuals.panel_fill = BG_CANVAS;
    visuals.faint_bg_color = BG_CARD;
    visuals.extreme_bg_color = Color32::from_rgb(8, 12, 20);
    visuals.window_corner_radius = CornerRadius::same(10);
    visuals.widgets.noninteractive.bg_fill = BG_CARD;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT_PRIMARY);
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(30, 42, 62);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(6);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(52, 70, 98));
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(42, 60, 88);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(6);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.2_f32, ACCENT_BLUE);
    visuals.widgets.active.bg_fill = Color32::from_rgb(37, 99, 235);
    visuals.widgets.active.corner_radius = CornerRadius::same(6);
    visuals.selection.bg_fill = Color32::from_rgb(30, 64, 118);
    ctx.set_visuals(visuals);
}

fn preset_flag_spec(preset: ScenarioPreset) -> (RegimeType, EconomicSystem, ReligionPolicy) {
    match preset {
        ScenarioPreset::AureliaDemocracy => (
            RegimeType::LiberalDemocracy,
            EconomicSystem::MixedEconomy,
            ReligionPolicy::SecularPluralism,
        ),
        ScenarioPreset::KrasnoviaCommunist => (
            RegimeType::OnePartyCommunist,
            EconomicSystem::PlannedCommunism,
            ReligionPolicy::StateAtheism,
        ),
        ScenarioPreset::VeridiaCapitalist => (
            RegimeType::OligarchicRepublic,
            EconomicSystem::FreeMarketCapitalism,
            ReligionPolicy::SecularPluralism,
        ),
        ScenarioPreset::SolariaDictatorship => (
            RegimeType::MilitaryDictatorship,
            EconomicSystem::StateCapitalism,
            ReligionPolicy::OfficialReligion,
        ),
        ScenarioPreset::AlNoorTheocracy => (
            RegimeType::TheocraticRepublic,
            EconomicSystem::StateCapitalism,
            ReligionPolicy::TheocraticLaw,
        ),
    }
}

impl eframe::App for NationSimApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(self.ui_zoom);

        // Handle auto-play ticking
        if self.screen == AppScreen::Dashboard
            && self.auto_play
            && self.state.active_event.is_none()
            && self.state.game_over.is_none()
        {
            if self.last_tick.elapsed() >= Duration::from_millis(self.auto_interval_ms()) {
                advance_quarter(&mut self.state);
                self.last_tick = Instant::now();
            }
            ctx.request_repaint_after(Duration::from_millis(100));
        }

        // Expire toast after 5 seconds
        if let Some((_, ts)) = &self.status_toast {
            if ts.elapsed() > Duration::from_secs(5) {
                self.status_toast = None;
            }
        }

        let metrics = UiMetrics::from_screen(ctx.screen_rect());

        match self.screen {
            AppScreen::Setup => self.render_setup_screen(ctx, &metrics),
            AppScreen::Dashboard => self.render_dashboard(ctx, &metrics),
        }
    }
}

impl NationSimApp {
    fn render_setup_screen(&mut self, ctx: &egui::Context, metrics: &UiMetrics) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(12.0);
                hero_card_frame(BORDER_GOLD).show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new("👑 SOVEREIGN // GRAND STRATEGY NATION SIMULATOR")
                                    .color(ACCENT_GOLD)
                                    .size(metrics.fs(22.0))
                                    .strong(),
                            );
                            ui.label(
                                RichText::new(
                                    "Forge Empires • Command Armies • Conquer Rivals • Shape Ideologies, Economies & History",
                                )
                                .color(TEXT_MUTED)
                                .size(metrics.fs(12.5)),
                            );
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if Self::save_path().exists()
                                && ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new("📂 CONTINUE CAMPAIGN")
                                                .color(ACCENT_GREEN)
                                                .size(metrics.fs(13.0))
                                                .strong(),
                                        )
                                        .min_size(egui::vec2(170.0 * metrics.scale, 34.0 * metrics.scale)),
                                    )
                                    .clicked()
                            {
                                self.load_game();
                            }
                        });
                    });
                });

                ui.add_space(14.0);
                ui.label(
                    RichText::new("⚔ SELECT A CAMPAIGN SCENARIO")
                        .color(ACCENT_CYAN)
                        .size(metrics.fs(14.5))
                        .strong(),
                );
                ui.add_space(8.0);

                let mut chosen_preset: Option<ScenarioPreset> = None;
                let preset_flag_h = (metrics.flag_h * 1.25).clamp(28.0, 52.0);
                let preset_flag_w = preset_flag_h * 1.55;

                for preset in ScenarioPreset::all() {
                    let (reg, eco, rel) = preset_flag_spec(*preset);
                    card_frame().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            draw_nation_flag_from_spec(
                                ui,
                                reg,
                                eco,
                                rel,
                                false,
                                false,
                                preset_flag_w,
                                preset_flag_h,
                            );
                            ui.add_space(6.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(preset.title())
                                        .color(TEXT_PRIMARY)
                                        .size(metrics.fs(15.0))
                                        .strong(),
                                );
                                ui.label(
                                    RichText::new(preset.subtitle())
                                        .color(ACCENT_BLUE)
                                        .size(metrics.fs(12.0)),
                                );
                                ui.add_space(2.0);
                                ui.label(
                                    RichText::new(preset.summary())
                                        .color(TEXT_MUTED)
                                        .size(metrics.fs(11.8)),
                                );
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new("▶ ASSUME COMMAND")
                                                .color(ACCENT_GOLD)
                                                .size(metrics.fs(12.5))
                                                .strong(),
                                        )
                                        .min_size(egui::vec2(148.0 * metrics.scale, 32.0 * metrics.scale)),
                                    )
                                    .clicked()
                                {
                                    chosen_preset = Some(*preset);
                                }
                            });
                        });
                    });
                    ui.add_space(6.0);
                }

                if let Some(p) = chosen_preset {
                    self.state = NationState::from_preset(p);
                    self.active_tab = DashboardTab::Overview;
                    self.auto_play = false;
                    self.screen = AppScreen::Dashboard;
                }

                ui.add_space(12.0);
                hero_card_frame(ACCENT_GREEN).show(ui, |ui| {
                    ui.label(
                        RichText::new("🏛 FORGE A CUSTOM SOVEREIGN NATION")
                            .color(ACCENT_GREEN)
                            .size(metrics.fs(14.5))
                            .strong(),
                    );
                    ui.add_space(8.0);

                    let preview_h = (metrics.flag_h * 2.1).clamp(48.0, 88.0);
                    let preview_w = preview_h * 1.55;

                    ui.horizontal_wrapped(|ui| {
                        // Live Custom Flag Preview Card
                        panel_frame().show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new("NATIONAL BANNER PREVIEW")
                                        .color(TEXT_MUTED)
                                        .size(metrics.fs(10.5))
                                        .strong(),
                                );
                                ui.add_space(6.0);
                                draw_nation_flag_from_spec(
                                    ui,
                                    self.custom_regime,
                                    self.custom_economy,
                                    self.custom_religion,
                                    false,
                                    false,
                                    preview_w,
                                    preview_h,
                                );
                                ui.add_space(6.0);
                                ui.label(
                                    RichText::new(&self.custom_name)
                                        .color(TEXT_PRIMARY)
                                        .size(metrics.fs(13.0))
                                        .strong(),
                                );
                                ui.label(
                                    RichText::new(format!(
                                        "{} {}",
                                        self.custom_title, self.custom_leader
                                    ))
                                    .color(ACCENT_GOLD)
                                    .size(metrics.fs(11.0)),
                                );
                            });
                        });

                        ui.add_space(12.0);

                        ui.vertical(|ui| {
                            egui::Grid::new("custom_nation_grid")
                                .num_columns(2)
                                .spacing([14.0, 8.0])
                                .show(ui, |ui| {
                                    ui.label("Nation Name:");
                                    ui.text_edit_singleline(&mut self.custom_name);
                                    ui.end_row();

                                    ui.label("Leader Name:");
                                    ui.text_edit_singleline(&mut self.custom_leader);
                                    ui.end_row();

                                    ui.label("Executive Title:");
                                    ui.text_edit_singleline(&mut self.custom_title);
                                    ui.end_row();

                                    ui.label("Starting Regime:");
                                    egui::ComboBox::from_id_salt("custom_regime_cb")
                                        .selected_text(self.custom_regime.label())
                                        .show_ui(ui, |ui| {
                                            for &r in RegimeType::all() {
                                                ui.selectable_value(
                                                    &mut self.custom_regime,
                                                    r,
                                                    r.label(),
                                                );
                                            }
                                        });
                                    ui.end_row();

                                    ui.label("Economic System:");
                                    egui::ComboBox::from_id_salt("custom_econ_cb")
                                        .selected_text(self.custom_economy.label())
                                        .show_ui(ui, |ui| {
                                            for &e in EconomicSystem::all() {
                                                ui.selectable_value(
                                                    &mut self.custom_economy,
                                                    e,
                                                    e.label(),
                                                );
                                            }
                                        });
                                    ui.end_row();

                                    ui.label("Religion Policy:");
                                    egui::ComboBox::from_id_salt("custom_rel_cb")
                                        .selected_text(self.custom_religion.label())
                                        .show_ui(ui, |ui| {
                                            for &rp in ReligionPolicy::all() {
                                                ui.selectable_value(
                                                    &mut self.custom_religion,
                                                    rp,
                                                    rp.label(),
                                                );
                                            }
                                        });
                                    ui.end_row();
                                });

                            ui.add_space(10.0);
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("🚀 LAUNCH CUSTOM CAMPAIGN")
                                            .color(ACCENT_GREEN)
                                            .size(metrics.fs(13.0))
                                            .strong(),
                                    )
                                    .min_size(egui::vec2(210.0 * metrics.scale, 34.0 * metrics.scale)),
                                )
                                .clicked()
                            {
                                self.state = NationState::custom(
                                    self.custom_name.clone(),
                                    self.custom_leader.clone(),
                                    self.custom_title.clone(),
                                    self.custom_regime,
                                    self.custom_economy,
                                    self.custom_religion,
                                );
                                self.active_tab = DashboardTab::Overview;
                                self.auto_play = false;
                                self.screen = AppScreen::Dashboard;
                            }
                        });
                    });
                });
            });
        });
    }

    fn render_dashboard(&mut self, ctx: &egui::Context, metrics: &UiMetrics) {
        if ctx.input_mut(|i| {
            i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
                egui::Key::M,
            ))
        }) {
            rebalance_sovereign_ledger(&mut self.state);
            self.set_toast("Sovereign Golden Mandate active: debt cleared, +$1.00T surplus, 0% taxes unlocked & Golden Era incidents enabled!");
        }

        let (power_score, rank_label, rank_col) = compute_sovereign_power_score(&self.state);

        // TOP COMMAND BAR (Game HUD Header)
        egui::TopBottomPanel::top("top_command_bar")
            .frame(
                egui::Frame::new()
                    .fill(BG_PANEL)
                    .stroke(Stroke::new(
                        1.2_f32,
                        if self.state.flags.ledger_rebalance || self.state.is_global_hegemon() {
                            BORDER_GOLD
                        } else {
                            BORDER_SUBTLE
                        },
                    ))
                    .inner_margin(Margin::symmetric(12, 8)),
            )
            .show(ctx, |ui| {
                // Row 1: Flag, Nation Identity, Badges & Turn Speed Controls
                ui.horizontal_wrapped(|ui| {
                    let flag_resp = draw_nation_flag_from_spec(
                        ui,
                        self.state.regime,
                        self.state.economy,
                        self.state.religion_policy,
                        self.state.flags.ledger_rebalance,
                        self.state.is_global_hegemon(),
                        metrics.flag_w,
                        metrics.flag_h,
                    );
                    if flag_resp.clicked() {
                        self.trigger_easter_egg_tap();
                    }

                    let title_resp = ui.add(
                        egui::Label::new(
                            RichText::new(&self.state.nation_name)
                                .color(if self.state.flags.ledger_rebalance {
                                    ACCENT_GOLD
                                } else {
                                    TEXT_PRIMARY
                                })
                                .size(metrics.fs(17.5))
                                .strong(),
                        )
                        .sense(egui::Sense::click()),
                    );
                    if title_resp.clicked() {
                        self.trigger_easter_egg_tap();
                    }

                    badge_scaled(ui, &format!("👑 {}", rank_label), rank_col, metrics.fs(10.5));
                    badge_scaled(ui, self.state.regime.short_badge(), ACCENT_PURPLE, metrics.fs(10.5));
                    badge_scaled(ui, self.state.economy.short_badge(), ACCENT_BLUE, metrics.fs(10.5));
                    badge_scaled(ui, &format!("📅 {}", self.state.date_label()), ACCENT_GREEN, metrics.fs(10.5));
                    badge_scaled(
                        ui,
                        &format!("⚡ PC: {:.0}", self.state.political_capital),
                        ACCENT_AMBER,
                        metrics.fs(10.5),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Menu").clicked() {
                            self.auto_play = false;
                            self.screen = AppScreen::Setup;
                        }
                        if ui.button("Load").clicked() {
                            self.load_game();
                        }
                        if ui.button("Save").clicked() {
                            self.save_game();
                        }
                        let zoom_lbl = format!("🔍 {:.0}%", self.ui_zoom * 100.0);
                        if ui
                            .button(zoom_lbl)
                            .on_hover_text("Cycle UI Zoom Scale (90% / 100% / 115% / 125%)")
                            .clicked()
                        {
                            self.ui_zoom = if self.ui_zoom < 0.95 {
                                1.0
                            } else if self.ui_zoom < 1.10 {
                                1.15
                            } else if self.ui_zoom < 1.20 {
                                1.25
                            } else {
                                0.90
                            };
                        }

                        ui.separator();

                        let can_advance =
                            self.state.active_event.is_none() && self.state.game_over.is_none();

                        let spd_lbl = format!("{}x", self.sim_speed);
                        if ui
                            .add_enabled(can_advance, egui::Button::new(spd_lbl))
                            .on_hover_text("Auto-Play Simulation Speed (1x / 2x / 4x)")
                            .clicked()
                        {
                            self.sim_speed = match self.sim_speed {
                                1 => 2,
                                2 => 4,
                                _ => 1,
                            };
                        }

                        let play_lbl = if self.auto_play { "⏸ Pause" } else { "▶ Auto" };
                        if ui.add_enabled(can_advance, egui::Button::new(play_lbl)).clicked() {
                            self.auto_play = !self.auto_play;
                            self.last_tick = Instant::now();
                        }

                        let next_btn = egui::Button::new(
                            RichText::new("NEXT TURN ▶▶")
                                .color(if self.state.flags.ledger_rebalance {
                                    ACCENT_GOLD
                                } else {
                                    ACCENT_GREEN
                                })
                                .size(metrics.fs(12.5))
                                .strong(),
                        )
                        .fill(Color32::from_rgb(20, 48, 42))
                        .stroke(Stroke::new(1.2_f32, ACCENT_GREEN));
                        if ui.add_enabled(can_advance, next_btn).clicked() {
                            advance_quarter(&mut self.state);
                        }
                    });
                });

                ui.add_space(6.0);

                // Row 2: Responsive Live KPI Strip (adapts width & columns to window size!)
                let avail_w = ui.available_width().max(400.0);
                let cols = if avail_w >= 960.0 {
                    6
                } else if avail_w >= 560.0 {
                    3
                } else {
                    2
                };
                let card_w = ((avail_w - (cols as f32 - 1.0) * 8.0) / cols as f32 - 22.0).max(110.0);

                ui.horizontal_wrapped(|ui| {
                    kpi_card(
                        ui,
                        "💰 GDP",
                        &fmt_billions(self.state.gdp_bn),
                        &format!("{:+.2}% / yr", self.state.gdp_growth),
                        if self.state.gdp_growth >= 0.0 { ACCENT_GREEN } else { ACCENT_RED },
                        card_w,
                    );
                    kpi_card(
                        ui,
                        "🏦 TREASURY",
                        &fmt_billions(self.state.treasury_bn),
                        &format!("Net {}/qtr", fmt_signed_billions(self.state.last_fiscal.net_balance)),
                        if self.state.last_fiscal.net_balance >= 0.0 { ACCENT_GREEN } else { ACCENT_AMBER },
                        card_w,
                    );
                    kpi_card(
                        ui,
                        "⭐ APPROVAL",
                        &format!("{:.1}%", self.state.approval),
                        &format!("Legitimacy {:.0}%", self.state.legitimacy),
                        severity_color(self.state.approval, true),
                        card_w,
                    );
                    kpi_card(
                        ui,
                        "🏛 STABILITY",
                        &format!("{:.1}%", self.state.stability),
                        &format!("Democracy {:.0}%", self.state.democracy_index),
                        severity_color(self.state.stability, true),
                        card_w,
                    );
                    kpi_card(
                        ui,
                        "⚔ COUP RISK",
                        &format!("{:.0}%", self.state.coup_risk),
                        &format!("Mil Loyalty {:.0}%", self.state.faction(FactionId::Military).approval),
                        severity_color(self.state.coup_risk, false),
                        card_w,
                    );
                    kpi_card(
                        ui,
                        "🔥 EXTREMISM",
                        &format!("{:.0}%", self.state.total_extremism()),
                        &format!("Insurgency {:.0}%", self.state.insurgency_strength),
                        severity_color(self.state.total_extremism(), false),
                        card_w,
                    );
                });
            });

        // BOTTOM LIVE WORLD NEWS TICKER BAR
        egui::TopBottomPanel::bottom("bottom_ticker_bar")
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(13, 20, 32))
                    .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                    .inner_margin(Margin::symmetric(12, 5)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    badge_scaled(
                        ui,
                        if self.state.active_wars_count() > 0 {
                            "⚔ WARTIME WIRE"
                        } else if self.state.flags.ledger_rebalance {
                            "⭐ GOLDEN WIRE"
                        } else {
                            "📡 WORLD WIRE"
                        },
                        if self.state.active_wars_count() > 0 {
                            ACCENT_RED
                        } else if self.state.flags.ledger_rebalance {
                            ACCENT_GOLD
                        } else {
                            ACCENT_CYAN
                        },
                        metrics.fs(10.0),
                    );
                    if let Some(latest) = self.state.logs.first() {
                        ui.label(
                            RichText::new(format!(
                                "[{}] {} — {}",
                                latest.date_label, latest.headline, latest.detail
                            ))
                            .color(TEXT_PRIMARY)
                            .size(metrics.fs(11.0)),
                        );
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!(
                                "Turn {}  •  Power {:.0}  •  Pop {:.1}M",
                                self.state.turn, power_score, self.state.population_m
                            ))
                            .color(TEXT_MUTED)
                            .size(metrics.fs(10.5)),
                        );
                    });
                });
            });

        // LEFT NAVIGATION SIDEBAR (Responsive width & scrollable flags!)
        egui::SidePanel::left("nav_sidebar")
            .exact_width(metrics.sidebar_w)
            .frame(
                egui::Frame::new()
                    .fill(BG_PANEL)
                    .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                    .inner_margin(Margin::same(10)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(
                        RichText::new("COMMAND ROOMS")
                            .color(ACCENT_GOLD)
                            .size(metrics.fs(10.8))
                            .strong(),
                    );
                    ui.add_space(6.0);

                    for &tab in DashboardTab::all() {
                        let selected = self.active_tab == tab;
                        let text = RichText::new(tab.label())
                            .size(metrics.fs(12.8))
                            .color(if selected { TEXT_PRIMARY } else { TEXT_MUTED })
                            .strong();
                        let btn = egui::Button::new(text)
                            .min_size(egui::vec2(ui.available_width(), (33.0 * metrics.scale).clamp(28.0, 42.0)))
                            .fill(if selected {
                                Color32::from_rgb(28, 56, 94)
                            } else {
                                Color32::TRANSPARENT
                            })
                            .stroke(if selected {
                                Stroke::new(1.2_f32, ACCENT_BLUE)
                            } else {
                                Stroke::NONE
                            });
                        if ui.add(btn).clicked() {
                            self.active_tab = tab;
                        }
                        ui.add_space(3.0);
                    }

                    ui.add_space(10.0);
                    // Sovereign Empire / Conquest Progress Mini-Card
                    panel_frame().show(ui, |ui| {
                        ui.label(
                            RichText::new("👑 SOVEREIGN RANK")
                                .color(TEXT_MUTED)
                                .size(metrics.fs(10.0))
                                .strong(),
                        );
                        ui.label(
                            RichText::new(rank_label)
                                .color(rank_col)
                                .size(metrics.fs(13.0))
                                .strong(),
                        );
                        ui.label(
                            RichText::new(format!("Power Rating: {:.0}", power_score))
                                .color(TEXT_MUTED)
                                .size(metrics.fs(10.8)),
                        );
                        ui.add_space(4.0);
                        let subdued = self.state.annexed_count() + self.state.puppet_count();
                        let dom_pct = (subdued as f64 / 5.0) * 100.0;
                        meter_bar(ui, &format!("Dominion ({}/5)", subdued), dom_pct, true);
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new("🚩 ACTIVE STATE FLAGS")
                            .color(ACCENT_CYAN)
                            .size(metrics.fs(10.8))
                            .strong(),
                    );
                    ui.add_space(5.0);

                    let mut flag_items: Vec<(&str, String, &str, Color32)> = Vec::new();
                    if self.state.flags.ledger_rebalance {
                        flag_items.push((
                            "⭐",
                            "GOLDEN MANDATE".into(),
                            "Debt cleared, +$1.00T/qtr surplus, 0% tax ceiling unlocked, coup immunity & Golden Era incidents active.",
                            ACCENT_GOLD,
                        ));
                    }
                    if self.state.is_global_hegemon() {
                        flag_items.push((
                            "👑",
                            "GLOBAL HEGEMON".into(),
                            "All 5 rival sovereign nations have been annexed or vassalized!",
                            ACCENT_PURPLE,
                        ));
                    }
                    let annexed_n = self.state.annexed_count();
                    if annexed_n > 0 {
                        flag_items.push((
                            "⚔",
                            format!("ANNEXED: {} NATION(S)", annexed_n),
                            "Conquered foreign powers permanently integrated into your territory.",
                            ACCENT_GREEN,
                        ));
                    }
                    let puppet_n = self.state.puppet_count();
                    if puppet_n > 0 {
                        flag_items.push((
                            "🛡",
                            format!("CLIENTS: {} STATE(S)", puppet_n),
                            "Compliant client regimes paying quarterly tribute and allied military support.",
                            ACCENT_PURPLE,
                        ));
                    }
                    if self.state.taxes.income_tax <= 0.01
                        && self.state.taxes.corporate_tax <= 0.01
                        && self.state.taxes.tariffs <= 0.01
                        && self.state.taxes.wealth_tax <= 0.01
                    {
                        flag_items.push((
                            "💎",
                            "ZERO-TAX PARADISE".into(),
                            "All national taxes at 0%! Boosting growth & citizen approval.",
                            ACCENT_CYAN,
                        ));
                    }
                    if self.state.flags.martial_law {
                        flag_items.push((
                            "🚨",
                            "MARTIAL LAW".into(),
                            "Military policing curbs insurgency but reduces civil liberties & rule of law.",
                            ACCENT_RED,
                        ));
                    }
                    if self.state.flags.press_censorship {
                        flag_items.push((
                            "🔇",
                            "PRESS CENSORSHIP".into(),
                            "State controls narratives; lowers press freedom & angers Intelligentsia.",
                            ACCENT_AMBER,
                        ));
                    }
                    if self.state.flags.secret_police {
                        flag_items.push((
                            "🕵",
                            "SECRET POLICE".into(),
                            "Deters coup attempts & bolsters stability at the cost of liberties.",
                            ACCENT_AMBER,
                        ));
                    }
                    if self.state.flags.mass_surveillance {
                        flag_items.push((
                            "📡",
                            "SURVEILLANCE GRID".into(),
                            "Suppresses insurgency & coup plots (-$1.4B/qtr, reduces liberties).",
                            ACCENT_AMBER,
                        ));
                    }
                    if self.state.flags.religious_courts {
                        flag_items.push((
                            "⚖",
                            "CLERICAL COURTS".into(),
                            "Empowers Clergy and raises clerical institutional power.",
                            ACCENT_PURPLE,
                        ));
                    }
                    if self.state.flags.five_year_plan {
                        flag_items.push((
                            "🏭",
                            "5-YEAR PLAN".into(),
                            "Boosts state industrial output and GDP growth.",
                            ACCENT_BLUE,
                        ));
                    }
                    if self.state.flags.universal_healthcare {
                        flag_items.push((
                            "🏥",
                            "UNIV. HEALTHCARE".into(),
                            "Guarantees universal care (+Healthcare & Labor approval).",
                            ACCENT_GREEN,
                        ));
                    }
                    if self.state.flags.sovereign_wealth_fund {
                        flag_items.push((
                            "🏦",
                            "SOVEREIGN FUND".into(),
                            "Compounds permanent quarterly passive investment dividends.",
                            ACCENT_GREEN,
                        ));
                    }
                    if self.state.flags.independent_judiciary {
                        flag_items.push((
                            "🏛",
                            "INDEP. COURTS".into(),
                            "Strengthens Rule of Law and lowers corruption.",
                            ACCENT_GREEN,
                        ));
                    }

                    if flag_items.is_empty() {
                        ui.label(
                            RichText::new("No active state flags")
                                .color(TEXT_MUTED)
                                .size(metrics.fs(11.0)),
                        );
                    } else {
                        for (icon, lbl, tip, col) in flag_items {
                            draw_status_flag_chip(ui, icon, &lbl, tip, col, metrics.scale);
                            ui.add_space(4.0);
                        }
                    }

                    if let Some((msg, _)) = &self.status_toast {
                        ui.add_space(10.0);
                        egui::Frame::new()
                            .fill(Color32::from_rgb(20, 38, 60))
                            .stroke(Stroke::new(1.2_f32, ACCENT_CYAN))
                            .corner_radius(CornerRadius::same(7))
                            .inner_margin(Margin::same(9))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(format!("⚡ {}", msg))
                                        .color(TEXT_PRIMARY)
                                        .size(metrics.fs(11.2)),
                                );
                            });
                    }
                });
            });

        // MAIN WORKSPACE PANEL
        let mut toast_out: Option<String> = None;
        let mut jump_to_military = false;
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.active_tab {
                DashboardTab::Overview => {
                    if render_overview_tab(ui, &self.state, metrics).is_some() {
                        jump_to_military = true;
                    }
                }
                DashboardTab::Economy => render_economy_tab(ui, &mut self.state, &mut toast_out, metrics),
                DashboardTab::Politics => render_politics_tab(ui, &mut self.state, &mut toast_out, metrics),
                DashboardTab::ReligionExtremism => {
                    render_religion_extremism_tab(ui, &mut self.state, &mut toast_out, metrics)
                }
                DashboardTab::MilitaryDiplomacy => {
                    render_military_diplomacy_tab(ui, &mut self.state, &mut toast_out, metrics)
                }
                DashboardTab::Decisions => render_decisions_tab(ui, &mut self.state, &mut toast_out, metrics),
                DashboardTab::Chronicles => {
                    render_chronicles_tab(ui, &self.state, &mut self.log_filter, metrics)
                }
            });
        });

        if jump_to_military {
            self.active_tab = DashboardTab::MilitaryDiplomacy;
        }
        if let Some(m) = toast_out {
            self.set_toast(m);
        }

        // INTERACTIVE CRISIS / VICTORY / GOLDEN ERA MODAL (Responsive & Scrollable)
        if let Some(ev) = self.state.active_event.clone() {
            self.auto_play = false;
            let modal_w = (metrics.screen_w * 0.56).clamp(430.0, 760.0);
            let is_positive = ev.id.starts_with("god_") || ev.id == "post_war_victory";
            let win_title = if is_positive {
                format!("⭐ SOVEREIGN DISPATCH // {}", ev.urgency)
            } else {
                format!("🚨 CLASSIFIED BRIEFING // {}", ev.urgency)
            };

            egui::Window::new(win_title)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .default_width(modal_w)
                .max_width(modal_w)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(metrics.screen_h * 0.72)
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                let ev_badge_col = if is_positive { ACCENT_GOLD } else { ACCENT_RED };
                                badge_scaled(ui, ev.category.label(), ev_badge_col, metrics.fs(11.0));
                                ui.label(
                                    RichText::new(&ev.title)
                                        .color(if is_positive { ACCENT_GOLD } else { TEXT_PRIMARY })
                                        .size(metrics.fs(16.0))
                                        .strong(),
                                );
                            });
                            ui.add_space(6.0);
                            ui.label(RichText::new(&ev.briefing).color(TEXT_PRIMARY).size(metrics.fs(13.0)));
                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(6.0);

                            let mut selected_choice: Option<String> = None;
                            for choice in &ev.choices {
                                card_frame().show(ui, |ui| {
                                    ui.label(
                                        RichText::new(&choice.label)
                                            .color(if is_positive { ACCENT_GREEN } else { ACCENT_BLUE })
                                            .size(metrics.fs(13.5))
                                            .strong(),
                                    );
                                    ui.label(
                                        RichText::new(&choice.description)
                                            .color(TEXT_PRIMARY)
                                            .size(metrics.fs(12.0)),
                                    );
                                    ui.label(
                                        RichText::new(format!("Projected Impact: {}", choice.impact_hint))
                                            .color(TEXT_MUTED)
                                            .size(metrics.fs(11.5)),
                                    );
                                    ui.add_space(5.0);
                                    let btn = egui::Button::new(
                                        RichText::new("▶ EXECUTE DIRECTIVE")
                                            .color(if is_positive { ACCENT_GOLD } else { TEXT_PRIMARY })
                                            .strong(),
                                    );
                                    if ui.add(btn).clicked() {
                                        selected_choice = Some(choice.id.clone());
                                    }
                                });
                                ui.add_space(6.0);
                            }

                            if let Some(cid) = selected_choice {
                                resolve_event_choice(&mut self.state, &cid);
                            }
                        });
                });
        }

        // GAME OVER / HISTORICAL LEGACY MODAL
        if let Some(reason) = self.state.game_over.clone() {
            self.auto_play = false;
            let modal_w = (metrics.screen_w * 0.48).clamp(400.0, 620.0);
            egui::Window::new("📜 HISTORICAL EPILOGUE // REGIME TERMINATED")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .default_width(modal_w)
                .max_width(modal_w)
                .show(ctx, |ui| {
                    ui.label(RichText::new(reason.title()).color(ACCENT_RED).size(metrics.fs(18.0)).strong());
                    ui.add_space(6.0);
                    ui.label(RichText::new(reason.message()).color(TEXT_PRIMARY).size(metrics.fs(13.0)));
                    ui.add_space(10.0);
                    ui.separator();
                    ui.label(format!(
                        "Tenure: {} Quarter(s) (Ended {})  |  Final GDP: {}  |  Annexed: {}  |  Regime: {}",
                        self.state.turn,
                        self.state.date_label(),
                        fmt_billions(self.state.gdp_bn),
                        self.state.annexed_count(),
                        self.state.regime.label()
                    ));
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.button(RichText::new("Start New Campaign").strong()).clicked() {
                            self.screen = AppScreen::Setup;
                        }
                        if Self::save_path().exists() && ui.button("Reload Last Save").clicked() {
                            self.load_game();
                        }
                    });
                });
        }
    }
}
