use crate::decisions::{all_decisions, can_enact, enact_decision};
use crate::models::*;
use crate::simulation::{calculate_coup_risk, calculate_fiscal, compute_credit_rating, diplomatic_action, zero_all_taxes};
use crate::ui::widgets::*;
use eframe::egui::{self, Color32, RichText, Stroke};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardTab {
    Overview,
    Economy,
    Politics,
    ReligionExtremism,
    MilitaryDiplomacy,
    Decisions,
    Chronicles,
}

impl DashboardTab {
    pub fn all() -> &'static [DashboardTab] {
        &[
            Self::Overview,
            Self::Economy,
            Self::Politics,
            Self::ReligionExtremism,
            Self::MilitaryDiplomacy,
            Self::Decisions,
            Self::Chronicles,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Overview => "🏰  War Room",
            Self::Economy => "💰  Economy & Tax",
            Self::Politics => "🏛  Politics & Court",
            Self::ReligionExtremism => "🕊  Faith & Security",
            Self::MilitaryDiplomacy => "⚔  Military & Map",
            Self::Decisions => "📜  Royal Decrees",
            Self::Chronicles => "📖  Chronicles",
        }
    }
}

macro_rules! two_col_responsive {
    ($ui:expr, |$u1:ident| $left:block, |$u2:ident| $right:block $(,)?) => {
        if $ui.available_width() >= 760.0 {
            $ui.columns(2, |cols| {
                {
                    let $u1 = &mut cols[0];
                    $left
                }
                {
                    let $u2 = &mut cols[1];
                    $right
                }
            });
        } else {
            {
                let $u1 = &mut *$ui;
                $left
            }
            $ui.add_space(10.0);
            {
                let $u2 = &mut *$ui;
                $right
            }
        }
    };
}

