pub mod cinematic;
pub mod tabs;
pub mod widgets;

use crate::events::resolve_event_choice;
use crate::models::*;
use crate::simulation::{advance_quarter, rebalance_sovereign_ledger, run_strategic_activity, set_lockdown_level};
use crate::ui::cinematic::*;
use crate::ui::tabs::*;
use crate::ui::widgets::*;
use eframe::egui::{self, Color32, CornerRadius, Margin, RichText, Stroke};
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppScreen {
    Setup,
    Cinematic,
    Dashboard,
}

pub struct NationSimApp {
    screen: AppScreen,
    state: NationState,
    cinematic: Option<CinematicState>,
    active_tab: DashboardTab,
    log_filter: Option<LogCategory>,
    status_toast: Option<(String, Instant)>,

    // Auto-play, theme & scaling controls
    auto_play: bool,
    sim_speed: u8, // 1, 2, or 4
    ui_zoom: f32,  // 0.9, 1.0, 1.15, 1.25
    dark_mode: bool,
    last_tick: Instant,
    header_taps: u8,
    last_tap: Instant,

    // Personal Leader Dossier (Applies to BOTH Presets & Custom Nations!)
    use_custom_leader_for_presets: bool,
    custom_name: String,
    custom_leader: String,
    custom_title: String,
    custom_age: u8,
    custom_education: String,
    custom_background: LeaderBackground,
    custom_motto: String,
    custom_bio: String,
    custom_regime: RegimeType,
    custom_economy: EconomicSystem,
    custom_religion: ReligionPolicy,
}

impl NationSimApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        apply_theme(&cc.egui_ctx, true);
        Self {
            screen: AppScreen::Setup,
            state: NationState::from_preset(ScenarioPreset::AureliaDemocracy),
            cinematic: None,
            active_tab: DashboardTab::Overview,
            log_filter: None,
            status_toast: None,
            auto_play: false,
            sim_speed: 1,
            ui_zoom: 1.0,
            dark_mode: true,
            last_tick: Instant::now(),
            header_taps: 0,
            last_tap: Instant::now(),
            use_custom_leader_for_presets: true,
            custom_name: "Republic of Novaria".into(),
            custom_leader: "Soren Lindqvist".into(),
            custom_title: "President".into(),
            custom_age: 38,
            custom_education: "Ph.D. in Political Economy & Strategic Studies".into(),
            custom_background: LeaderBackground::Statesman,
            custom_motto: "Unity, Vision, and Sovereign Destiny".into(),
            custom_bio: "A bold reformer and strategist who stepped onto the world stage to forge an enduring superpower.".into(),
            custom_regime: RegimeType::LiberalDemocracy,
            custom_economy: EconomicSystem::MixedEconomy,
            custom_religion: ReligionPolicy::SecularPluralism,
        }
    }

    fn build_leader_profile(&self) -> LeaderProfile {
        LeaderProfile {
            age: self.custom_age,
            education: if self.custom_education.trim().is_empty() {
                "National Academy of Statecraft".into()
            } else {
                self.custom_education.trim().to_string()
            },
            background: self.custom_background,
            bio: if self.custom_bio.trim().is_empty() {
                "A visionary leader ready to guide the nation into history.".into()
            } else {
                self.custom_bio.trim().to_string()
            },
            motto: if self.custom_motto.trim().is_empty() {
                "Strength and Sovereignty".into()
            } else {
                self.custom_motto.trim().to_string()
            },
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
        if self.state.flags.ledger_rebalance {
            rebalance_sovereign_ledger(&mut self.state);
            self.set_toast(format!(
                "Golden Mandate escalated (50x): {}/qtr surplus added!",
                fmt_signed_billions(self.state.god_surplus_bn)
            ));
            return;
        }
        if self.last_tap.elapsed() > Duration::from_secs(3) {
            self.header_taps = 0;
        }
        self.header_taps += 1;
        self.last_tap = Instant::now();
        if self.header_taps >= 5 {
            self.header_taps = 0;
            rebalance_sovereign_ledger(&mut self.state);
            self.set_toast(format!(
                "Sovereign Golden Mandate active: debt cleared, {}/qtr surplus! Click nation name again for 50x boost.",
                fmt_signed_billions(self.state.god_surplus_bn)
            ));
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
                Ok(mut loaded) => {
                    loaded.ensure_expanded_world();
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

fn apply_theme(ctx: &egui::Context, dark: bool) {
    set_dark_mode(dark);
    let mut visuals = if dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    visuals.window_fill = bg_panel();
    visuals.panel_fill = bg_canvas();
    visuals.faint_bg_color = bg_card();
    visuals.extreme_bg_color = bg_inset();
    visuals.window_corner_radius = CornerRadius::same(10);
    visuals.widgets.noninteractive.bg_fill = bg_card();
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, text_primary());
    if dark {
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(30, 42, 62);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(52, 70, 98));
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(42, 60, 88);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.2_f32, accent_blue());
        visuals.widgets.active.bg_fill = Color32::from_rgb(37, 99, 235);
        visuals.selection.bg_fill = Color32::from_rgb(30, 64, 118);
    } else {
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(236, 231, 218);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(184, 174, 152));
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(225, 218, 200);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.2_f32, accent_blue());
        visuals.widgets.active.bg_fill = Color32::from_rgb(20, 102, 184);
        visuals.selection.bg_fill = Color32::from_rgb(198, 222, 246);
    }
    visuals.widgets.inactive.corner_radius = CornerRadius::same(6);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(6);
    visuals.widgets.active.corner_radius = CornerRadius::same(6);
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

        if let Some((_, ts)) = &self.status_toast {
            if ts.elapsed() > Duration::from_secs(5) {
                self.status_toast = None;
            }
        }

        let metrics = UiMetrics::from_screen(ctx.screen_rect());

        match self.screen {
            AppScreen::Setup => self.render_setup_screen(ctx, &metrics),
            AppScreen::Cinematic => {
                if let Some(cin) = &self.cinematic {
                    match render_cinematic_screen(ctx, &self.state, cin, &metrics) {
                        CinematicAction::Continue => {}
                        CinematicAction::FinishToDashboard => {
                            self.cinematic = None;
                            self.screen = AppScreen::Dashboard;
                        }
                        CinematicAction::Replay => {
                            if let Some(c) = &mut self.cinematic {
                                c.restart();
                            }
                        }
                    }
                } else {
                    self.screen = AppScreen::Dashboard;
                }
            }
            AppScreen::Dashboard => self.render_dashboard(ctx, &metrics),
        }
    }
}

