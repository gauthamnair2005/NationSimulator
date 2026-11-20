pub mod tabs;
pub mod widgets;

use crate::events::resolve_event_choice;
use crate::models::*;
use crate::simulation::advance_quarter;
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

    // Auto-play controls
    auto_play: bool,
    auto_interval_ms: u64,
    last_tick: Instant,

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
            auto_interval_ms: 1400,
            last_tick: Instant::now(),
            custom_name: "Republic of Novaria".into(),
            custom_leader: "Soren Lindqvist".into(),
            custom_title: "President".into(),
            custom_regime: RegimeType::LiberalDemocracy,
            custom_economy: EconomicSystem::MixedEconomy,
            custom_religion: ReligionPolicy::SecularPluralism,
        }
    }

    fn save_path() -> PathBuf {
        PathBuf::from("sovereign_save.json")
    }

    fn set_toast(&mut self, msg: impl Into<String>) {
        self.status_toast = Some((msg.into(), Instant::now()));
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
    visuals.extreme_bg_color = Color32::from_rgb(9, 13, 19);
    visuals.widgets.noninteractive.bg_fill = BG_CARD;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT_PRIMARY);
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(32, 43, 60);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(44, 59, 82);
    visuals.widgets.active.bg_fill = Color32::from_rgb(37, 99, 235);
    visuals.selection.bg_fill = Color32::from_rgb(30, 64, 118);
    ctx.set_visuals(visuals);
}

impl eframe::App for NationSimApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle auto-play ticking
        if self.screen == AppScreen::Dashboard
            && self.auto_play
            && self.state.active_event.is_none()
            && self.state.game_over.is_none()
        {
            if self.last_tick.elapsed() >= Duration::from_millis(self.auto_interval_ms) {
                advance_quarter(&mut self.state);
                self.last_tick = Instant::now();
            }
            ctx.request_repaint_after(Duration::from_millis(120));
        }

        // Expire toast after 5 seconds
        if let Some((_, ts)) = &self.status_toast {
            if ts.elapsed() > Duration::from_secs(5) {
                self.status_toast = None;
            }
        }

        match self.screen {
            AppScreen::Setup => self.render_setup_screen(ctx),
            AppScreen::Dashboard => self.render_dashboard(ctx),
        }
    }
}