pub fn render_overview_tab(
    ui: &mut egui::Ui,
    state: &NationState,
    metrics: &UiMetrics,
) -> Option<ForeignPowerId> {
    ui.label(
        RichText::new("🏰 SOVEREIGN WAR ROOM & STRATEGIC OVERVIEW")
            .color(ACCENT_GOLD)
            .size(metrics.fs(15.5))
            .strong(),
    );
    ui.add_space(6.0);

    // Alerts Banner
    let mut alerts: Vec<(Color32, String)> = Vec::new();
    if state.coup_risk >= 50.0 {
        alerts.push((
            ACCENT_RED,
            format!(
                "CRITICAL COUP RISK ({:.0}%): Military/Security loyalty dangerously low!",
                state.coup_risk
            ),
        ));
    }
    if state.total_extremism() >= 45.0 || state.insurgency_strength >= 40.0 {
        alerts.push((
            ACCENT_RED,
            format!(
                "EXTREMISM ALERT: Insurgency at {:.0}%, Extremism index at {:.0}%.",
                state.insurgency_strength,
                state.total_extremism()
            ),
        ));
    }
    if state.active_wars_count() > 0 {
        alerts.push((
            ACCENT_AMBER,
            format!(
                "ACTIVE CONFLICT: Armed forces engaged in {} war(s). War Exhaustion: {:.0}%.",
                state.active_wars_count(),
                state.war_exhaustion
            ),
        ));
    }
    if state.inflation >= 10.0 {
        alerts.push((
            ACCENT_AMBER,
            format!(
                "INFLATION SURGE ({:.1}%): Consider raising Central Bank interest rates or curbing deficits.",
                state.inflation
            ),
        ));
    }
    if state.imf_austerity_turns > 0 {
        alerts.push((
            ACCENT_AMBER,
            format!(
                "GMC AUSTERITY MANDATE: {} quarter(s) remaining.",
                state.imf_austerity_turns
            ),
        ));
    }
    if state.regime.is_democratic() && !state.flags.term_limits_abolished {
        alerts.push((
            ACCENT_BLUE,
            format!(
                "NEXT GENERAL ELECTION: {} quarter(s) remaining.",
                state.quarters_until_election
            ),
        ));
    }
    if state.is_global_hegemon() {
        alerts.push((
            ACCENT_GOLD,
            format!(
                "GLOBAL HEGEMONY: All rival sovereign powers have been annexed or vassalized by {}!",
                state.nation_name
            ),
        ));
    }
    if state.flags.ledger_rebalance {
        alerts.push((
            ACCENT_AMBER,
            "GOLDEN MANDATE ACTIVE: Sovereign debt cleared, +$1.00T/qtr surplus, 0% tax ceiling & Golden Era incidents enabled.".into(),
        ));
    }

    if !alerts.is_empty() {
        card_frame().show(ui, |ui| {
            ui.label(
                RichText::new("⚡ ACTIVE SITUATION ADVISORIES")
                    .color(TEXT_MUTED)
                    .size(metrics.fs(11.0))
                    .strong(),
            );
            ui.add_space(4.0);
            for (col, msg) in alerts {
                ui.horizontal_wrapped(|ui| {
                    badge_scaled(ui, "ALERT", col, metrics.fs(10.0));
                    ui.label(RichText::new(msg).color(TEXT_PRIMARY).size(metrics.fs(12.2)));
                });
            }
        });
        ui.add_space(10.0);
    }

    // Interactive Grand-Strategy Geopolitical Theater Map
    let clicked_power = draw_world_theater_map(ui, state, metrics);
    ui.add_space(10.0);

    // Achievements / Trophies Bar
    card_frame().show(ui, |ui| {
        let achievements = compute_achievements(state);
        let unlocked_n = achievements.iter().filter(|a| a.3).count();
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new("🏆 SOVEREIGN MILESTONES & TROPHIES")
                    .color(ACCENT_GOLD)
                    .size(metrics.fs(12.0))
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                badge_scaled(
                    ui,
                    &format!("{}/{} UNLOCKED", unlocked_n, achievements.len()),
                    ACCENT_GOLD,
                    metrics.fs(10.5),
                );
            });
        });
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            for (icon, name, desc, unlocked, col) in achievements {
                let display_col = if unlocked { col } else { Color32::from_rgb(80, 92, 112) };
                let label = if unlocked {
                    format!("{} {}", icon, name)
                } else {
                    format!("🔒 {}", name)
                };
                let tip = if unlocked {
                    format!("UNLOCKED: {}", desc)
                } else {
                    format!("LOCKED: {}", desc)
                };
                draw_status_flag_chip(ui, if unlocked { "★" } else { "•" }, &label, &tip, display_col, metrics.scale * 0.96);
            }
        });
    });
    ui.add_space(10.0);

    // Responsive Trend Charts Grid (3 / 2 / 1 columns based on window width)
    let gdp_hist: Vec<f64> = state.history.iter().map(|h| h.gdp_bn).collect();
    let app_hist: Vec<f64> = state.history.iter().map(|h| h.approval).collect();
    let stab_hist: Vec<f64> = state.history.iter().map(|h| h.stability).collect();
    let inf_hist: Vec<f64> = state.history.iter().map(|h| h.inflation).collect();
    let coup_hist: Vec<f64> = state.history.iter().map(|h| h.coup_risk).collect();
    let ext_hist: Vec<f64> = state.history.iter().map(|h| h.extremism).collect();

    let charts: [(&str, &Vec<f64>, &str, Color32); 6] = [
        ("REAL GDP ($B)", &gdp_hist, "B", ACCENT_BLUE),
        ("PUBLIC APPROVAL", &app_hist, "%", ACCENT_GREEN),
        ("REGIME STABILITY", &stab_hist, "%", ACCENT_PURPLE),
        ("INFLATION RATE", &inf_hist, "%", ACCENT_AMBER),
        ("MILITARY COUP RISK", &coup_hist, "%", ACCENT_RED),
        ("EXTREMISM INDEX", &ext_hist, "%", ACCENT_RED),
    ];

    let chart_cols = if ui.available_width() >= 860.0 {
        3
    } else if ui.available_width() >= 540.0 {
        2
    } else {
        1
    };

    for row in charts.chunks(chart_cols) {
        if row.len() == 1 {
            let (t, vals, unit, col) = row[0];
            sparkline_card(ui, t, vals, unit, col);
        } else {
            ui.columns(row.len(), |cols| {
                for (i, &(t, vals, unit, col)) in row.iter().enumerate() {
                    sparkline_card(&mut cols[i], t, vals, unit, col);
                }
            });
        }
        ui.add_space(8.0);
    }

    // State Profile (with Large Flag!) & Recent Intelligence Feed
    two_col_responsive!(
        ui,
        |ui| {
            card_frame().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let hero_h = (metrics.flag_h * 1.45).clamp(34.0, 62.0);
                    let hero_w = hero_h * 1.55;
                    draw_nation_flag_from_spec(
                        ui,
                        state.regime,
                        state.economy,
                        state.religion_policy,
                        state.flags.ledger_rebalance,
                        state.is_global_hegemon(),
                        hero_w,
                        hero_h,
                    );
                    ui.add_space(6.0);
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("NATIONAL PROFILE & CAPACITY")
                                .color(ACCENT_CYAN)
                                .size(metrics.fs(12.0))
                                .strong(),
                        );
                        ui.label(
                            RichText::new(format!("{} {}", state.leader_title, state.leader_name))
                                .color(TEXT_PRIMARY)
                                .size(metrics.fs(14.0))
                                .strong(),
                        );
                        ui.label(
                            RichText::new(format!(
                                "Pop: {:.2}M  •  Credit: {}  •  Debt/GDP: {:.1}%  •  Gini: {:.1}",
                                state.population_m,
                                state.credit_rating,
                                state.debt_to_gdp(),
                                state.gini
                            ))
                            .color(TEXT_MUTED)
                            .size(metrics.fs(11.5)),
                        );
                    });
                });

                if state.annexed_count() > 0 || state.puppet_count() > 0 {
                    ui.add_space(6.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Empire Territories:").color(TEXT_MUTED));
                        ui.label(
                            RichText::new(format!(
                                "{} Annexed • {} Client State(s)",
                                state.annexed_count(),
                                state.puppet_count()
                            ))
                            .color(ACCENT_GOLD)
                            .strong(),
                        );
                    });
                    if !state.annexed_territories.is_empty() {
                        ui.label(
                            RichText::new(format!("Provinces: {}", state.annexed_territories.join(", ")))
                                .color(TEXT_MUTED)
                                .size(11.5),
                        );
                    }
                }

                ui.add_space(8.0);
                meter_bar(ui, "Democracy Index", state.democracy_index, true);
                meter_bar(ui, "Rule of Law", state.rule_of_law, true);
                meter_bar(ui, "Civil Liberties", state.civil_liberties, true);
                meter_bar(ui, "Corruption Level", state.corruption, false);
                meter_bar(
                    ui,
                    "Healthcare & Education Avg",
                    (state.healthcare_level + state.education_level) * 0.5,
                    true,
                );
                meter_bar(
                    ui,
                    "Technology & Infrastructure",
                    (state.tech_level + state.infrastructure_level) * 0.5,
                    true,
                );
            });
        },
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("📡 RECENT INTELLIGENCE DISPATCHES")
                        .color(ACCENT_CYAN)
                        .size(metrics.fs(12.0))
                        .strong(),
                );
                ui.add_space(6.0);
                for entry in state.logs.iter().take(7) {
                    let col = match entry.category {
                        LogCategory::Crisis => ACCENT_RED,
                        LogCategory::Military => ACCENT_AMBER,
                        LogCategory::Economic => ACCENT_GREEN,
                        LogCategory::Religious => ACCENT_PURPLE,
                        _ => ACCENT_BLUE,
                    };
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(&entry.date_label).color(TEXT_MUTED).size(11.0));
                        badge(ui, entry.category.label(), col);
                        ui.label(RichText::new(&entry.headline).color(TEXT_PRIMARY).size(12.0).strong());
                    });
                    ui.label(RichText::new(&entry.detail).color(TEXT_MUTED).size(11.5));
                    ui.add_space(4.0);
                    ui.separator();
                }
            });
        },
    );

    clicked_power
}