impl NationSimApp {
    fn render_setup_screen(&mut self, ctx: &egui::Context, metrics: &UiMetrics) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(12.0);
                hero_card_frame(border_gold()).show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new("👑 SOVEREIGN // GRAND STRATEGY NATION SIMULATOR")
                                    .color(accent_gold())
                                    .size(metrics.fs(22.0))
                                    .strong(),
                            );
                            ui.label(
                                RichText::new(
                                    "18 Fictional World Nations • Combined Arms (Army/Air/Navy) • Weapons Trade • Pandemics • Conquests & Splits",
                                )
                                .color(text_muted())
                                .size(metrics.fs(12.5)),
                            );
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let theme_lbl = if self.dark_mode { "☀ Light" } else { "🌙 Dark" };
                            if ui.button(theme_lbl).clicked() {
                                self.dark_mode = !self.dark_mode;
                                apply_theme(ctx, self.dark_mode);
                            }
                            if Self::save_path().exists()
                                && ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new("📂 CONTINUE CAMPAIGN")
                                                .color(accent_green())
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

                ui.add_space(12.0);
                // PERSONAL SOVEREIGN LEADER DOSSIER (Applies to BOTH Presets & Custom Nations!)
                hero_card_frame(accent_cyan()).show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new("👤 YOUR SOVEREIGN LEADER DOSSIER (NAME, AGE, EDUCATION & BIO)")
                                .color(accent_cyan())
                                .size(metrics.fs(14.0))
                                .strong(),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.checkbox(
                                &mut self.use_custom_leader_for_presets,
                                RichText::new("Apply My Leader Identity to Preset Nations Too")
                                    .color(accent_gold())
                                    .strong(),
                            );
                        });
                    });
                    ui.add_space(6.0);

                    if ui.available_width() >= 760.0 {
                        ui.columns(2, |cols| {
                            egui::Grid::new("leader_dossier_left")
                                .num_columns(2)
                                .spacing([12.0, 7.0])
                                .show(&mut cols[0], |ui| {
                                    ui.label("Your Name:");
                                    ui.text_edit_singleline(&mut self.custom_leader);
                                    ui.end_row();

                                    ui.label("Executive Title:");
                                    ui.text_edit_singleline(&mut self.custom_title);
                                    ui.end_row();

                                    ui.label("Age:");
                                    ui.add(egui::Slider::new(&mut self.custom_age, 22..=82).text("yrs"));
                                    ui.end_row();

                                    ui.label("Education:");
                                    ui.text_edit_singleline(&mut self.custom_education);
                                    ui.end_row();
                                });

                            egui::Grid::new("leader_dossier_right")
                                .num_columns(2)
                                .spacing([12.0, 7.0])
                                .show(&mut cols[1], |ui| {
                                    ui.label("Background Trait:");
                                    egui::ComboBox::from_id_salt("leader_bg_cb")
                                        .selected_text(self.custom_background.label())
                                        .show_ui(ui, |ui| {
                                            for &bg in LeaderBackground::all() {
                                                ui.selectable_value(&mut self.custom_background, bg, bg.label());
                                            }
                                        });
                                    ui.end_row();

                                    ui.label("Starting Perk:");
                                    ui.label(
                                        RichText::new(self.custom_background.perk_summary())
                                            .color(accent_green())
                                            .size(11.5),
                                    );
                                    ui.end_row();

                                    ui.label("Personal Motto:");
                                    ui.text_edit_singleline(&mut self.custom_motto);
                                    ui.end_row();

                                    ui.label("Leader Biography:");
                                    ui.add(
                                        egui::TextEdit::multiline(&mut self.custom_bio)
                                            .desired_rows(2)
                                            .desired_width(f32::INFINITY),
                                    );
                                    ui.end_row();
                                });
                        });
                    } else {
                        egui::Grid::new("leader_dossier_stacked")
                            .num_columns(2)
                            .spacing([12.0, 7.0])
                            .show(ui, |ui| {
                                ui.label("Your Name:");
                                ui.text_edit_singleline(&mut self.custom_leader);
                                ui.end_row();
                                ui.label("Executive Title:");
                                ui.text_edit_singleline(&mut self.custom_title);
                                ui.end_row();
                                ui.label("Age:");
                                ui.add(egui::Slider::new(&mut self.custom_age, 22..=82).text("yrs"));
                                ui.end_row();
                                ui.label("Education:");
                                ui.text_edit_singleline(&mut self.custom_education);
                                ui.end_row();
                                ui.label("Background Trait:");
                                egui::ComboBox::from_id_salt("leader_bg_cb_s")
                                    .selected_text(self.custom_background.label())
                                    .show_ui(ui, |ui| {
                                        for &bg in LeaderBackground::all() {
                                            ui.selectable_value(&mut self.custom_background, bg, bg.label());
                                        }
                                    });
                                ui.end_row();
                                ui.label("Personal Motto:");
                                ui.text_edit_singleline(&mut self.custom_motto);
                                ui.end_row();
                                ui.label("Biography:");
                                ui.add(egui::TextEdit::multiline(&mut self.custom_bio).desired_rows(2));
                                ui.end_row();
                            });
                    }
                });

                ui.add_space(14.0);
                ui.label(
                    RichText::new("⚔ SELECT A PRESET CAMPAIGN SCENARIO")
                        .color(accent_cyan())
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
                                        .color(text_primary())
                                        .size(metrics.fs(15.0))
                                        .strong(),
                                );
                                ui.label(
                                    RichText::new(preset.subtitle())
                                        .color(accent_blue())
                                        .size(metrics.fs(12.0)),
                                );
                                ui.add_space(2.0);
                                ui.label(
                                    RichText::new(preset.summary())
                                        .color(text_muted())
                                        .size(metrics.fs(11.8)),
                                );
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new("▶ ASSUME COMMAND")
                                                .color(accent_gold())
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
                    if self.use_custom_leader_for_presets {
                        let profile = self.build_leader_profile();
                        self.state.apply_leader_profile(&self.custom_leader, &self.custom_title, profile);
                    }
                    self.active_tab = DashboardTab::Overview;
                    self.auto_play = false;
                    let kind = CinematicKind::for_preset(&self.state);
                    self.cinematic = Some(CinematicState::new(kind));
                    self.screen = AppScreen::Cinematic;
                }

                ui.add_space(12.0);
                hero_card_frame(accent_green()).show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new("🏛 FORGE A CUSTOM SOVEREIGN NATION ON UNCLAIMED LAND")
                                .color(accent_green())
                                .size(metrics.fs(15.0))
                                .strong(),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            badge_scaled(ui, "🌅 CUSTOM FOUNDING CINEMATIC", accent_gold(), metrics.fs(10.2));
                        });
                    });
                    ui.add_space(8.0);

                    let preview_h = (metrics.flag_h * 2.1).clamp(50.0, 90.0);
                    let preview_w = preview_h * 1.55;
                    let mut launch_custom = false;

                    if ui.available_width() >= 720.0 {
                        ui.columns(2, |cols| {
                            // Left: Custom Nation Form
                            egui::Grid::new("custom_nation_grid")
                                .num_columns(2)
                                .spacing([14.0, 9.0])
                                .show(&mut cols[0], |ui| {
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
                                                ui.selectable_value(&mut self.custom_regime, r, r.label());
                                            }
                                        });
                                    ui.end_row();

                                    ui.label("Economic System:");
                                    egui::ComboBox::from_id_salt("custom_econ_cb")
                                        .selected_text(self.custom_economy.label())
                                        .show_ui(ui, |ui| {
                                            for &e in EconomicSystem::all() {
                                                ui.selectable_value(&mut self.custom_economy, e, e.label());
                                            }
                                        });
                                    ui.end_row();

                                    ui.label("Religion Policy:");
                                    egui::ComboBox::from_id_salt("custom_rel_cb")
                                        .selected_text(self.custom_religion.label())
                                        .show_ui(ui, |ui| {
                                            for &rp in ReligionPolicy::all() {
                                                ui.selectable_value(&mut self.custom_religion, rp, rp.label());
                                            }
                                        });
                                    ui.end_row();
                                });

                            // Right: Live National Banner Preview
                            panel_frame().show(&mut cols[1], |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        RichText::new("NATIONAL BANNER PREVIEW")
                                            .color(text_muted())
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
                                            .color(text_primary())
                                            .size(metrics.fs(14.0))
                                            .strong(),
                                    );
                                    ui.label(
                                        RichText::new(format!("{} {}", self.custom_title, self.custom_leader))
                                            .color(accent_gold())
                                            .size(metrics.fs(11.5)),
                                    );
                                });
                            });
                        });
                    } else {
                        panel_frame().show(ui, |ui| {
                            ui.vertical_centered(|ui| {
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
                                ui.add_space(4.0);
                                ui.label(RichText::new(&self.custom_name).color(text_primary()).size(metrics.fs(13.5)).strong());
                            });
                        });
                        ui.add_space(8.0);
                        egui::Grid::new("custom_nation_grid_narrow")
                            .num_columns(2)
                            .spacing([12.0, 8.0])
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
                                egui::ComboBox::from_id_salt("custom_regime_cb_n")
                                    .selected_text(self.custom_regime.label())
                                    .show_ui(ui, |ui| {
                                        for &r in RegimeType::all() {
                                            ui.selectable_value(&mut self.custom_regime, r, r.label());
                                        }
                                    });
                                ui.end_row();
                                ui.label("Economic System:");
                                egui::ComboBox::from_id_salt("custom_econ_cb_n")
                                    .selected_text(self.custom_economy.label())
                                    .show_ui(ui, |ui| {
                                        for &e in EconomicSystem::all() {
                                            ui.selectable_value(&mut self.custom_economy, e, e.label());
                                        }
                                    });
                                ui.end_row();
                                ui.label("Religion Policy:");
                                egui::ComboBox::from_id_salt("custom_rel_cb_n")
                                    .selected_text(self.custom_religion.label())
                                    .show_ui(ui, |ui| {
                                        for &rp in ReligionPolicy::all() {
                                            ui.selectable_value(&mut self.custom_religion, rp, rp.label());
                                        }
                                    });
                                ui.end_row();
                            });
                    }

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(8.0);
                    let create_fill = if self.dark_mode {
                        Color32::from_rgb(18, 56, 42)
                    } else {
                        Color32::from_rgb(208, 240, 218)
                    };
                    let create_btn = egui::Button::new(
                        RichText::new("🚀 CREATE & FOUND CUSTOM NATION ON UNCLAIMED LAND")
                            .color(accent_green())
                            .size(metrics.fs(14.0))
                            .strong(),
                    )
                    .fill(create_fill)
                    .stroke(Stroke::new(1.6_f32, accent_green()))
                    .min_size(egui::vec2(ui.available_width().min(480.0), 40.0 * metrics.scale));

                    ui.vertical_centered(|ui| {
                        if ui.add(create_btn).clicked() {
                            launch_custom = true;
                        }
                    });

                    if launch_custom {
                        self.state = NationState::custom(
                            self.custom_name.clone(),
                            self.custom_leader.clone(),
                            self.custom_title.clone(),
                            self.custom_regime,
                            self.custom_economy,
                            self.custom_religion,
                        );
                        let profile = self.build_leader_profile();
                        self.state.apply_leader_profile(&self.custom_leader, &self.custom_title, profile);
                        self.active_tab = DashboardTab::Overview;
                        self.auto_play = false;
                        self.cinematic = Some(CinematicState::new(CinematicKind::FoundingNewHomeland));
                        self.screen = AppScreen::Cinematic;
                    }
                });
                ui.add_space(24.0);
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
            self.set_toast(format!(
                "Sovereign Golden Mandate: {}/qtr surplus active (press again or click nation name for 50x)!",
                fmt_signed_billions(self.state.god_surplus_bn)
            ));
        }

        let (power_score, rank_label, rank_col) = compute_sovereign_power_score(&self.state);
        let era_label = compute_era_label(&self.state);

        // TOP COMMAND BAR (Game HUD Header)
        egui::TopBottomPanel::top("top_command_bar")
            .frame(
                egui::Frame::new()
                    .fill(bg_panel())
                    .stroke(Stroke::new(
                        1.2_f32,
                        if self.state.flags.ledger_rebalance || self.state.is_global_hegemon() {
                            border_gold()
                        } else {
                            border_subtle()
                        },
                    ))
                    .inner_margin(Margin::symmetric(12, 8)),
            )
            .show(ctx, |ui| {
                // Row 1: Flag, Nation Identity, Era/Rank & Game Controls
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
                                    accent_gold()
                                } else {
                                    text_primary()
                                })
                                .size(metrics.fs(17.5))
                                .strong(),
                        )
                        .sense(egui::Sense::click()),
                    );
                    if title_resp.clicked() {
                        self.trigger_easter_egg_tap();
                    }

                    badge_scaled(ui, &format!("👑 {}", rank_label), rank_col, metrics.fs(10.2));
                    badge_scaled(ui, &format!("⏳ {}", era_label), accent_cyan(), metrics.fs(10.0));
                    badge_scaled(ui, self.state.regime.short_badge(), accent_purple(), metrics.fs(10.0));
                    badge_scaled(ui, self.state.economy.short_badge(), accent_blue(), metrics.fs(10.0));
                    badge_scaled(ui, &format!("📅 {}", self.state.date_label()), accent_green(), metrics.fs(10.2));
                    let pc_label = if self.state.flags.ledger_rebalance {
                        "⚡ PC ∞".to_string()
                    } else {
                        format!("⚡ PC {:.0}", self.state.political_capital)
                    };
                    badge_scaled(
                        ui,
                        &pc_label,
                        if self.state.flags.ledger_rebalance { accent_gold() } else { accent_amber() },
                        metrics.fs(10.2),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("🎬 Intro").on_hover_text("Replay Intro Cinematic").clicked() {
                            self.auto_play = false;
                            let kind = CinematicKind::for_preset(&self.state);
                            self.cinematic = Some(CinematicState::new(kind));
                            self.screen = AppScreen::Cinematic;
                        }
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
                        let theme_lbl = if self.dark_mode { "☀ Light" } else { "🌙 Dark" };
                        if ui
                            .button(theme_lbl)
                            .on_hover_text("Toggle Light (Parchment Strategy) / Dark (Obsidian War Room) UI Theme")
                            .clicked()
                        {
                            self.dark_mode = !self.dark_mode;
                            apply_theme(ctx, self.dark_mode);
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

                        let next_fill = if self.dark_mode {
                            Color32::from_rgb(20, 52, 44)
                        } else {
                            Color32::from_rgb(212, 240, 222)
                        };
                        let next_btn = egui::Button::new(
                            RichText::new("NEXT TURN ▶▶")
                                .color(if self.state.flags.ledger_rebalance {
                                    accent_gold()
                                } else {
                                    accent_green()
                                })
                                .size(metrics.fs(12.5))
                                .strong(),
                        )
                        .fill(next_fill)
                        .stroke(Stroke::new(1.3_f32, accent_green()));
                        if ui.add_enabled(can_advance, next_btn).clicked() {
                            advance_quarter(&mut self.state);
                        }
                    });
                });

                ui.add_space(6.0);

                // Row 2: Responsive Live KPI Strip
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
                        if self.state.gdp_growth >= 0.0 { accent_green() } else { accent_red() },
                        card_w,
                    );
                    kpi_card(
                        ui,
                        "🏦 TREASURY",
                        &fmt_billions(self.state.treasury_bn),
                        &format!("Net {}/qtr", fmt_signed_billions(self.state.last_fiscal.net_balance)),
                        if self.state.last_fiscal.net_balance >= 0.0 { accent_green() } else { accent_amber() },
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
                        "⚔ FORCES",
                        &format!("{:.0}K", self.state.active_troops_k),
                        &format!(
                            "🪖{} ✈{} ⚓{} • Coup {:.0}%",
                            self.state.armed_forces.army_divisions,
                            self.state.armed_forces.fighter_squadrons,
                            self.state.armed_forces.destroyers + self.state.armed_forces.submarines,
                            self.state.coup_risk
                        ),
                        severity_color(self.state.coup_risk, false),
                        card_w,
                    );
                    kpi_card(
                        ui,
                        if self.state.pandemic.active { "🦠 PANDEMIC" } else { "🔥 EXTREMISM" },
                        &if self.state.pandemic.active {
                            format!("{:.0}%", self.state.pandemic.severity)
                        } else {
                            format!("{:.0}%", self.state.total_extremism())
                        },
                        &if self.state.pandemic.active {
                            format!("Vaccine {:.0}% • {}", self.state.pandemic.vaccine_progress, self.state.pandemic.lockdown_level.label())
                        } else {
                            format!("Insurgency {:.0}%", self.state.insurgency_strength)
                        },
                        if self.state.pandemic.active {
                            severity_color(self.state.pandemic.severity, false)
                        } else {
                            severity_color(self.state.total_extremism(), false)
                        },
                        card_w,
                    );
                });
            });

        // BOTTOM LIVE WORLD NEWS TICKER BAR
        egui::TopBottomPanel::bottom("bottom_ticker_bar")
            .frame(
                egui::Frame::new()
                    .fill(bg_panel())
                    .stroke(Stroke::new(1.0_f32, border_subtle()))
                    .inner_margin(Margin::symmetric(12, 5)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    badge_scaled(
                        ui,
                        if self.state.pandemic.active {
                            "🦠 HEALTH EMERGENCY"
                        } else if self.state.active_wars_count() > 0 {
                            "⚔ WARTIME WIRE"
                        } else if self.state.flags.ledger_rebalance {
                            "⭐ GOLDEN WIRE"
                        } else {
                            "📡 WORLD WIRE"
                        },
                        if self.state.pandemic.active || self.state.active_wars_count() > 0 {
                            accent_red()
                        } else if self.state.flags.ledger_rebalance {
                            accent_gold()
                        } else {
                            accent_cyan()
                        },
                        metrics.fs(10.0),
                    );
                    if let Some(latest) = self.state.logs.first() {
                        ui.label(
                            RichText::new(format!(
                                "[{}] {} — {}",
                                latest.date_label, latest.headline, latest.detail
                            ))
                            .color(text_primary())
                            .size(metrics.fs(11.0)),
                        );
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!(
                                "Turn {}  •  Power {:.0}  •  Pop {:.1}M  •  Arms Sales {}",
                                self.state.turn,
                                power_score,
                                self.state.population_m,
                                fmt_billions(self.state.armed_forces.total_arms_sales_bn)
                            ))
                            .color(text_muted())
                            .size(metrics.fs(10.5)),
                        );
                    });
                });
            });

        // LEFT NAVIGATION SIDEBAR
        egui::SidePanel::left("nav_sidebar")
            .exact_width(metrics.sidebar_w)
            .frame(
                egui::Frame::new()
                    .fill(bg_panel())
                    .stroke(Stroke::new(1.0_f32, border_subtle()))
                    .inner_margin(Margin::same(10)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(
                        RichText::new("COMMAND ROOMS")
                            .color(accent_gold())
                            .size(metrics.fs(10.8))
                            .strong(),
                    );
                    ui.add_space(6.0);

                    for &tab in DashboardTab::all() {
                        let selected = self.active_tab == tab;
                        let sel_bg = if self.dark_mode {
                            Color32::from_rgb(28, 56, 94)
                        } else {
                            Color32::from_rgb(214, 230, 248)
                        };
                        let text = RichText::new(tab.label())
                            .size(metrics.fs(12.6))
                            .color(if selected { text_primary() } else { text_muted() })
                            .strong();
                        let btn = egui::Button::new(text)
                            .min_size(egui::vec2(ui.available_width(), (32.0 * metrics.scale).clamp(27.0, 40.0)))
                            .fill(if selected { sel_bg } else { Color32::TRANSPARENT })
                            .stroke(if selected {
                                Stroke::new(1.2_f32, accent_blue())
                            } else {
                                Stroke::NONE
                            });
                        if ui.add(btn).clicked() {
                            self.active_tab = tab;
                        }
                        ui.add_space(3.0);
                    }

                    ui.add_space(10.0);
                    panel_frame().show(ui, |ui| {
                        ui.label(
                            RichText::new("👑 SOVEREIGN RANK")
                                .color(text_muted())
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
                                .color(text_muted())
                                .size(metrics.fs(10.8)),
                        );
                        ui.add_space(4.0);
                        let subdued = self.state.annexed_count() + self.state.puppet_count();
                        let total_conq = self.state.total_conquerable_powers().max(1);
                        let dom_pct = (subdued as f64 / total_conq as f64) * 100.0;
                        meter_bar(ui, &format!("Dominion ({}/{})", subdued, total_conq), dom_pct, true);
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new("🚩 ACTIVE STATE FLAGS")
                            .color(accent_cyan())
                            .size(metrics.fs(10.8))
                            .strong(),
                    );
                    ui.add_space(5.0);

                    let mut flag_items: Vec<(&str, String, &str, Color32)> = Vec::new();
                    if self.state.flags.ledger_rebalance {
                        let g_surplus = if self.state.god_surplus_bn > 0.0 {
                            self.state.god_surplus_bn
                        } else {
                            1000.0
                        };
                        flag_items.push((
                            "⭐",
                            format!("GOLDEN: {}/QTR", fmt_signed_billions(g_surplus)),
                            "Debt cleared, escalating quarterly surplus (+50x per click on nation name), 0% taxes unlocked, coup immunity & Golden Era incidents active.",
                            accent_gold(),
                        ));
                    }
                    if self.state.pandemic.active {
                        flag_items.push((
                            "🦠",
                            format!("PANDEMIC ({:.0}%)", self.state.pandemic.severity),
                            "Active public health outbreak! Adjust lockdown or fund Vaccine Sprint in Programs & Ops.",
                            accent_red(),
                        ));
                    }
                    if self.state.pandemic.lockdown_level != LockdownLevel::None {
                        flag_items.push((
                            "🔒",
                            self.state.pandemic.lockdown_level.label().to_uppercase(),
                            "Active movement restrictions curbing pathogen spread at economic cost.",
                            accent_amber(),
                        ));
                    }
                    if self.state.is_global_hegemon() {
                        flag_items.push((
                            "👑",
                            "GLOBAL HEGEMON".into(),
                            "All rival sovereign nations have been annexed or vassalized!",
                            accent_purple(),
                        ));
                    }
                    let annexed_n = self.state.annexed_count();
                    if annexed_n > 0 {
                        flag_items.push((
                            "⚔",
                            format!("ANNEXED: {} NATION(S)", annexed_n),
                            "Conquered foreign powers permanently integrated into your territory.",
                            accent_green(),
                        ));
                    }
                    let puppet_n = self.state.puppet_count();
                    if puppet_n > 0 {
                        flag_items.push((
                            "🛡",
                            format!("CLIENTS: {} STATE(S)", puppet_n),
                            "Compliant client regimes paying quarterly tribute and allied military support.",
                            accent_purple(),
                        ));
                    }
                    let rebel_n = self.state.rebel_states_count();
                    if rebel_n > 0 {
                        flag_items.push((
                            "🔥",
                            format!("REBELS: {} STATE(S)", rebel_n),
                            "Breakaway separatist rebel states active on the geopolitical map!",
                            accent_red(),
                        ));
                    }
                    if self.state.programs.fighter_jet_tier >= 2 {
                        flag_items.push((
                            "✈",
                            format!("JET: {}", self.state.programs.fighter_jet_name),
                            "Advanced indigenous stealth/hypersonic fighter squadrons operational.",
                            accent_cyan(),
                        ));
                    }
                    if self.state.programs.nuclear_warheads > 0 {
                        flag_items.push((
                            "☢",
                            format!("NUKES: {} WARHEADS", self.state.programs.nuclear_warheads),
                            "Active nuclear arsenal boosting strategic deterrence and war power.",
                            accent_amber(),
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
                            accent_cyan(),
                        ));
                    }
                    if self.state.flags.martial_law {
                        flag_items.push(("🚨", "MARTIAL LAW".into(), "Military policing curbs insurgency but reduces liberties.", accent_red()));
                    }
                    if self.state.flags.press_censorship {
                        flag_items.push(("🔇", "PRESS CENSORSHIP".into(), "State controls narratives; lowers press freedom.", accent_amber()));
                    }
                    if self.state.flags.secret_police {
                        flag_items.push(("🕵", "SECRET POLICE".into(), "Deters coup attempts & bolsters stability.", accent_amber()));
                    }
                    if self.state.flags.mass_surveillance {
                        flag_items.push(("📡", "SURVEILLANCE GRID".into(), "Suppresses insurgency & coup plots (-$1.4B/qtr).", accent_amber()));
                    }
                    if self.state.flags.religious_courts {
                        flag_items.push(("⚖", "CLERICAL COURTS".into(), "Empowers Clergy and raises clerical institutional power.", accent_purple()));
                    }
                    if self.state.flags.five_year_plan {
                        flag_items.push(("🏭", "5-YEAR PLAN".into(), "Boosts state industrial output and GDP growth.", accent_blue()));
                    }
                    if self.state.flags.universal_healthcare {
                        flag_items.push(("🏥", "UNIV. HEALTHCARE".into(), "Guarantees universal care (+Healthcare & Labor approval).", accent_green()));
                    }
                    if self.state.flags.sovereign_wealth_fund {
                        flag_items.push(("🏦", "SOVEREIGN FUND".into(), "Compounds permanent quarterly passive investment dividends.", accent_green()));
                    }
                    if self.state.flags.independent_judiciary {
                        flag_items.push(("🏛", "INDEP. COURTS".into(), "Strengthens Rule of Law and lowers corruption.", accent_green()));
                    }

                    if flag_items.is_empty() {
                        ui.label(
                            RichText::new("No active state flags")
                                .color(text_muted())
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
                            .fill(bg_card())
                            .stroke(Stroke::new(1.2_f32, accent_cyan()))
                            .corner_radius(CornerRadius::same(7))
                            .inner_margin(Margin::same(9))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(format!("⚡ {}", msg))
                                        .color(text_primary())
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
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Live Interactive Pandemic Alert Strip whenever an outbreak is active!
                if self.state.pandemic.active {
                    hero_card_frame(accent_red()).show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            badge_scaled(ui, "🦠 PANDEMIC EMERGENCY", accent_red(), metrics.fs(10.5));
                            ui.label(
                                RichText::new(format!(
                                    "{} — Severity {:.0}% | Infected {:.1}% | Vaccine {:.0}%",
                                    self.state.pandemic.pathogen_name,
                                    self.state.pandemic.severity,
                                    self.state.pandemic.infected_pct,
                                    self.state.pandemic.vaccine_progress
                                ))
                                .color(text_primary())
                                .size(metrics.fs(12.2))
                                .strong(),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("💉 Vaccine Sprint ($12B)").clicked() {
                                    match run_strategic_activity(&mut self.state, "pandemic_vaccine_sprint") {
                                        Ok(m) => toast_out = Some(m),
                                        Err(e) => toast_out = Some(e),
                                    }
                                }
                                for &lvl in LockdownLevel::all().iter().rev() {
                                    let active = self.state.pandemic.lockdown_level == lvl;
                                    if ui.selectable_label(active, lvl.label()).clicked() && !active {
                                        toast_out = Some(set_lockdown_level(&mut self.state, lvl));
                                    }
                                }
                            });
                        });
                    });
                    ui.add_space(8.0);
                }

                match self.active_tab {
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
                    DashboardTab::ProgramsOps => {
                        render_programs_ops_tab(ui, &mut self.state, &mut toast_out, metrics)
                    }
                    DashboardTab::Decisions => render_decisions_tab(ui, &mut self.state, &mut toast_out, metrics),
                    DashboardTab::Chronicles => {
                        render_chronicles_tab(ui, &self.state, &mut self.log_filter, metrics)
                    }
                }
            });
        });

        if jump_to_military {
            self.active_tab = DashboardTab::MilitaryDiplomacy;
        }
        if let Some(m) = toast_out {
            self.set_toast(m);
        }

        // INTERACTIVE CRISIS / VICTORY / GOLDEN ERA MODAL
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
                                let ev_badge_col = if is_positive { accent_gold() } else { accent_red() };
                                badge_scaled(ui, ev.category.label(), ev_badge_col, metrics.fs(11.0));
                                ui.label(
                                    RichText::new(&ev.title)
                                        .color(if is_positive { accent_gold() } else { text_primary() })
                                        .size(metrics.fs(16.0))
                                        .strong(),
                                );
                            });
                            ui.add_space(6.0);
                            ui.label(RichText::new(&ev.briefing).color(text_primary()).size(metrics.fs(13.0)));
                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(6.0);

                            let mut selected_choice: Option<String> = None;
                            for choice in &ev.choices {
                                card_frame().show(ui, |ui| {
                                    ui.label(
                                        RichText::new(&choice.label)
                                            .color(if is_positive { accent_green() } else { accent_blue() })
                                            .size(metrics.fs(13.5))
                                            .strong(),
                                    );
                                    ui.label(
                                        RichText::new(&choice.description)
                                            .color(text_primary())
                                            .size(metrics.fs(12.0)),
                                    );
                                    ui.label(
                                        RichText::new(format!("Projected Impact: {}", choice.impact_hint))
                                            .color(text_muted())
                                            .size(metrics.fs(11.5)),
                                    );
                                    ui.add_space(5.0);
                                    let btn = egui::Button::new(
                                        RichText::new("▶ EXECUTE DIRECTIVE")
                                            .color(if is_positive { accent_gold() } else { text_primary() })
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

        // GAME OVER MODAL
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
                    ui.label(RichText::new(reason.title()).color(accent_red()).size(metrics.fs(18.0)).strong());
                    ui.add_space(6.0);
                    ui.label(RichText::new(reason.message()).color(text_primary()).size(metrics.fs(13.0)));
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