impl NationSimApp {
    fn render_setup_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("SOVEREIGN // STATECRAFT SIMULATOR").color(ACCENT_BLUE).size(24.0).strong());
                        ui.label(RichText::new("Geopolitical, Economic, Ideological, Religious & Military Dashboard Engine").color(TEXT_MUTED).size(13.0));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if Self::save_path().exists() && ui.button(RichText::new("Load Saved Game").size(13.0).strong()).clicked() {
                            self.load_game();
                        }
                    });
                });
                ui.add_space(14.0);
                ui.separator();
                ui.add_space(10.0);

                ui.label(RichText::new("SELECT A REAL-WORLD ARCHETYPE SCENARIO").color(TEXT_PRIMARY).size(15.0).strong());
                ui.add_space(8.0);

                let mut chosen_preset: Option<ScenarioPreset> = None;
                for preset in ScenarioPreset::all() {
                    card_frame().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(preset.title()).color(TEXT_PRIMARY).size(15.0).strong());
                                ui.label(RichText::new(preset.subtitle()).color(ACCENT_BLUE).size(12.0));
                                ui.add_space(3.0);
                                ui.label(RichText::new(preset.summary()).color(TEXT_MUTED).size(12.0));
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(RichText::new("ASSUME COMMAND").strong()).clicked() {
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
                card_frame().show(ui, |ui| {
                    ui.label(RichText::new("OR BUILD A CUSTOM SOVEREIGN NATION").color(ACCENT_GREEN).size(14.5).strong());
                    ui.add_space(8.0);
                    egui::Grid::new("custom_nation_grid").num_columns(2).spacing([14.0, 8.0]).show(ui, |ui| {
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
                    ui.add_space(10.0);
                    if ui.button(RichText::new("LAUNCH CUSTOM NATION").color(ACCENT_GREEN).strong()).clicked() {
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
    }

    fn render_dashboard(&mut self, ctx: &egui::Context) {
        // TOP COMMAND BAR
        egui::TopBottomPanel::top("top_command_bar")
            .frame(
                egui::Frame::new()
                    .fill(BG_PANEL)
                    .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                    .inner_margin(Margin::symmetric(12, 8)),
            )
            .show(ctx, |ui| {
                // Row 1: Identity & Turn Controls
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(&self.state.nation_name).color(TEXT_PRIMARY).size(17.0).strong());
                    badge(ui, self.state.regime.short_badge(), ACCENT_PURPLE);
                    badge(ui, self.state.economy.short_badge(), ACCENT_BLUE);
                    badge(ui, &self.state.date_label(), ACCENT_GREEN);
                    badge(ui, &format!("POL CAPITAL: {:.0}", self.state.political_capital), ACCENT_AMBER);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("New Game").clicked() {
                            self.auto_play = false;
                            self.screen = AppScreen::Setup;
                        }
                        if ui.button("Load").clicked() {
                            self.load_game();
                        }
                        if ui.button("Save").clicked() {
                            self.save_game();
                        }

                        ui.separator();

                        let can_advance = self.state.active_event.is_none() && self.state.game_over.is_none();
                        let play_lbl = if self.auto_play { "⏸ Pause" } else { "▶ Auto" };
                        if ui.add_enabled(can_advance, egui::Button::new(play_lbl)).clicked() {
                            self.auto_play = !self.auto_play;
                            self.last_tick = Instant::now();
                        }

                        if ui
                            .add_enabled(
                                can_advance,
                                egui::Button::new(RichText::new("NEXT QUARTER ▶▶").color(ACCENT_GREEN).strong()),
                            )
                            .clicked()
                        {
                            advance_quarter(&mut self.state);
                        }
                    });
                });

                ui.add_space(6.0);

                // Row 2: Live KPI Strip
                ui.horizontal_wrapped(|ui| {
                    kpi_card(
                        ui,
                        "GDP",
                        &format!("${:.1}B", self.state.gdp_bn),
                        &format!("{:+.2}% / yr", self.state.gdp_growth),
                        if self.state.gdp_growth >= 0.0 { ACCENT_GREEN } else { ACCENT_RED },
                        138.0,
                    );
                    kpi_card(
                        ui,
                        "TREASURY",
                        &format!("${:.1}B", self.state.treasury_bn),
                        &format!("Net {:+.1}B/qtr", self.state.last_fiscal.net_balance),
                        if self.state.last_fiscal.net_balance >= 0.0 { ACCENT_GREEN } else { ACCENT_AMBER },
                        142.0,
                    );
                    kpi_card(
                        ui,
                        "APPROVAL",
                        &format!("{:.1}%", self.state.approval),
                        &format!("Legitimacy {:.0}%", self.state.legitimacy),
                        severity_color(self.state.approval, true),
                        138.0,
                    );
                    kpi_card(
                        ui,
                        "STABILITY",
                        &format!("{:.1}%", self.state.stability),
                        &format!("Democracy {:.0}%", self.state.democracy_index),
                        severity_color(self.state.stability, true),
                        138.0,
                    );
                    kpi_card(
                        ui,
                        "COUP RISK",
                        &format!("{:.0}%", self.state.coup_risk),
                        &format!("Mil App {:.0}%", self.state.faction(FactionId::Military).approval),
                        severity_color(self.state.coup_risk, false),
                        135.0,
                    );
                    kpi_card(
                        ui,
                        "EXTREMISM",
                        &format!("{:.0}%", self.state.total_extremism()),
                        &format!("Insurgency {:.0}%", self.state.insurgency_strength),
                        severity_color(self.state.total_extremism(), false),
                        138.0,
                    );
                });
            });

        // LEFT NAVIGATION SIDEBAR
        egui::SidePanel::left("nav_sidebar")
            .exact_width(215.0)
            .frame(
                egui::Frame::new()
                    .fill(BG_PANEL)
                    .stroke(Stroke::new(1.0_f32, BORDER_SUBTLE))
                    .inner_margin(Margin::same(10)),
            )
            .show(ctx, |ui| {
                ui.label(RichText::new("COMMAND MODULES").color(TEXT_MUTED).size(11.0).strong());
                ui.add_space(6.0);

                for &tab in DashboardTab::all() {
                    let selected = self.active_tab == tab;
                    let text = RichText::new(tab.label())
                        .size(13.0)
                        .color(if selected { TEXT_PRIMARY } else { TEXT_MUTED });
                    let btn = egui::Button::new(text)
                        .min_size(egui::vec2(ui.available_width(), 32.0))
                        .fill(if selected {
                            Color32::from_rgb(30, 58, 95)
                        } else {
                            Color32::TRANSPARENT
                        });
                    if ui.add(btn).clicked() {
                        self.active_tab = tab;
                    }
                    ui.add_space(3.0);
                }

                ui.add_space(14.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label(RichText::new("ACTIVE FLAGS").color(TEXT_MUTED).size(11.0).strong());
                ui.add_space(4.0);

                let mut active_flags: Vec<(&str, Color32)> = Vec::new();
                if self.state.flags.martial_law { active_flags.push(("MARTIAL LAW", ACCENT_RED)); }
                if self.state.flags.press_censorship { active_flags.push(("CENSORSHIP", ACCENT_AMBER)); }
                if self.state.flags.secret_police { active_flags.push(("SECRET POLICE", ACCENT_AMBER)); }
                if self.state.flags.mass_surveillance { active_flags.push(("SURVEILLANCE", ACCENT_AMBER)); }
                if self.state.flags.religious_courts { active_flags.push(("CLERICAL COURTS", ACCENT_PURPLE)); }
                if self.state.flags.five_year_plan { active_flags.push(("5-YEAR PLAN", ACCENT_BLUE)); }
                if self.state.flags.universal_healthcare { active_flags.push(("UNIV. HEALTHCARE", ACCENT_GREEN)); }
                if self.state.flags.sovereign_wealth_fund { active_flags.push(("SOVEREIGN FUND", ACCENT_GREEN)); }
                if self.state.flags.independent_judiciary { active_flags.push(("INDEP. COURTS", ACCENT_GREEN)); }

                if active_flags.is_empty() {
                    ui.label(RichText::new("None active").color(TEXT_MUTED).size(11.5));
                } else {
                    ui.horizontal_wrapped(|ui| {
                        for (lbl, col) in active_flags {
                            badge(ui, lbl, col);
                        }
                    });
                }

                if let Some((msg, _)) = &self.status_toast {
                    ui.add_space(12.0);
                    egui::Frame::new()
                        .fill(Color32::from_rgb(24, 42, 62))
                        .stroke(Stroke::new(1.0_f32, ACCENT_BLUE))
                        .corner_radius(CornerRadius::same(5))
                        .inner_margin(Margin::same(8))
                        .show(ui, |ui| {
                            ui.label(RichText::new(msg).color(TEXT_PRIMARY).size(11.5));
                        });
                }
            });

        // MAIN WORKSPACE PANEL
        let mut toast_out: Option<String> = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.active_tab {
                DashboardTab::Overview => render_overview_tab(ui, &self.state),
                DashboardTab::Economy => render_economy_tab(ui, &mut self.state, &mut toast_out),
                DashboardTab::Politics => render_politics_tab(ui, &mut self.state, &mut toast_out),
                DashboardTab::ReligionExtremism => {
                    render_religion_extremism_tab(ui, &mut self.state, &mut toast_out)
                }
                DashboardTab::MilitaryDiplomacy => {
                    render_military_diplomacy_tab(ui, &mut self.state, &mut toast_out)
                }
                DashboardTab::Decisions => render_decisions_tab(ui, &mut self.state, &mut toast_out),
                DashboardTab::Chronicles => render_chronicles_tab(ui, &self.state, &mut self.log_filter),
            });
        });

        if let Some(m) = toast_out {
            self.set_toast(m);
        }

        // INTERACTIVE CRISIS / SCENARIO MODAL
        if let Some(ev) = self.state.active_event.clone() {
            self.auto_play = false;
            egui::Window::new(format!("CLASSIFIED BRIEFING // {}", ev.urgency))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .default_width(640.0)
                .max_width(640.0)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        badge(ui, ev.category.label(), ACCENT_RED);
                        ui.label(RichText::new(&ev.title).color(TEXT_PRIMARY).size(16.0).strong());
                    });
                    ui.add_space(6.0);
                    ui.label(RichText::new(&ev.briefing).color(TEXT_PRIMARY).size(13.0));
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(6.0);

                    let mut selected_choice: Option<String> = None;
                    for choice in &ev.choices {
                        card_frame().show(ui, |ui| {
                            ui.label(RichText::new(&choice.label).color(ACCENT_BLUE).size(13.5).strong());
                            ui.label(RichText::new(&choice.description).color(TEXT_PRIMARY).size(12.0));
                            ui.label(RichText::new(format!("Projected Impact: {}", choice.impact_hint)).color(TEXT_MUTED).size(11.5));
                            ui.add_space(4.0);
                            if ui.button("EXECUTE DIRECTIVE").clicked() {
                                selected_choice = Some(choice.id.clone());
                            }
                        });
                        ui.add_space(6.0);
                    }

                    if let Some(cid) = selected_choice {
                        resolve_event_choice(&mut self.state, &cid);
                    }
                });
        }

        // GAME OVER / HISTORICAL LEGACY MODAL
        if let Some(reason) = self.state.game_over.clone() {
            self.auto_play = false;
            egui::Window::new("HISTORICAL EPILOGUE // REGIME TERMINATED")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .default_width(560.0)
                .max_width(560.0)
                .show(ctx, |ui| {
                    ui.label(RichText::new(reason.title()).color(ACCENT_RED).size(18.0).strong());
                    ui.add_space(6.0);
                    ui.label(RichText::new(reason.message()).color(TEXT_PRIMARY).size(13.0));
                    ui.add_space(10.0);
                    ui.separator();
                    ui.label(format!(
                        "Tenure: {} Quarter(s) (Ended {})  |  Final GDP: ${:.1}B  |  Final Regime: {}",
                        self.state.turn,
                        self.state.date_label(),
                        self.state.gdp_bn,
                        self.state.regime.label()
                    ));
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.button(RichText::new("Start New Nation").strong()).clicked() {
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