pub fn render_economy_tab(
    ui: &mut egui::Ui,
    state: &mut NationState,
    status_msg: &mut Option<String>,
    metrics: &UiMetrics,
) {
    ui.label(
        RichText::new("💰 MACROECONOMIC COMMAND & TREASURY")
            .color(ACCENT_GREEN)
            .size(metrics.fs(15.5))
            .strong(),
    );
    ui.add_space(6.0);

    let fiscal = calculate_fiscal(state);
    state.last_fiscal = fiscal.clone();

    // Responsive Top Macro Summary Row
    let avail_w = ui.available_width().max(380.0);
    let cols = if avail_w >= 880.0 {
        5
    } else if avail_w >= 540.0 {
        3
    } else {
        2
    };
    let card_w = ((avail_w - (cols as f32 - 1.0) * 8.0) / cols as f32 - 22.0).max(125.0);

    ui.horizontal_wrapped(|ui| {
        kpi_card(
            ui,
            "NOMINAL GDP",
            &fmt_billions(state.gdp_bn),
            &format!("Growth {:+.2}%/yr", state.gdp_growth),
            severity_color(state.gdp_growth * 15.0 + 40.0, true),
            card_w,
        );
        kpi_card(
            ui,
            "QUARTERLY BALANCE",
            &fmt_signed_billions(fiscal.net_balance),
            &format!(
                "Rev {} / Exp {}",
                fmt_billions(fiscal.total_revenue),
                fmt_billions(fiscal.total_spending)
            ),
            if fiscal.net_balance >= 0.0 { ACCENT_GREEN } else { ACCENT_RED },
            card_w,
        );
        kpi_card(
            ui,
            "NATIONAL DEBT",
            &fmt_billions(state.national_debt_bn),
            &format!("{:.1}% GDP • {}", state.debt_to_gdp(), state.credit_rating),
            severity_color(state.debt_to_gdp() * 0.7, false),
            card_w,
        );
        kpi_card(
            ui,
            "INFLATION",
            &format!("{:.1}%", state.inflation),
            &format!("Central Rate {:.1}%", state.taxes.interest_rate),
            severity_color(state.inflation * 4.5, false),
            card_w,
        );
        kpi_card(
            ui,
            "UNEMPLOYMENT",
            &format!("{:.1}%", state.unemployment),
            &format!("Gini Index {:.1}", state.gini),
            severity_color(state.unemployment * 5.0, false),
            card_w,
        );
    });

    ui.add_space(10.0);

    two_col_responsive!(
        ui,
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("ECONOMIC MODEL & OWNERSHIP")
                        .color(TEXT_MUTED)
                        .size(12.0)
                        .strong(),
                );
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label("Current System:");
                    badge(ui, state.economy.label(), ACCENT_BLUE);
                });
                ui.label(
                    RichText::new(
                        "Switch economic models structurally via Royal Decrees (Nationalization / Shock Therapy) or adjust levers below:",
                    )
                    .color(TEXT_MUTED)
                    .size(11.5),
                );
                ui.add_space(6.0);

                if ui.button("⚔ Switch to Total War Economy (15 PC)").clicked() {
                    if state.political_capital >= 15.0 {
                        state.political_capital -= 15.0;
                        state.economy = EconomicSystem::WarEconomy;
                        state.defense_posture = DefensePosture::FullMobilization;
                        state.log(
                            LogCategory::Economic,
                            "Total War Economy Proclaimed",
                            "Civilian factories converted to defense production.",
                        );
                        *status_msg = Some("Shifted to Total War Economy.".into());
                    } else {
                        *status_msg = Some("Need 15 Political Capital.".into());
                    }
                }

                ui.separator();
                ui.label(
                    RichText::new("TAXATION & MONETARY POLICY")
                        .color(TEXT_MUTED)
                        .size(12.0)
                        .strong(),
                );
                ui.add_space(4.0);

                let min_tax = if state.flags.ledger_rebalance { 0.0 } else { 5.0 };
                let min_rate = if state.flags.ledger_rebalance { 0.0 } else { 0.5 };
                ui.add(
                    egui::Slider::new(&mut state.taxes.income_tax, min_tax..=70.0)
                        .text("Income Tax (%)")
                        .fixed_decimals(1),
                );
                ui.add(
                    egui::Slider::new(&mut state.taxes.corporate_tax, min_tax..=65.0)
                        .text("Corporate Tax (%)")
                        .fixed_decimals(1),
                );
                ui.add(
                    egui::Slider::new(&mut state.taxes.tariffs, 0.0..=60.0)
                        .text("Import Tariffs (%)")
                        .fixed_decimals(1),
                );
                ui.add(
                    egui::Slider::new(&mut state.taxes.wealth_tax, 0.0..=30.0)
                        .text("Wealth / Capital Tax (%)")
                        .fixed_decimals(1),
                );
                ui.add(
                    egui::Slider::new(&mut state.taxes.state_ownership, 0.0..=100.0)
                        .text("State Enterprise Share (%)")
                        .fixed_decimals(0),
                );
                ui.add(
                    egui::Slider::new(&mut state.taxes.interest_rate, min_rate..=25.0)
                        .text("Central Bank Rate (%)")
                        .fixed_decimals(1),
                );

                if state.flags.ledger_rebalance {
                    ui.add_space(4.0);
                    if ui
                        .button(
                            RichText::new("⭐ Set All Taxes & Rates to 0% (Golden Mandate)")
                                .color(ACCENT_GOLD)
                                .strong(),
                        )
                        .clicked()
                    {
                        zero_all_taxes(state);
                        *status_msg = Some("All taxes & central bank interest rate set to 0.0%!".into());
                    }
                }

                ui.add_space(8.0);
                ui.checkbox(
                    &mut state.flags.capital_controls,
                    "Capital Controls (Blocks flight, angers Capitalists & markets)",
                );

                ui.separator();
                ui.label(
                    RichText::new("QUARTERLY REVENUE BREAKDOWN")
                        .color(TEXT_MUTED)
                        .size(11.5)
                        .strong(),
                );
                ui.label(format!(
                    "Income Tax: ${:.2}B  |  Corporate Tax: ${:.2}B",
                    fiscal.income_rev, fiscal.corp_rev
                ));
                ui.label(format!(
                    "Tariffs: ${:.2}B  |  Wealth Tax: ${:.2}B  |  State Enterprises: ${:.2}B",
                    fiscal.tariff_rev, fiscal.wealth_rev, fiscal.soe_rev
                ));
                ui.label(
                    RichText::new(format!("Quarterly Debt Servicing Interest: -${:.2}B", fiscal.debt_interest))
                        .color(ACCENT_AMBER),
                );
            });
        },
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("PUBLIC BUDGET ALLOCATIONS (INTENSITY 0-100)")
                        .color(TEXT_MUTED)
                        .size(12.0)
                        .strong(),
                );
                ui.add_space(6.0);

                ui.add(egui::Slider::new(&mut state.budget.healthcare, 0.0..=100.0).text("Healthcare").fixed_decimals(0));
                ui.add(egui::Slider::new(&mut state.budget.education, 0.0..=100.0).text("Education & Universities").fixed_decimals(0));
                ui.add(egui::Slider::new(&mut state.budget.infrastructure, 0.0..=100.0).text("Infrastructure & Energy").fixed_decimals(0));
                ui.add(egui::Slider::new(&mut state.budget.welfare, 0.0..=100.0).text("Social Welfare & Pensions").fixed_decimals(0));
                ui.add(egui::Slider::new(&mut state.budget.military, 0.0..=100.0).text("Military & Defense").fixed_decimals(0));
                ui.add(egui::Slider::new(&mut state.budget.security_intel, 0.0..=100.0).text("Internal Security & Intel").fixed_decimals(0));
                ui.add(egui::Slider::new(&mut state.budget.research, 0.0..=100.0).text("Scientific R&D").fixed_decimals(0));
                ui.add(egui::Slider::new(&mut state.budget.religion_culture, 0.0..=100.0).text("Religion & Cultural Subsidies").fixed_decimals(0));

                ui.add_space(10.0);
                ui.separator();
                ui.label(
                    RichText::new("TREASURY & DEBT OPERATIONS")
                        .color(TEXT_MUTED)
                        .size(12.0)
                        .strong(),
                );
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    if ui.button("💵 Pay Down $15B Debt").clicked() {
                        if state.treasury_bn >= 15.0 && state.national_debt_bn > 0.0 {
                            let pay = state.national_debt_bn.min(15.0);
                            state.treasury_bn -= pay;
                            state.national_debt_bn -= pay;
                            state.credit_rating = compute_credit_rating(state);
                            *status_msg = Some(format!("Retired ${:.1}B of sovereign debt.", pay));
                        } else {
                            *status_msg = Some("Insufficient Treasury reserves ($15B required).".into());
                        }
                    }
                    if ui.button("🖨 Print Emergency Liquidity (+$20B, +3.5% Infl.)").clicked() {
                        state.treasury_bn += 20.0;
                        state.inflation += 3.5;
                        state.log(
                            LogCategory::Economic,
                            "Central Bank Monetized Deficit",
                            "Printed $20B in liquidity, stoking inflation.",
                        );
                        *status_msg = Some("Printed $20B liquidity (+3.5% inflation).".into());
                    }
                });
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!("Trade Balance: {:+.1}B / qtr", state.trade_balance_bn))
                        .color(if state.trade_balance_bn >= 0.0 { ACCENT_GREEN } else { ACCENT_AMBER }),
                );
            });
        },
    );
}

pub fn render_politics_tab(
    ui: &mut egui::Ui,
    state: &mut NationState,
    status_msg: &mut Option<String>,
    metrics: &UiMetrics,
) {
    ui.label(
        RichText::new("🏛 POLITICAL ARCHITECTURE, DECREES & DOMESTIC FACTIONS")
            .color(ACCENT_PURPLE)
            .size(metrics.fs(15.5))
            .strong(),
    );
    ui.add_space(6.0);

    two_col_responsive!(
        ui,
        |ui| {
            card_frame().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("REGIME TYPE:").color(TEXT_MUTED).strong());
                    badge(ui, state.regime.label(), ACCENT_PURPLE);
                });
                ui.add_space(6.0);

                meter_bar(ui, "Public Approval", state.approval, true);
                meter_bar(ui, "Regime Stability", state.stability, true);
                meter_bar(ui, "Constitutional Legitimacy", state.legitimacy, true);
                meter_bar(ui, "Democracy Index", state.democracy_index, true);
                meter_bar(ui, "Civil Liberties", state.civil_liberties, true);
                meter_bar(ui, "Press Freedom", state.press_freedom, true);
                meter_bar(ui, "Rule of Law", state.rule_of_law, true);
                meter_bar(ui, "State Corruption", state.corruption, false);
                meter_bar(ui, "Military Coup Risk", state.coup_risk, false);

                ui.separator();
                ui.label(
                    RichText::new("EXECUTIVE DECREES & EMERGENCY POWERS")
                        .color(TEXT_MUTED)
                        .size(12.0)
                        .strong(),
                );
                ui.label(
                    RichText::new("Toggling decrees immediately reshapes liberties, faction loyalty, and stability.")
                        .color(TEXT_MUTED)
                        .size(11.0),
                );
                ui.add_space(4.0);

                if ui
                    .checkbox(
                        &mut state.flags.martial_law,
                        "Martial Law (Curbs insurgency, crushes liberties & rule of law)",
                    )
                    .changed()
                {
                    state.log(
                        LogCategory::Political,
                        if state.flags.martial_law {
                            "Martial Law Declared"
                        } else {
                            "Martial Law Lifted"
                        },
                        "Executive order modified emergency military policing.",
                    );
                }
                if ui
                    .checkbox(
                        &mut state.flags.press_censorship,
                        "State Press Censorship (Controls narrative, angers Intelligentsia)",
                    )
                    .changed()
                {
                    state.log(
                        LogCategory::Political,
                        if state.flags.press_censorship {
                            "Media Censorship Enacted"
                        } else {
                            "Press Freedom Restored"
                        },
                        "Information ministry updated broadcast regulations.",
                    );
                }
                if ui
                    .checkbox(
                        &mut state.flags.secret_police,
                        "Secret Police Directorate (Deters coups, increases corruption & fear)",
                    )
                    .changed()
                {
                    state.log(
                        LogCategory::Political,
                        if state.flags.secret_police {
                            "Secret Police Authorized"
                        } else {
                            "Secret Police Disbanded"
                        },
                        "Internal security charter revised.",
                    );
                }
                if ui
                    .checkbox(
                        &mut state.flags.opposition_banned,
                        "Outlaw Political Opposition (Consolidates party rule, hurts democracy)",
                    )
                    .changed()
                {
                    state.log(
                        LogCategory::Political,
                        if state.flags.opposition_banned {
                            "Opposition Parties Banned"
                        } else {
                            "Opposition Legalized"
                        },
                        "Electoral registry altered by decree.",
                    );
                }
                if ui
                    .checkbox(
                        &mut state.flags.term_limits_abolished,
                        "Abolish Term Limits & Suspend Elections",
                    )
                    .changed()
                {
                    if state.flags.term_limits_abolished && state.regime.is_democratic() {
                        state.regime = RegimeType::AbsoluteAutocracy;
                        *status_msg =
                            Some("Abolishing elections transitioned regime to Absolute Autocracy!".into());
                    }
                }
                ui.checkbox(
                    &mut state.flags.independent_judiciary,
                    "Independent Judiciary (Boosts Rule of Law & fights Corruption)",
                );
                state.coup_risk = calculate_coup_risk(state);
            });
        },
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("DOMESTIC POWER BLOCS & INTEREST GROUPS")
                        .color(TEXT_MUTED)
                        .size(12.0)
                        .strong(),
                );
                ui.label(
                    RichText::new(
                        "Powerful factions with low approval trigger strikes, protests, insurgencies, or military coups.",
                    )
                    .color(TEXT_MUTED)
                    .size(11.0),
                );
                ui.add_space(6.0);

                let mut action_to_run: Option<(FactionId, bool)> = None;
                for f in &state.factions {
                    panel_frame().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(RichText::new(&f.name).color(TEXT_PRIMARY).size(13.0).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let app_col = severity_color(f.approval, true);
                                badge(ui, &format!("APP {:.0}%", f.approval), app_col);
                                badge(ui, &format!("CLOUT {:.0}%", f.clout), ACCENT_BLUE);
                            });
                        });
                        ui.label(RichText::new(&f.description).color(TEXT_MUTED).size(11.0));
                        ui.add_space(3.0);
                        ui.horizontal_wrapped(|ui| {
                            if ui.small_button("🤝 Subsidize / Appease (8 PC, $5B)").clicked() {
                                action_to_run = Some((f.id, true));
                            }
                            if ui.small_button("⛓ Suppress / Marginalize (10 PC)").clicked() {
                                action_to_run = Some((f.id, false));
                            }
                        });
                    });
                    ui.add_space(4.0);
                }

                if let Some((fid, appease)) = action_to_run {
                    if appease {
                        if state.political_capital >= 8.0 && state.treasury_bn >= 5.0 {
                            state.political_capital -= 8.0;
                            state.treasury_bn -= 5.0;
                            state.adjust_faction(fid, 12.0, 4.0);
                            let fname = state.faction(fid).name.clone();
                            state.log(
                                LogCategory::Political,
                                format!("Appeased {}", fname),
                                "Targeted patronage and concessions raised faction loyalty.",
                            );
                            *status_msg = Some(format!("Appeased {} (+12% approval).", fname));
                        } else {
                            *status_msg = Some("Need 8 PC and $5B Treasury to appease faction.".into());
                        }
                    } else if state.political_capital >= 10.0 {
                        state.political_capital -= 10.0;
                        state.adjust_faction(fid, -10.0, -11.0);
                        state.civil_liberties = (state.civil_liberties - 4.0).max(4.0);
                        let fname = state.faction(fid).name.clone();
                        state.log(
                            LogCategory::Political,
                            format!("Marginalized {}", fname),
                            "Administrative pressure reduced faction institutional clout.",
                        );
                        *status_msg = Some(format!(
                            "Curbed clout of {} (-11% clout, -10% approval).",
                            fname
                        ));
                    } else {
                        *status_msg = Some("Need 10 PC to suppress faction.".into());
                    }
                    state.coup_risk = calculate_coup_risk(state);
                }
            });
        },
    );
}

pub fn render_religion_extremism_tab(
    ui: &mut egui::Ui,
    state: &mut NationState,
    status_msg: &mut Option<String>,
    metrics: &UiMetrics,
) {
    ui.label(
        RichText::new("🕊 RELIGION, IDEOLOGY & COUNTER-EXTREMISM COMMAND")
            .color(ACCENT_AMBER)
            .size(metrics.fs(15.5))
            .strong(),
    );
    ui.add_space(6.0);

    two_col_responsive!(
        ui,
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("STATE-RELIGION CONSTITUTIONAL POLICY")
                        .color(TEXT_MUTED)
                        .size(12.0)
                        .strong(),
                );
                ui.add_space(6.0);

                for &pol in ReligionPolicy::all() {
                    let selected = state.religion_policy == pol;
                    panel_frame().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            let label = if selected {
                                format!("● {} (ACTIVE)", pol.label())
                            } else {
                                pol.label().to_string()
                            };
                            ui.label(
                                RichText::new(label)
                                    .color(if selected { ACCENT_BLUE } else { TEXT_PRIMARY })
                                    .strong(),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if !selected && ui.button("Adopt (16 PC)").clicked() {
                                    if state.political_capital >= 16.0 {
                                        state.political_capital -= 16.0;
                                        state.religion_policy = pol;
                                        state.flags.religious_courts =
                                            pol == ReligionPolicy::TheocraticLaw;
                                        match pol {
                                            ReligionPolicy::StateAtheism => {
                                                state.adjust_faction(FactionId::Clergy, -24.0, -14.0);
                                                state.adjust_faction(FactionId::Nationalists, -10.0, -2.0);
                                            }
                                            ReligionPolicy::SecularPluralism => {
                                                state.adjust_faction(FactionId::Intelligentsia, 14.0, 5.0);
                                                state.sectarian_tension =
                                                    (state.sectarian_tension - 10.0).max(0.0);
                                            }
                                            ReligionPolicy::OfficialReligion => {
                                                state.adjust_faction(FactionId::Clergy, 14.0, 8.0);
                                                state.adjust_faction(FactionId::Nationalists, 8.0, 4.0);
                                            }
                                            ReligionPolicy::TheocraticLaw => {
                                                state.adjust_faction(FactionId::Clergy, 25.0, 18.0);
                                                state.adjust_faction(FactionId::Intelligentsia, -24.0, -8.0);
                                                state.civil_liberties =
                                                    (state.civil_liberties - 16.0).max(5.0);
                                            }
                                        }
                                        state.log(
                                            LogCategory::Religious,
                                            format!("Religious Charter: {}", pol.label()),
                                            pol.description(),
                                        );
                                        *status_msg = Some(format!("Adopted {} policy.", pol.label()));
                                    } else {
                                        *status_msg = Some("Need 16 Political Capital.".into());
                                    }
                                }
                            });
                        });
                        ui.label(RichText::new(pol.description()).color(TEXT_MUTED).size(11.5));
                    });
                    ui.add_space(4.0);
                }

                ui.add_space(6.0);
                meter_bar(ui, "Population Religiosity", state.religiosity, true);
                meter_bar(ui, "Clerical Institutional Power", state.clerical_power, false);
                ui.checkbox(&mut state.flags.religious_courts, "Clerical / Religious Courts Jurisdiction");
            });
        },
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("EXTREMISM, SECTARIANISM & INSURGENCY MONITOR")
                        .color(TEXT_MUTED)
                        .size(12.0)
                        .strong(),
                );
                ui.add_space(6.0);

                meter_bar(ui, "Religious Fundamentalist Extremism", state.religious_extremism, false);
                meter_bar(ui, "Political / Ideological Radicalism", state.ideological_extremism, false);
                meter_bar(ui, "Sectarian & Ethnic Tension", state.sectarian_tension, false);
                meter_bar(ui, "Armed Insurgency Strength", state.insurgency_strength, false);

                ui.add_space(8.0);
                ui.separator();
                ui.label(
                    RichText::new("COUNTER-EXTREMISM OPERATIONS")
                        .color(TEXT_MUTED)
                        .size(12.0)
                        .strong(),
                );
                ui.add_space(6.0);

                ui.checkbox(
                    &mut state.flags.deradicalization_program,
                    "Ongoing Civic Deradicalization Program (-$0.9B/qtr)",
                );
                ui.checkbox(
                    &mut state.flags.mass_surveillance,
                    "Telecom & Biometric Surveillance (-$1.4B/qtr, -Liberties)",
                );
                ui.add_space(6.0);

                if ui.button("⚔ Launch Targeted Special Forces Raids (10 PC, $6B)").clicked() {
                    if state.political_capital >= 10.0 && state.treasury_bn >= 6.0 {
                        state.political_capital -= 10.0;
                        state.treasury_bn -= 6.0;
                        state.insurgency_strength = (state.insurgency_strength - 14.0).max(0.0);
                        state.religious_extremism = (state.religious_extremism - 6.0).max(0.0);
                        state.ideological_extremism = (state.ideological_extremism - 6.0).max(0.0);
                        state.log(
                            LogCategory::Military,
                            "Counter-Extremism Raids",
                            "Special forces dismantled radical safehouses and weapons caches.",
                        );
                        *status_msg = Some("Insurgency reduced by 14%.".into());
                    } else {
                        *status_msg = Some("Need 10 PC and $6B Treasury.".into());
                    }
                }
                ui.add_space(4.0);
                if ui.button("🕊 National Reconciliation & Interfaith Dialogue (12 PC, $4B)").clicked() {
                    if state.political_capital >= 12.0 && state.treasury_bn >= 4.0 {
                        state.political_capital -= 12.0;
                        state.treasury_bn -= 4.0;
                        state.sectarian_tension = (state.sectarian_tension - 15.0).max(0.0);
                        state.religious_extremism = (state.religious_extremism - 8.0).max(0.0);
                        state.stability = (state.stability + 6.0).min(100.0);
                        state.log(
                            LogCategory::Religious,
                            "National Reconciliation Summit",
                            "Moderate leaders signed a peace charter cooling sectarian tension.",
                        );
                        *status_msg = Some("Sectarian tension cooled (-15%).".into());
                    } else {
                        *status_msg = Some("Need 12 PC and $4B Treasury.".into());
                    }
                }
            });
        },
    );
}

pub fn render_military_diplomacy_tab(
    ui: &mut egui::Ui,
    state: &mut NationState,
    status_msg: &mut Option<String>,
    metrics: &UiMetrics,
) {
    ui.label(
        RichText::new("⚔ DEFENSE COMMAND, NUCLEAR DETERRENCE & GEOPOLITICS")
            .color(ACCENT_RED)
            .size(metrics.fs(15.5))
            .strong(),
    );
    ui.add_space(6.0);

    // Military Command Strip
    two_col_responsive!(
        ui,
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("ARMED FORCES READINESS & POSTURE")
                        .color(TEXT_MUTED)
                        .size(12.0)
                        .strong(),
                );
                ui.add_space(4.0);
                let sub_w = ((ui.available_width() - 32.0) * 0.5).clamp(120.0, 220.0);
                ui.horizontal_wrapped(|ui| {
                    kpi_card(
                        ui,
                        "ACTIVE TROOPS",
                        &format!("{:.0}K", state.active_troops_k),
                        if state.flags.conscription {
                            "Conscription ON"
                        } else {
                            "Volunteer Force"
                        },
                        ACCENT_BLUE,
                        sub_w,
                    );
                    kpi_card(
                        ui,
                        "COUP RISK",
                        &format!("{:.0}%", state.coup_risk),
                        &format!("Mil Loyalty {:.0}%", state.faction(FactionId::Military).approval),
                        severity_color(state.coup_risk, false),
                        sub_w,
                    );
                });
                ui.add_space(6.0);
                meter_bar(ui, "Combat Readiness", state.military_readiness, true);
                meter_bar(ui, "Military Technology", state.military_tech, true);
                meter_bar(ui, "War Exhaustion", state.war_exhaustion, false);

                ui.horizontal_wrapped(|ui| {
                    ui.label("Defense Posture:");
                    for &posture in DefensePosture::all() {
                        let active = state.defense_posture == posture;
                        if ui.selectable_label(active, posture.label()).clicked() {
                            state.defense_posture = posture;
                        }
                    }
                });
                ui.checkbox(
                    &mut state.flags.conscription,
                    "Mandatory National Conscription (+45% troop pool, lowers unemployment)",
                );
            });
        },
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("☢ STRATEGIC NUCLEAR PROGRAM")
                        .color(TEXT_MUTED)
                        .size(12.0)
                        .strong(),
                );
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label("Current Tier:");
                    badge(
                        ui,
                        state.nuclear_status.label(),
                        if state.nuclear_status.stage() >= 3 {
                            ACCENT_RED
                        } else {
                            ACCENT_AMBER
                        },
                    );
                });
                ui.add_space(6.0);
                if state.nuclear_status != NuclearStatus::StrategicTriad {
                    meter_bar(ui, "Progress to Next Tier", state.nuclear_progress, true);
                    if ui.button("☢ Fund Nuclear Sprint (+35% Progress, 16 PC, $16B)").clicked() {
                        if state.political_capital >= 16.0 && state.treasury_bn >= 16.0 {
                            state.political_capital -= 16.0;
                            state.treasury_bn -= 16.0;
                            if state.nuclear_status == NuclearStatus::None {
                                state.nuclear_status = NuclearStatus::CivilianResearch;
                                state.nuclear_progress = 20.0;
                            } else {
                                state.nuclear_progress += 35.0;
                                if state.nuclear_progress >= 100.0 {
                                    if let Some(next) = state.nuclear_status.next() {
                                        state.nuclear_status = next;
                                        state.nuclear_progress = 0.0;
                                    }
                                }
                            }
                            if state.nuclear_status.stage() >= 2 {
                                state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 15.0;
                            }
                            state.log(
                                LogCategory::Military,
                                "Nuclear Program Accelerated",
                                format!("Current nuclear status: {}", state.nuclear_status.label()),
                            );
                            *status_msg = Some("Advanced sovereign nuclear program.".into());
                        } else {
                            *status_msg = Some("Need 16 PC and $16B Treasury.".into());
                        }
                    }
                } else {
                    ui.label(
                        RichText::new(
                            "Full Strategic ICBM Triad operational. Massive military deterrence active.",
                        )
                        .color(ACCENT_GREEN),
                    );
                }
                ui.add_space(8.0);
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    if ui.button("🎖 Grand Military Parade (6 PC, $3B)").clicked() {
                        if state.political_capital >= 6.0 && state.treasury_bn >= 3.0 {
                            state.political_capital -= 6.0;
                            state.treasury_bn -= 3.0;
                            state.adjust_faction(FactionId::Military, 8.0, 3.0);
                            state.adjust_faction(FactionId::Nationalists, 7.0, 2.0);
                            state.coup_risk = calculate_coup_risk(state);
                            *status_msg = Some("Military parade boosted Army & Nationalist morale.".into());
                        } else {
                            *status_msg = Some("Need 6 PC and $3B.".into());
                        }
                    }
                    if ui.button("💵 Raise Officer Pay (+10% Mil Loyalty, $6B)").clicked() {
                        if state.treasury_bn >= 6.0 {
                            state.treasury_bn -= 6.0;
                            state.adjust_faction(FactionId::Military, 10.0, 2.0);
                            state.coup_risk = calculate_coup_risk(state);
                            *status_msg = Some("Officer bonuses reduced coup risk.".into());
                        } else {
                            *status_msg = Some("Need $6B Treasury.".into());
                        }
                    }
                });
            });
        },
    );

    ui.add_space(10.0);

    // Foreign Powers Diplomatic Matrix with Scalable Vector Flags & War Frontline Bars
    card_frame().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new("🌍 GLOBAL POWERS & BILATERAL DIPLOMACY")
                    .color(ACCENT_CYAN)
                    .size(metrics.fs(12.5))
                    .strong(),
            );
            if state.annexed_count() > 0 || state.puppet_count() > 0 {
                ui.add_space(8.0);
                badge_scaled(
                    ui,
                    &format!("{} ANNEXED • {} CLIENT", state.annexed_count(), state.puppet_count()),
                    ACCENT_GOLD,
                    metrics.fs(10.5),
                );
            }
        });
        ui.add_space(6.0);

        let mut pending_dip: Option<(ForeignPowerId, &'static str)> = None;
        let annex_cost = if state.flags.ledger_rebalance { 10 } else { 24 };
        let puppet_cost = if state.flags.ledger_rebalance { 8 } else { 18 };
        let fp_flag_h = (metrics.flag_h * 0.98).clamp(22.0, 42.0);
        let fp_flag_w = fp_flag_h * 1.55;

        for p in &state.foreign_powers {
            let border = if p.annexed {
                BORDER_GOLD
            } else if p.at_war {
                ACCENT_RED
            } else if p.puppet_state {
                ACCENT_PURPLE
            } else {
                BORDER_SUBTLE
            };
            egui::Frame::new()
                .fill(BG_PANEL)
                .stroke(Stroke::new(if p.at_war || p.annexed { 1.3_f32 } else { 1.0_f32 }, border))
                .corner_radius(egui::CornerRadius::same(7))
                .inner_margin(egui::Margin::same(10))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        draw_foreign_flag(ui, p, fp_flag_w, fp_flag_h);
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(&p.name)
                                .color(if p.annexed { ACCENT_GOLD } else { TEXT_PRIMARY })
                                .size(metrics.fs(13.5))
                                .strong(),
                        );
                        ui.label(
                            RichText::new(format!("({})", p.ideology))
                                .color(TEXT_MUTED)
                                .size(metrics.fs(11.2)),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if p.annexed {
                                badge_scaled(ui, "★ ANNEXED TERRITORY", ACCENT_GOLD, metrics.fs(10.5));
                            } else {
                                if p.puppet_state {
                                    badge_scaled(ui, "👑 CLIENT STATE", ACCENT_PURPLE, metrics.fs(10.5));
                                }
                                if p.at_war {
                                    badge_scaled(
                                        ui,
                                        &format!("⚔ AT WAR • {:+.0}", p.war_score),
                                        ACCENT_RED,
                                        metrics.fs(10.5),
                                    );
                                }
                                if p.sanctioning_us {
                                    badge_scaled(ui, "SANCTIONING US", ACCENT_RED, metrics.fs(10.0));
                                }
                                if p.defense_alliance {
                                    badge_scaled(ui, "🛡 ALLY", ACCENT_GREEN, metrics.fs(10.0));
                                }
                                if p.trade_pact {
                                    badge_scaled(ui, "TRADE PACT", ACCENT_BLUE, metrics.fs(10.0));
                                }
                                if p.proxy_funding {
                                    badge_scaled(ui, "PROXY OPS", ACCENT_AMBER, metrics.fs(10.0));
                                }
                                signed_relation_bar(ui, p.relation);
                                ui.label(
                                    RichText::new(format!("Rel {:+.0}", p.relation))
                                        .color(TEXT_PRIMARY)
                                        .size(12.0)
                                        .strong(),
                                );
                            }
                        });
                    });

                    // Visual Tug-of-War Frontline Bar during Active War!
                    if p.at_war && !p.annexed {
                        ui.add_space(6.0);
                        draw_war_frontline_bar(ui, &state.nation_name, &p.short_name, p.war_score);
                    }

                    ui.add_space(5.0);
                    if p.annexed {
                        ui.label(
                            RichText::new(format!(
                                "Sovereign province incorporated into {} — population, GDP, and strategic assets integrated.",
                                state.nation_name
                            ))
                            .color(ACCENT_GREEN)
                            .size(metrics.fs(11.5)),
                        );
                    } else {
                        ui.horizontal_wrapped(|ui| {
                            let extra = if p.puppet_state {
                                format!(" | Tribute +${:.1}B/qtr", p.economic_power * 0.22)
                            } else {
                                String::new()
                            };
                            ui.label(
                                RichText::new(format!(
                                    "Mil {:.0} | Econ {:.0}{}",
                                    p.military_power, p.economic_power, extra
                                ))
                                .color(TEXT_MUTED)
                                .size(11.5),
                            );
                            ui.add_space(8.0);
                            if p.puppet_state {
                                if ui.small_button("Improve Ties (6 PC)").clicked() {
                                    pending_dip = Some((p.id, "improve"));
                                }
                                if ui
                                    .small_button(
                                        RichText::new(format!("👑 Annex Client State ({} PC)", annex_cost))
                                            .color(ACCENT_GOLD)
                                            .strong(),
                                    )
                                    .clicked()
                                {
                                    pending_dip = Some((p.id, "annex_country"));
                                }
                            } else if !p.at_war {
                                if ui.small_button("Improve Ties (6 PC)").clicked() {
                                    pending_dip = Some((p.id, "improve"));
                                }
                                if ui
                                    .small_button(if p.trade_pact {
                                        "End Trade (12 PC)"
                                    } else {
                                        "Trade Pact (12 PC)"
                                    })
                                    .clicked()
                                {
                                    pending_dip = Some((p.id, "trade_pact"));
                                }
                                if ui
                                    .small_button(if p.defense_alliance {
                                        "Break Alliance"
                                    } else {
                                        "Defense Treaty (18 PC)"
                                    })
                                    .clicked()
                                {
                                    pending_dip = Some((p.id, "defense_alliance"));
                                }
                                if ui
                                    .small_button(if p.we_sanction_them {
                                        "Lift Sanctions"
                                    } else {
                                        "Sanction (8 PC)"
                                    })
                                    .clicked()
                                {
                                    pending_dip = Some((p.id, "sanction"));
                                }
                                if ui.small_button("Espionage ($6B, 14 PC)").clicked() {
                                    pending_dip = Some((p.id, "espionage"));
                                }
                                if p.id != ForeignPowerId::GlobalMonetaryCouncil {
                                    if ui
                                        .small_button(if p.proxy_funding {
                                            "Stop Proxies"
                                        } else {
                                            "Fund Rebels ($8B)"
                                        })
                                        .clicked()
                                    {
                                        pending_dip = Some((p.id, "fund_proxies"));
                                    }
                                    if ui
                                        .small_button(
                                            RichText::new("⚔ Declare War (20 PC)").color(ACCENT_RED).strong(),
                                        )
                                        .clicked()
                                    {
                                        pending_dip = Some((p.id, "declare_war"));
                                    }
                                }
                            } else {
                                if ui
                                    .small_button(
                                        RichText::new("🕊 Negotiate Peace Treaty (12 PC)").color(ACCENT_BLUE),
                                    )
                                    .clicked()
                                {
                                    pending_dip = Some((p.id, "sue_peace"));
                                }
                                if p.war_score >= 65.0 {
                                    if ui
                                        .small_button(
                                            RichText::new(format!("👑 Install Puppet ({} PC)", puppet_cost))
                                                .color(ACCENT_PURPLE)
                                                .strong(),
                                        )
                                        .clicked()
                                    {
                                        pending_dip = Some((p.id, "install_puppet"));
                                    }
                                }
                                if p.war_score >= 70.0 {
                                    if ui
                                        .small_button(
                                            RichText::new(format!("★ Conquer & Annex ({} PC)", annex_cost))
                                                .color(ACCENT_GOLD)
                                                .strong(),
                                        )
                                        .clicked()
                                    {
                                        pending_dip = Some((p.id, "annex_country"));
                                    }
                                } else {
                                    ui.add_enabled(
                                        false,
                                        egui::Button::new("Annex (Need +70 War Score)").small(),
                                    );
                                }
                            }
                        });
                    }
                });
            ui.add_space(5.0);
        }

        if let Some((pid, act)) = pending_dip {
            match diplomatic_action(state, pid, act) {
                Ok(msg) => *status_msg = Some(msg),
                Err(err) => *status_msg = Some(format!("Action Blocked: {}", err)),
            }
        }
    });
}

pub fn render_decisions_tab(
    ui: &mut egui::Ui,
    state: &mut NationState,
    status_msg: &mut Option<String>,
    metrics: &UiMetrics,
) {
    ui.label(
        RichText::new("📜 ROYAL DECREES, STRUCTURAL REFORMS & IDEOLOGICAL SHIFTS")
            .color(ACCENT_BLUE)
            .size(metrics.fs(15.5))
            .strong(),
    );
    ui.label(
        RichText::new(
            "Pivot between Capitalism, Communism, Democracy, Autocracy, and Theocracy — or launch strategic programs.",
        )
        .color(TEXT_MUTED)
        .size(metrics.fs(12.0)),
    );
    ui.add_space(8.0);

    let defs = all_decisions();
    let mut to_enact: Option<&'static str> = None;

    for def in &defs {
        let check = can_enact(state, def);
        let cat_col = match def.category {
            LogCategory::Economic => ACCENT_GREEN,
            LogCategory::Political => ACCENT_BLUE,
            LogCategory::Military => ACCENT_RED,
            LogCategory::Religious => ACCENT_PURPLE,
            _ => ACCENT_AMBER,
        };
        card_frame().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                badge(ui, def.category.label(), cat_col);
                ui.label(
                    RichText::new(def.name)
                        .color(TEXT_PRIMARY)
                        .size(metrics.fs(13.5))
                        .strong(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| match &check {
                    Ok(()) => {
                        if ui
                            .button(
                                RichText::new(format!(
                                    "▶ ENACT ({:.0} PC | ${:.0}B)",
                                    def.political_cost, def.treasury_cost_bn
                                ))
                                .color(ACCENT_GREEN)
                                .strong(),
                            )
                            .clicked()
                        {
                            to_enact = Some(def.id);
                        }
                    }
                    Err(reason) => {
                        ui.add_enabled(false, egui::Button::new(reason));
                    }
                });
            });
            ui.add_space(3.0);
            ui.label(RichText::new(def.description).color(TEXT_PRIMARY).size(12.0));
            ui.label(
                RichText::new(format!("Effects: {}", def.effects_summary))
                    .color(TEXT_MUTED)
                    .size(11.5),
            );
        });
        ui.add_space(6.0);
    }

    if let Some(id) = to_enact {
        match enact_decision(state, id) {
            Ok(msg) => *status_msg = Some(msg),
            Err(err) => *status_msg = Some(err),
        }
    }
}

pub fn render_chronicles_tab(
    ui: &mut egui::Ui,
    state: &NationState,
    filter: &mut Option<LogCategory>,
    metrics: &UiMetrics,
) {
    ui.label(
        RichText::new("📖 IMPERIAL CHRONICLES & HISTORICAL ARCHIVES")
            .color(ACCENT_BLUE)
            .size(metrics.fs(15.5))
            .strong(),
    );
    ui.add_space(6.0);

    ui.horizontal_wrapped(|ui| {
        if ui.selectable_label(filter.is_none(), "All Dispatches").clicked() {
            *filter = None;
        }
        for cat in [
            LogCategory::Political,
            LogCategory::Economic,
            LogCategory::Military,
            LogCategory::Religious,
            LogCategory::Crisis,
            LogCategory::Diplomacy,
        ] {
            if ui.selectable_label(*filter == Some(cat), cat.label()).clicked() {
                *filter = Some(cat);
            }
        }
    });
    ui.add_space(6.0);

    card_frame().show(ui, |ui| {
        for entry in &state.logs {
            if let Some(f) = filter {
                if entry.category != *f {
                    continue;
                }
            }
            let col = match entry.category {
                LogCategory::Crisis => ACCENT_RED,
                LogCategory::Military => ACCENT_AMBER,
                LogCategory::Economic => ACCENT_GREEN,
                LogCategory::Religious => ACCENT_PURPLE,
                _ => ACCENT_BLUE,
            };
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(format!("[{}]", entry.date_label)).color(TEXT_MUTED).size(11.5));
                badge(ui, entry.category.label(), col);
                ui.label(RichText::new(&entry.headline).color(TEXT_PRIMARY).size(12.5).strong());
            });
            ui.label(RichText::new(&entry.detail).color(TEXT_MUTED).size(11.5));
            ui.separator();
        }
    });
}
