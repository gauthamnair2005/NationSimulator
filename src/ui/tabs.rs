use crate::decisions::{all_decisions, can_enact, enact_decision};
use crate::models::*;
use crate::simulation::{
    calculate_coup_risk, calculate_fiscal, compute_credit_rating, diplomatic_action,
    run_strategic_activity, set_lockdown_level, spawn_domestic_rebel_state,
    trigger_pandemic_outbreak, zero_all_taxes,
};
use crate::ui::widgets::*;
use eframe::egui::{self, Color32, RichText, Stroke};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardTab {
    Overview,
    Economy,
    Politics,
    ReligionExtremism,
    MilitaryDiplomacy,
    ProgramsOps,
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
            Self::ProgramsOps,
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
            Self::ProgramsOps => "🎯  Programs & Ops",
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
            .color(accent_gold())
            .size(metrics.fs(15.5))
            .strong(),
    );
    ui.add_space(6.0);

    // Alerts Banner
    let mut alerts: Vec<(Color32, String)> = Vec::new();
    if state.coup_risk >= 50.0 {
        alerts.push((accent_red(), format!("CRITICAL COUP RISK ({:.0}%): Military/Security loyalty dangerously low!", state.coup_risk)));
    }
    if state.total_extremism() >= 45.0 || state.insurgency_strength >= 40.0 {
        alerts.push((accent_red(), format!("EXTREMISM ALERT: Insurgency at {:.0}%, Extremism index at {:.0}%.", state.insurgency_strength, state.total_extremism())));
    }
    if state.active_wars_count() > 0 {
        alerts.push((accent_amber(), format!("ACTIVE CONFLICT: Armed forces engaged in {} war(s). War Exhaustion: {:.0}%.", state.active_wars_count(), state.war_exhaustion)));
    }
    if state.inflation >= 10.0 {
        alerts.push((accent_amber(), format!("INFLATION SURGE ({:.1}%): Consider raising Central Bank interest rates or curbing deficits.", state.inflation)));
    }
    if state.imf_austerity_turns > 0 {
        alerts.push((accent_amber(), format!("GMC AUSTERITY MANDATE: {} quarter(s) remaining.", state.imf_austerity_turns)));
    }
    if state.regime.is_democratic() && !state.flags.term_limits_abolished {
        alerts.push((accent_blue(), format!("NEXT GENERAL ELECTION: {} quarter(s) remaining.", state.quarters_until_election)));
    }
    if state.is_global_hegemon() {
        alerts.push((accent_gold(), format!("GLOBAL HEGEMONY: All rival sovereign powers have been annexed or vassalized by {}!", state.nation_name)));
    }
    if state.flags.ledger_rebalance {
        let g_surplus = if state.god_surplus_bn > 0.0 { state.god_surplus_bn } else { 1000.0 };
        alerts.push((
            accent_amber(),
            format!(
                "GOLDEN MANDATE ACTIVE: Sovereign debt cleared, {}/qtr surplus (click nation name for 50x), 0% tax ceiling & Golden Era incidents enabled.",
                fmt_signed_billions(g_surplus)
            ),
        ));
    }

    if !alerts.is_empty() {
        card_frame().show(ui, |ui| {
            ui.label(RichText::new("⚡ ACTIVE SITUATION ADVISORIES").color(text_muted()).size(metrics.fs(11.0)).strong());
            ui.add_space(4.0);
            for (col, msg) in alerts {
                ui.horizontal_wrapped(|ui| {
                    badge_scaled(ui, "ALERT", col, metrics.fs(10.0));
                    ui.label(RichText::new(msg).color(text_primary()).size(metrics.fs(12.2)));
                });
            }
        });
        ui.add_space(10.0);
    }

    // Interactive Grand-Strategy World Theater Chart
    let clicked_power = draw_world_theater_map(ui, state, metrics);
    ui.add_space(10.0);

    // Achievements / Trophies Bar
    card_frame().show(ui, |ui| {
        let achievements = compute_achievements(state);
        let unlocked_n = achievements.iter().filter(|a| a.3).count();
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("🏆 SOVEREIGN MILESTONES & TROPHIES").color(accent_gold()).size(metrics.fs(12.0)).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                badge_scaled(ui, &format!("{}/{} UNLOCKED", unlocked_n, achievements.len()), accent_gold(), metrics.fs(10.5));
            });
        });
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            for (icon, name, desc, unlocked, col) in achievements {
                let display_col = if unlocked { col } else { text_muted() };
                let label = if unlocked { format!("{} {}", icon, name) } else { format!("🔒 {}", name) };
                let tip = if unlocked { format!("UNLOCKED: {}", desc) } else { format!("LOCKED: {}", desc) };
                draw_status_flag_chip(ui, if unlocked { "★" } else { "•" }, &label, &tip, display_col, metrics.scale * 0.94);
            }
        });
    });
    ui.add_space(10.0);

    // Responsive Trend Charts Grid
    let gdp_hist: Vec<f64> = state.history.iter().map(|h| h.gdp_bn).collect();
    let app_hist: Vec<f64> = state.history.iter().map(|h| h.approval).collect();
    let stab_hist: Vec<f64> = state.history.iter().map(|h| h.stability).collect();
    let inf_hist: Vec<f64> = state.history.iter().map(|h| h.inflation).collect();
    let coup_hist: Vec<f64> = state.history.iter().map(|h| h.coup_risk).collect();
    let ext_hist: Vec<f64> = state.history.iter().map(|h| h.extremism).collect();

    let charts: [(&str, &Vec<f64>, &str, Color32); 6] = [
        ("REAL GDP ($B)", &gdp_hist, "B", accent_blue()),
        ("PUBLIC APPROVAL", &app_hist, "%", accent_green()),
        ("REGIME STABILITY", &stab_hist, "%", accent_purple()),
        ("INFLATION RATE", &inf_hist, "%", accent_amber()),
        ("MILITARY COUP RISK", &coup_hist, "%", accent_red()),
        ("EXTREMISM INDEX", &ext_hist, "%", accent_red()),
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

    // National Profile & Recent Dispatches
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
                        ui.label(RichText::new("NATIONAL PROFILE & LEADER DOSSIER").color(accent_cyan()).size(metrics.fs(12.0)).strong());
                        ui.label(
                            RichText::new(format!(
                                "{} {}  (Age {} • {})",
                                state.leader_title,
                                state.leader_name,
                                state.leader_profile.age,
                                state.leader_profile.background.label()
                            ))
                            .color(text_primary())
                            .size(metrics.fs(13.8))
                            .strong(),
                        );
                        ui.label(
                            RichText::new(format!(
                                "🎓 {}  •  “{}”",
                                state.leader_profile.education, state.leader_profile.motto
                            ))
                            .color(accent_gold())
                            .size(metrics.fs(11.3))
                            .italics(),
                        );
                        if !state.leader_profile.bio.trim().is_empty() {
                            ui.label(
                                RichText::new(&state.leader_profile.bio)
                                    .color(text_muted())
                                    .size(metrics.fs(11.0)),
                            );
                        }
                        ui.label(
                            RichText::new(format!(
                                "Pop: {:.2}M  •  Credit: {}  •  Debt/GDP: {:.1}%  •  Gini: {:.1}",
                                state.population_m, state.credit_rating, state.debt_to_gdp(), state.gini
                            ))
                            .color(text_muted())
                            .size(metrics.fs(11.2)),
                        );
                    });
                });

                if state.annexed_count() > 0 || state.puppet_count() > 0 {
                    ui.add_space(6.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Empire Territories:").color(text_muted()));
                        ui.label(
                            RichText::new(format!("{} Annexed • {} Client State(s)", state.annexed_count(), state.puppet_count()))
                                .color(accent_gold())
                                .strong(),
                        );
                    });
                    if !state.annexed_territories.is_empty() {
                        ui.label(RichText::new(format!("Provinces: {}", state.annexed_territories.join(", "))).color(text_muted()).size(11.5));
                    }
                }

                ui.add_space(8.0);
                meter_bar(ui, "Democracy Index", state.democracy_index, true);
                meter_bar(ui, "Rule of Law", state.rule_of_law, true);
                meter_bar(ui, "Civil Liberties", state.civil_liberties, true);
                meter_bar(ui, "Corruption Level", state.corruption, false);
                meter_bar(ui, "Healthcare & Education Avg", (state.healthcare_level + state.education_level) * 0.5, true);
                meter_bar(ui, "Technology & Infrastructure", (state.tech_level + state.infrastructure_level) * 0.5, true);
            });
        },
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(RichText::new("📡 RECENT INTELLIGENCE DISPATCHES").color(accent_cyan()).size(metrics.fs(12.0)).strong());
                ui.add_space(6.0);
                for entry in state.logs.iter().take(7) {
                    let col = match entry.category {
                        LogCategory::Crisis => accent_red(),
                        LogCategory::Military => accent_amber(),
                        LogCategory::Economic => accent_green(),
                        LogCategory::Religious => accent_purple(),
                        _ => accent_blue(),
                    };
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(&entry.date_label).color(text_muted()).size(11.0));
                        badge(ui, entry.category.label(), col);
                        ui.label(RichText::new(&entry.headline).color(text_primary()).size(12.0).strong());
                    });
                    ui.label(RichText::new(&entry.detail).color(text_muted()).size(11.5));
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
    ui.label(RichText::new("💰 MACROECONOMIC COMMAND & TREASURY").color(accent_green()).size(metrics.fs(15.5)).strong());
    ui.add_space(6.0);

    let fiscal = calculate_fiscal(state);
    state.last_fiscal = fiscal.clone();

    let avail_w = ui.available_width().max(380.0);
    let cols = if avail_w >= 880.0 { 5 } else if avail_w >= 540.0 { 3 } else { 2 };
    let card_w = ((avail_w - (cols as f32 - 1.0) * 8.0) / cols as f32 - 22.0).max(125.0);

    ui.horizontal_wrapped(|ui| {
        kpi_card(ui, "NOMINAL GDP", &fmt_billions(state.gdp_bn), &format!("Growth {:+.2}%/yr", state.gdp_growth), severity_color(state.gdp_growth * 15.0 + 40.0, true), card_w);
        kpi_card(ui, "QUARTERLY BALANCE", &fmt_signed_billions(fiscal.net_balance), &format!("Rev {} / Exp {}", fmt_billions(fiscal.total_revenue), fmt_billions(fiscal.total_spending)), if fiscal.net_balance >= 0.0 { accent_green() } else { accent_red() }, card_w);
        kpi_card(ui, "NATIONAL DEBT", &fmt_billions(state.national_debt_bn), &format!("{:.1}% GDP • {}", state.debt_to_gdp(), state.credit_rating), severity_color(state.debt_to_gdp() * 0.7, false), card_w);
        kpi_card(ui, "INFLATION", &format!("{:.1}%", state.inflation), &format!("Central Rate {:.1}%", state.taxes.interest_rate), severity_color(state.inflation * 4.5, false), card_w);
        kpi_card(ui, "ARMS EXPORTS", &fmt_billions(state.armed_forces.total_arms_sales_bn), &format!("{} Contracts • Unemp {:.1}%", state.armed_forces.weapons_contracts_signed, state.unemployment), accent_cyan(), card_w);
    });

    ui.add_space(10.0);

    two_col_responsive!(
        ui,
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(RichText::new("ECONOMIC MODEL & OWNERSHIP").color(text_muted()).size(12.0).strong());
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label("Current System:");
                    badge(ui, state.economy.label(), accent_blue());
                });
                ui.label(RichText::new("Switch economic models structurally via Royal Decrees or adjust levers below:").color(text_muted()).size(11.5));
                ui.add_space(6.0);

                if ui.button("⚔ Switch to Total War Economy (15 PC)").clicked() {
                    if state.has_pc(15.0) {
                        state.spend_pc(15.0);
                        state.economy = EconomicSystem::WarEconomy;
                        state.defense_posture = DefensePosture::FullMobilization;
                        state.log(LogCategory::Economic, "Total War Economy Proclaimed", "Civilian factories converted to defense production.");
                        *status_msg = Some("Shifted to Total War Economy.".into());
                    } else {
                        *status_msg = Some("Need 15 Political Capital.".into());
                    }
                }

                ui.separator();
                ui.label(RichText::new("TAXATION & MONETARY POLICY").color(text_muted()).size(12.0).strong());
                ui.add_space(4.0);

                let min_tax = if state.flags.ledger_rebalance { 0.0 } else { 5.0 };
                let min_rate = if state.flags.ledger_rebalance { 0.0 } else { 0.5 };
                ui.add(egui::Slider::new(&mut state.taxes.income_tax, min_tax..=70.0).text("Income Tax (%)").fixed_decimals(1));
                ui.add(egui::Slider::new(&mut state.taxes.corporate_tax, min_tax..=65.0).text("Corporate Tax (%)").fixed_decimals(1));
                ui.add(egui::Slider::new(&mut state.taxes.tariffs, 0.0..=60.0).text("Import Tariffs (%)").fixed_decimals(1));
                ui.add(egui::Slider::new(&mut state.taxes.wealth_tax, 0.0..=30.0).text("Wealth / Capital Tax (%)").fixed_decimals(1));
                ui.add(egui::Slider::new(&mut state.taxes.state_ownership, 0.0..=100.0).text("State Enterprise Share (%)").fixed_decimals(0));
                ui.add(egui::Slider::new(&mut state.taxes.interest_rate, min_rate..=25.0).text("Central Bank Rate (%)").fixed_decimals(1));

                if state.flags.ledger_rebalance {
                    ui.add_space(4.0);
                    if ui.button(RichText::new("⭐ Set All Taxes & Rates to 0% (Golden Mandate)").color(accent_gold()).strong()).clicked() {
                        zero_all_taxes(state);
                        *status_msg = Some("All taxes & central bank interest rate set to 0.0%!".into());
                    }
                }

                ui.add_space(8.0);
                ui.checkbox(&mut state.flags.capital_controls, "Capital Controls (Blocks flight, angers Capitalists & markets)");

                ui.separator();
                ui.label(RichText::new("QUARTERLY REVENUE BREAKDOWN").color(text_muted()).size(11.5).strong());
                ui.label(format!("Income Tax: {}  |  Corporate Tax: {}", fmt_billions(fiscal.income_rev), fmt_billions(fiscal.corp_rev)));
                ui.label(format!("Tariffs: {}  |  Wealth Tax: {}  |  State Enterprises: {}", fmt_billions(fiscal.tariff_rev), fmt_billions(fiscal.wealth_rev), fmt_billions(fiscal.soe_rev)));
                ui.label(RichText::new(format!("Quarterly Debt Servicing Interest: -{}", fmt_billions(fiscal.debt_interest))).color(accent_amber()));
            });
        },
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(RichText::new("PUBLIC BUDGET ALLOCATIONS (INTENSITY 0-100)").color(text_muted()).size(12.0).strong());
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
                ui.label(RichText::new("TREASURY & DEBT OPERATIONS").color(text_muted()).size(12.0).strong());
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
                        state.log(LogCategory::Economic, "Central Bank Monetized Deficit", "Printed $20B in liquidity, stoking inflation.");
                        *status_msg = Some("Printed $20B liquidity (+3.5% inflation).".into());
                    }
                });
                ui.add_space(6.0);
                ui.label(RichText::new(format!("Trade Balance: {:+.1}B / qtr  |  Gini: {:.1}", state.trade_balance_bn, state.gini)).color(if state.trade_balance_bn >= 0.0 { accent_green() } else { accent_amber() }));
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
    ui.label(RichText::new("🏛 POLITICAL ARCHITECTURE, DECREES & DOMESTIC FACTIONS").color(accent_purple()).size(metrics.fs(15.5)).strong());
    ui.add_space(6.0);

    two_col_responsive!(
        ui,
        |ui| {
            card_frame().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("REGIME TYPE:").color(text_muted()).strong());
                    badge(ui, state.regime.label(), accent_purple());
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
                ui.label(RichText::new("EXECUTIVE DECREES & EMERGENCY POWERS").color(text_muted()).size(12.0).strong());
                ui.add_space(4.0);

                if ui.checkbox(&mut state.flags.martial_law, "Martial Law (Curbs insurgency, crushes liberties & rule of law)").changed() {
                    state.log(LogCategory::Political, if state.flags.martial_law { "Martial Law Declared" } else { "Martial Law Lifted" }, "Executive order modified emergency military policing.");
                }
                if ui.checkbox(&mut state.flags.press_censorship, "State Press Censorship (Controls narrative, angers Intelligentsia)").changed() {
                    state.log(LogCategory::Political, if state.flags.press_censorship { "Media Censorship Enacted" } else { "Press Freedom Restored" }, "Information ministry updated broadcast regulations.");
                }
                if ui.checkbox(&mut state.flags.secret_police, "Secret Police Directorate (Deters coups, increases corruption & fear)").changed() {
                    state.log(LogCategory::Political, if state.flags.secret_police { "Secret Police Authorized" } else { "Secret Police Disbanded" }, "Internal security charter revised.");
                }
                if ui.checkbox(&mut state.flags.opposition_banned, "Outlaw Political Opposition (Consolidates party rule, hurts democracy)").changed() {
                    state.log(LogCategory::Political, if state.flags.opposition_banned { "Opposition Parties Banned" } else { "Opposition Legalized" }, "Electoral registry altered by decree.");
                }
                if ui.checkbox(&mut state.flags.term_limits_abolished, "Abolish Term Limits & Suspend Elections").changed() {
                    if state.flags.term_limits_abolished && state.regime.is_democratic() {
                        state.regime = RegimeType::AbsoluteAutocracy;
                        *status_msg = Some("Abolishing elections transitioned regime to Absolute Autocracy!".into());
                    }
                }
                ui.checkbox(&mut state.flags.independent_judiciary, "Independent Judiciary (Boosts Rule of Law & fights Corruption)");
                state.coup_risk = calculate_coup_risk(state);
            });
        },
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(RichText::new("DOMESTIC POWER BLOCS & INTEREST GROUPS").color(text_muted()).size(12.0).strong());
                ui.label(RichText::new("Powerful factions with low approval trigger strikes, protests, insurgencies, or coups.").color(text_muted()).size(11.0));
                ui.add_space(6.0);

                let mut action_to_run: Option<(FactionId, bool)> = None;
                for f in &state.factions {
                    panel_frame().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(RichText::new(&f.name).color(text_primary()).size(13.0).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let app_col = severity_color(f.approval, true);
                                badge(ui, &format!("APP {:.0}%", f.approval), app_col);
                                badge(ui, &format!("CLOUT {:.0}%", f.clout), accent_blue());
                            });
                        });
                        ui.label(RichText::new(&f.description).color(text_muted()).size(11.0));
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
                        if state.has_pc(8.0) && state.treasury_bn >= 5.0 {
                            state.spend_pc(8.0);
                            state.treasury_bn -= 5.0;
                            state.adjust_faction(fid, 12.0, 4.0);
                            let fname = state.faction(fid).name.clone();
                            state.log(LogCategory::Political, format!("Appeased {}", fname), "Targeted patronage and concessions raised faction loyalty.");
                            *status_msg = Some(format!("Appeased {} (+12% approval).", fname));
                        } else {
                            *status_msg = Some("Need 8 PC and $5B Treasury to appease faction.".into());
                        }
                    } else if state.has_pc(10.0) {
                        state.spend_pc(10.0);
                        state.adjust_faction(fid, -10.0, -11.0);
                        state.civil_liberties = (state.civil_liberties - 4.0).max(4.0);
                        let fname = state.faction(fid).name.clone();
                        state.log(LogCategory::Political, format!("Marginalized {}", fname), "Administrative pressure reduced faction institutional clout.");
                        *status_msg = Some(format!("Curbed clout of {} (-11% clout, -10% approval).", fname));
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
    ui.label(RichText::new("🕊 RELIGION, IDEOLOGY & COUNTER-EXTREMISM COMMAND").color(accent_amber()).size(metrics.fs(15.5)).strong());
    ui.add_space(6.0);

    two_col_responsive!(
        ui,
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(RichText::new("STATE-RELIGION CONSTITUTIONAL POLICY").color(text_muted()).size(12.0).strong());
                ui.add_space(6.0);

                for &pol in ReligionPolicy::all() {
                    let selected = state.religion_policy == pol;
                    panel_frame().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            let label = if selected { format!("● {} (ACTIVE)", pol.label()) } else { pol.label().to_string() };
                            ui.label(RichText::new(label).color(if selected { accent_blue() } else { text_primary() }).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if !selected && ui.button("Adopt (16 PC)").clicked() {
                                    if state.has_pc(16.0) {
                                        state.spend_pc(16.0);
                                        state.religion_policy = pol;
                                        state.flags.religious_courts = pol == ReligionPolicy::TheocraticLaw;
                                        match pol {
                                            ReligionPolicy::StateAtheism => {
                                                state.adjust_faction(FactionId::Clergy, -24.0, -14.0);
                                                state.adjust_faction(FactionId::Nationalists, -10.0, -2.0);
                                            }
                                            ReligionPolicy::SecularPluralism => {
                                                state.adjust_faction(FactionId::Intelligentsia, 14.0, 5.0);
                                                state.sectarian_tension = (state.sectarian_tension - 10.0).max(0.0);
                                            }
                                            ReligionPolicy::OfficialReligion => {
                                                state.adjust_faction(FactionId::Clergy, 14.0, 8.0);
                                                state.adjust_faction(FactionId::Nationalists, 8.0, 4.0);
                                            }
                                            ReligionPolicy::TheocraticLaw => {
                                                state.adjust_faction(FactionId::Clergy, 25.0, 18.0);
                                                state.adjust_faction(FactionId::Intelligentsia, -24.0, -8.0);
                                                state.civil_liberties = (state.civil_liberties - 16.0).max(5.0);
                                            }
                                        }
                                        state.log(LogCategory::Religious, format!("Religious Charter: {}", pol.label()), pol.description());
                                        *status_msg = Some(format!("Adopted {} policy.", pol.label()));
                                    } else {
                                        *status_msg = Some("Need 16 Political Capital.".into());
                                    }
                                }
                            });
                        });
                        ui.label(RichText::new(pol.description()).color(text_muted()).size(11.5));
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
                ui.label(RichText::new("EXTREMISM, SECTARIANISM & INSURGENCY MONITOR").color(text_muted()).size(12.0).strong());
                ui.add_space(6.0);

                meter_bar(ui, "Religious Fundamentalist Extremism", state.religious_extremism, false);
                meter_bar(ui, "Political / Ideological Radicalism", state.ideological_extremism, false);
                meter_bar(ui, "Sectarian & Ethnic Tension", state.sectarian_tension, false);
                meter_bar(ui, "Armed Insurgency Strength", state.insurgency_strength, false);

                ui.add_space(8.0);
                ui.separator();
                ui.label(RichText::new("COUNTER-EXTREMISM OPERATIONS").color(text_muted()).size(12.0).strong());
                ui.add_space(6.0);

                ui.checkbox(&mut state.flags.deradicalization_program, "Ongoing Civic Deradicalization Program (-$0.9B/qtr)");
                ui.checkbox(&mut state.flags.mass_surveillance, "Telecom & Biometric Surveillance (-$1.4B/qtr, -Liberties)");
                ui.add_space(6.0);

                if ui.button("⚔ Launch Targeted Special Forces Raids (10 PC, $6B)").clicked() {
                    if state.has_pc(10.0) && state.treasury_bn >= 6.0 {
                        state.spend_pc(10.0);
                        state.treasury_bn -= 6.0;
                        state.insurgency_strength = (state.insurgency_strength - 14.0).max(0.0);
                        state.religious_extremism = (state.religious_extremism - 6.0).max(0.0);
                        state.ideological_extremism = (state.ideological_extremism - 6.0).max(0.0);
                        state.log(LogCategory::Military, "Counter-Extremism Raids", "Special forces dismantled radical safehouses and weapons caches.");
                        *status_msg = Some("Insurgency reduced by 14%.".into());
                    } else {
                        *status_msg = Some("Need 10 PC and $6B Treasury.".into());
                    }
                }
                ui.add_space(4.0);
                if ui.button("🕊 National Reconciliation & Interfaith Dialogue (12 PC, $4B)").clicked() {
                    if state.has_pc(12.0) && state.treasury_bn >= 4.0 {
                        state.spend_pc(12.0);
                        state.treasury_bn -= 4.0;
                        state.sectarian_tension = (state.sectarian_tension - 15.0).max(0.0);
                        state.religious_extremism = (state.religious_extremism - 8.0).max(0.0);
                        state.stability = (state.stability + 6.0).min(100.0);
                        state.log(LogCategory::Religious, "National Reconciliation Summit", "Moderate leaders signed a peace charter cooling sectarian tension.");
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
    ui.label(RichText::new("⚔ COMBINED ARMS COMMAND, NUCLEAR DETERRENCE & GEOPOLITICS").color(accent_red()).size(metrics.fs(15.5)).strong());
    ui.add_space(6.0);

    // Combined Arms 3-Branch Overview Strip (Army, Air Force, Navy)
    let mut quick_procure: Option<&'static str> = None;
    let branch_cols = if ui.available_width() >= 760.0 { 3 } else { 1 };
    let render_army = |ui: &mut egui::Ui, state: &NationState, qp: &mut Option<&'static str>| {
        hero_card_frame(accent_amber()).show(ui, |ui| {
            ui.label(RichText::new("🪖 ARMY — GROUND COMMAND").color(accent_amber()).size(metrics.fs(12.2)).strong());
            ui.label(
                RichText::new(format!(
                    "{} Divisions  •  {} Tanks  •  {:.0}K Troops",
                    state.armed_forces.army_divisions, state.armed_forces.tanks_count, state.active_troops_k
                ))
                .color(text_primary())
                .size(11.8),
            );
            meter_bar(ui, "Army Readiness", state.armed_forces.army_readiness, true);
            if ui.small_button("🪖 Procure +2 Divisions & +200 Tanks ($12B)").clicked() {
                *qp = Some("procure_army_division");
            }
        });
    };
    let render_airforce = |ui: &mut egui::Ui, state: &NationState, qp: &mut Option<&'static str>| {
        hero_card_frame(accent_cyan()).show(ui, |ui| {
            ui.label(RichText::new("✈ AIR FORCE — AEROSPACE").color(accent_cyan()).size(metrics.fs(12.2)).strong());
            ui.label(
                RichText::new(format!(
                    "{} Squadrons  •  {} SAMs  •  Jet Tier {}/4",
                    state.armed_forces.fighter_squadrons,
                    state.armed_forces.air_defense_batteries,
                    state.programs.fighter_jet_tier
                ))
                .color(text_primary())
                .size(11.8),
            );
            meter_bar(ui, "Air Force Readiness", state.armed_forces.airforce_readiness, true);
            if ui.small_button("✈ Procure +2 Squadrons & +3 SAMs ($15B)").clicked() {
                *qp = Some("procure_airforce_squadron");
            }
        });
    };
    let render_navy = |ui: &mut egui::Ui, state: &NationState, qp: &mut Option<&'static str>| {
        hero_card_frame(accent_blue()).show(ui, |ui| {
            ui.label(RichText::new("⚓ NAVY — MARITIME FLEET").color(accent_blue()).size(metrics.fs(12.2)).strong());
            ui.label(
                RichText::new(format!(
                    "{} Destroyers  •  {} Subs  •  {} Carriers",
                    state.armed_forces.destroyers, state.armed_forces.submarines, state.programs.carrier_groups
                ))
                .color(text_primary())
                .size(11.8),
            );
            meter_bar(ui, "Navy Readiness", state.armed_forces.navy_readiness, true);
            if ui.small_button("⚓ Procure +3 Destroyers & +2 Subs ($18B)").clicked() {
                *qp = Some("procure_navy_fleet");
            }
        });
    };

    if branch_cols == 3 {
        ui.columns(3, |cols| {
            render_army(&mut cols[0], state, &mut quick_procure);
            render_airforce(&mut cols[1], state, &mut quick_procure);
            render_navy(&mut cols[2], state, &mut quick_procure);
        });
    } else {
        render_army(ui, state, &mut quick_procure);
        ui.add_space(6.0);
        render_airforce(ui, state, &mut quick_procure);
        ui.add_space(6.0);
        render_navy(ui, state, &mut quick_procure);
    }

    if let Some(act) = quick_procure {
        match run_strategic_activity(state, act) {
            Ok(msg) => *status_msg = Some(msg),
            Err(err) => *status_msg = Some(format!("Procurement Blocked: {}", err)),
        }
    }

    ui.add_space(10.0);

    // Posture & Nuclear Strip
    two_col_responsive!(
        ui,
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(RichText::new("POSTURE, CONSCRIPTION & MORALE").color(text_muted()).size(12.0).strong());
                ui.add_space(6.0);
                meter_bar(ui, "Joint Combat Readiness", state.military_readiness, true);
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
                ui.checkbox(&mut state.flags.conscription, "Mandatory National Conscription (+45% troop pool, lowers unemployment)");
            });
        },
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(RichText::new("☢ STRATEGIC NUCLEAR PROGRAM").color(text_muted()).size(12.0).strong());
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label("Current Tier:");
                    badge(ui, state.nuclear_status.label(), if state.nuclear_status.stage() >= 3 { accent_red() } else { accent_amber() });
                    badge(ui, &format!("{} Warheads", state.programs.nuclear_warheads), accent_red());
                });
                ui.add_space(6.0);
                if state.nuclear_status != NuclearStatus::StrategicTriad {
                    meter_bar(ui, "Progress to Next Tier", state.nuclear_progress, true);
                    if ui.button("☢ Fund Nuclear Sprint (+35% Progress, 16 PC, $16B)").clicked() {
                        if state.has_pc(16.0) && state.treasury_bn >= 16.0 {
                            state.spend_pc(16.0);
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
                            state.log(LogCategory::Military, "Nuclear Program Accelerated", format!("Current nuclear status: {}", state.nuclear_status.label()));
                            *status_msg = Some("Advanced sovereign nuclear program.".into());
                        } else {
                            *status_msg = Some("Need 16 PC and $16B Treasury.".into());
                        }
                    }
                } else {
                    ui.label(RichText::new("Full Strategic ICBM Triad operational. Massive military deterrence active.").color(accent_green()));
                }
                ui.add_space(8.0);
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    if ui.button("🎖 Grand Military Parade (6 PC, $3B)").clicked() {
                        if state.has_pc(6.0) && state.treasury_bn >= 3.0 {
                            state.spend_pc(6.0);
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

    // Foreign Powers Diplomatic Matrix
    card_frame().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(format!("🌍 WORLD NATIONS & DIPLOMACY ({} POWERS)", state.foreign_powers.len())).color(accent_cyan()).size(metrics.fs(12.5)).strong());
            if state.annexed_count() > 0 || state.puppet_count() > 0 {
                ui.add_space(8.0);
                badge_scaled(ui, &format!("{} ANNEXED • {} CLIENT", state.annexed_count(), state.puppet_count()), accent_gold(), metrics.fs(10.5));
            }
        });
        ui.add_space(6.0);

        let mut pending_dip: Option<(ForeignPowerId, &'static str)> = None;
        let annex_cost = if state.flags.ledger_rebalance { 0 } else { 24 };
        let puppet_cost = if state.flags.ledger_rebalance { 0 } else { 18 };
        let visit_cost = if state.flags.ledger_rebalance { 0 } else { 10 };
        let split_cost = if state.flags.ledger_rebalance { 0 } else { 18 };
        let arms_cost = if state.flags.ledger_rebalance { 0 } else { 8 };
        let fp_flag_h = (metrics.flag_h * 0.98).clamp(22.0, 42.0);
        let fp_flag_w = fp_flag_h * 1.55;

        for p in &state.foreign_powers {
            let border = if p.annexed {
                border_gold()
            } else if p.at_war || p.is_rebel_state {
                accent_red()
            } else if p.puppet_state {
                accent_purple()
            } else {
                border_subtle()
            };
            egui::Frame::new()
                .fill(bg_panel())
                .stroke(Stroke::new(if p.at_war || p.annexed { 1.3_f32 } else { 1.0_f32 }, border))
                .corner_radius(egui::CornerRadius::same(7))
                .inner_margin(egui::Margin::same(10))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        draw_foreign_flag(ui, p, fp_flag_w, fp_flag_h);
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(&p.name)
                                .color(if p.annexed { accent_gold() } else { text_primary() })
                                .size(metrics.fs(13.5))
                                .strong(),
                        );
                        ui.label(RichText::new(format!("({})", p.ideology)).color(text_muted()).size(metrics.fs(11.2)));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if p.annexed {
                                badge_scaled(ui, "★ ANNEXED TERRITORY", accent_gold(), metrics.fs(10.5));
                            } else {
                                if p.is_rebel_state {
                                    badge_scaled(ui, "🔥 REBEL STATE", accent_red(), metrics.fs(10.5));
                                } else if let Some(origin) = &p.parent_origin {
                                    badge_scaled(ui, &format!("✨ SPLIT (ex-{})", origin.split(' ').next().unwrap_or(origin)), accent_cyan(), metrics.fs(10.0));
                                }
                                if p.puppet_state {
                                    badge_scaled(ui, "👑 CLIENT STATE", accent_purple(), metrics.fs(10.5));
                                }
                                if p.at_war {
                                    badge_scaled(ui, &format!("⚔ AT WAR • {:+.0}", p.war_score), accent_red(), metrics.fs(10.5));
                                }
                                if p.sanctioning_us {
                                    badge_scaled(ui, "SANCTIONING US", accent_red(), metrics.fs(10.0));
                                }
                                if p.defense_alliance {
                                    badge_scaled(ui, "🛡 ALLY", accent_green(), metrics.fs(10.0));
                                }
                                if p.trade_pact {
                                    badge_scaled(ui, "TRADE PACT", accent_blue(), metrics.fs(10.0));
                                }
                                if p.proxy_funding {
                                    badge_scaled(ui, "PROXY OPS", accent_amber(), metrics.fs(10.0));
                                }
                                signed_relation_bar(ui, p.relation);
                                ui.label(RichText::new(format!("Rel {:+.0}", p.relation)).color(text_primary()).size(12.0).strong());
                            }
                        });
                    });

                    if p.at_war && !p.annexed {
                        ui.add_space(6.0);
                        draw_war_frontline_bar(ui, &state.nation_name, &p.short_name, p.war_score);
                    }

                    ui.add_space(5.0);
                    if p.annexed {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                RichText::new(format!(
                                    "Sovereign province incorporated into {} — population, GDP, and strategic assets integrated.",
                                    state.nation_name
                                ))
                                .color(accent_green())
                                .size(metrics.fs(11.5)),
                            );
                            if ui.small_button(RichText::new("🏳 Release Allied Client (10 PC)").color(accent_cyan())).clicked() {
                                pending_dip = Some((p.id, "release_nation"));
                            }
                        });
                    } else {
                        ui.horizontal_wrapped(|ui| {
                            let extra = if p.puppet_state {
                                format!(" | Tribute +${:.1}B/qtr", p.economic_power * 0.22)
                            } else {
                                String::new()
                            };
                            ui.label(RichText::new(format!("Mil {:.0} | Econ {:.0}{}", p.military_power, p.economic_power, extra)).color(text_muted()).size(11.5));
                            ui.add_space(8.0);
                            if p.puppet_state {
                                if ui.small_button(format!("✈ State Visit ($3B, {} PC)", visit_cost)).clicked() {
                                    pending_dip = Some((p.id, "diplomatic_visit"));
                                }
                                if ui.small_button(RichText::new(format!("💰 Sell Weapons ({} PC)", arms_cost)).color(accent_green())).clicked() {
                                    pending_dip = Some((p.id, "sell_arms"));
                                }
                                if ui.small_button(RichText::new(format!("👑 Annex Client State ({} PC)", annex_cost)).color(accent_gold()).strong()).clicked() {
                                    pending_dip = Some((p.id, "annex_country"));
                                }
                            } else if !p.at_war {
                                if ui.small_button("Improve Ties (6 PC)").clicked() {
                                    pending_dip = Some((p.id, "improve"));
                                }
                                if ui.small_button(RichText::new(format!("✈ State Visit ($3B, {} PC)", visit_cost)).color(accent_cyan())).clicked() {
                                    pending_dip = Some((p.id, "diplomatic_visit"));
                                }
                                if p.id != ForeignPowerId::GlobalMonetaryCouncil {
                                    if ui.small_button(RichText::new(format!("💰 Sell Weapons ({} PC)", arms_cost)).color(accent_green())).clicked() {
                                        pending_dip = Some((p.id, "sell_arms"));
                                    }
                                }
                                if ui.small_button(if p.trade_pact { "End Trade (12 PC)" } else { "Trade Pact (12 PC)" }).clicked() {
                                    pending_dip = Some((p.id, "trade_pact"));
                                }
                                if ui.small_button(if p.defense_alliance { "Break Alliance" } else { "Defense Treaty (18 PC)" }).clicked() {
                                    pending_dip = Some((p.id, "defense_alliance"));
                                }
                                if ui.small_button(if p.we_sanction_them { "Lift Sanctions" } else { "Sanction (8 PC)" }).clicked() {
                                    pending_dip = Some((p.id, "sanction"));
                                }
                                if ui.small_button("Espionage ($6B, 14 PC)").clicked() {
                                    pending_dip = Some((p.id, "espionage"));
                                }
                                if p.id != ForeignPowerId::GlobalMonetaryCouncil {
                                    if ui.small_button(if p.proxy_funding { "Stop Proxies" } else { "Fund Rebels ($8B)" }).clicked() {
                                        pending_dip = Some((p.id, "fund_proxies"));
                                    }
                                    if p.proxy_funding || p.spy_network || state.flags.ledger_rebalance {
                                        if ui.small_button(RichText::new(format!("🔥 Foment Split ($12B, {} PC)", split_cost)).color(accent_amber()).strong()).clicked() {
                                            pending_dip = Some((p.id, "foment_split"));
                                        }
                                    }
                                    if ui.small_button(RichText::new("⚔ Declare War (20 PC)").color(accent_red()).strong()).clicked() {
                                        pending_dip = Some((p.id, "declare_war"));
                                    }
                                }
                            } else {
                                if ui.small_button(RichText::new("🕊 Negotiate Peace Treaty (12 PC)").color(accent_blue())).clicked() {
                                    pending_dip = Some((p.id, "sue_peace"));
                                }
                                if ui.small_button(RichText::new(format!("🔥 Foment Split ($12B, {} PC)", split_cost)).color(accent_amber())).clicked() {
                                    pending_dip = Some((p.id, "foment_split"));
                                }
                                if p.war_score >= 65.0 {
                                    if ui.small_button(RichText::new(format!("👑 Install Puppet ({} PC)", puppet_cost)).color(accent_purple()).strong()).clicked() {
                                        pending_dip = Some((p.id, "install_puppet"));
                                    }
                                }
                                if p.war_score >= 70.0 {
                                    if ui.small_button(RichText::new(format!("★ Conquer & Annex ({} PC)", annex_cost)).color(accent_gold()).strong()).clicked() {
                                        pending_dip = Some((p.id, "annex_country"));
                                    }
                                } else {
                                    ui.add_enabled(false, egui::Button::new("Annex (Need +70 War Score)").small());
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

pub fn render_programs_ops_tab(
    ui: &mut egui::Ui,
    state: &mut NationState,
    status_msg: &mut Option<String>,
    metrics: &UiMetrics,
) {
    ui.label(
        RichText::new("🎯 STRATEGIC PROGRAMMES, ARMS TRADE, PANDEMIC & EXECUTIVE OPS")
            .color(accent_cyan())
            .size(metrics.fs(15.5))
            .strong(),
    );
    ui.label(
        RichText::new(
            "Build stealth fighter jets, procure Army/Air Force/Navy hardware, export defense weapons, run live nuke & missile tests, manage pandemics & lockdowns, and deliver speeches.",
        )
        .color(text_muted())
        .size(metrics.fs(12.0)),
    );
    ui.add_space(8.0);

    let mut run_act: Option<&'static str> = None;
    let god = state.flags.ledger_rebalance;

    // Row 1: Fighter Jet Programme & Nuclear Tests/Arsenal
    two_col_responsive!(
        ui,
        |ui| {
            hero_card_frame(accent_cyan()).show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("✈ INDIGENOUS FIGHTER JET PROGRAMME").color(accent_cyan()).size(metrics.fs(13.0)).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        badge_scaled(ui, &format!("TIER {}/4", state.programs.fighter_jet_tier), accent_cyan(), metrics.fs(10.5));
                    });
                });
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label("Aircraft Codename:");
                    ui.text_edit_singleline(&mut state.programs.fighter_jet_name);
                });
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!(
                        "Active Generation: {}  |  Flight Tests: {}",
                        state.programs.fighter_tier_label(), state.programs.fighter_tests
                    ))
                    .color(text_primary())
                    .size(12.0),
                );
                if state.programs.fighter_jet_tier < 4 {
                    meter_bar(ui, "Next Generation R&D", state.programs.fighter_jet_progress, true);
                } else {
                    ui.label(RichText::new("★ Hypersonic Aerospace Dominance achieved!").color(accent_gold()).size(11.5));
                }
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    let adv_pc = if god { 0 } else { 12 };
                    let tst_pc = if god { 0 } else { 6 };
                    if ui.button(RichText::new(format!("🔧 Launch / Advance Jet ({} PC, $14B)", adv_pc)).color(accent_cyan()).strong()).clicked() {
                        run_act = Some("fighter_advance");
                    }
                    if ui.button(RichText::new(format!("🛫 Flight Test & Airshow ({} PC, $4B)", tst_pc)).color(accent_green()).strong()).clicked() {
                        run_act = Some("fighter_test");
                    }
                });
            });
        },
        |ui| {
            hero_card_frame(accent_amber()).show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("☢ NUCLEAR ARSENAL & LIVE NUKE TESTS").color(accent_amber()).size(metrics.fs(13.0)).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        badge_scaled(ui, &format!("{} WARHEADS", state.programs.nuclear_warheads), accent_red(), metrics.fs(10.5));
                    });
                });
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label("Nuclear Status:");
                    badge(ui, state.nuclear_status.label(), accent_amber());
                    ui.label(RichText::new(format!("Live Tests: {}", state.programs.nuke_tests_conducted)).color(text_muted()).size(11.5));
                });
                ui.add_space(4.0);
                if state.nuclear_status != NuclearStatus::StrategicTriad {
                    meter_bar(ui, "Progress to Next Nuclear Tier", state.nuclear_progress, true);
                } else {
                    ui.label(RichText::new("★ Full Strategic ICBM Triad operational!").color(accent_gold()).size(11.5));
                }
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    let wh_pc = if god { 0 } else { 10 };
                    let nt_pc = if god { 0 } else { 14 };
                    if ui.button(RichText::new(format!("☢ Build Warheads ({} PC, $14B)", wh_pc)).color(accent_amber()).strong()).clicked() {
                        run_act = Some("nuke_warheads");
                    }
                    if ui.button(RichText::new(format!("💥 Live Nuclear Test ({} PC, $10B)", nt_pc)).color(accent_red()).strong()).clicked() {
                        run_act = Some("nuke_test");
                    }
                });
            });
        },
    );

    ui.add_space(10.0);

    // Row 2: Defense Procurement & Global Arms Sales + Pandemic & Lockdown Command
    two_col_responsive!(
        ui,
        |ui| {
            hero_card_frame(accent_green()).show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("📦 DEFENSE PROCUREMENT & 💰 WEAPONS SALES").color(accent_green()).size(metrics.fs(12.8)).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        badge_scaled(
                            ui,
                            &format!("EXPORTS: {}", fmt_billions(state.armed_forces.total_arms_sales_bn)),
                            accent_gold(),
                            metrics.fs(10.2),
                        );
                    });
                });
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!(
                        "Army: {} Divs / {} Tanks  |  Air: {} Sqds / {} SAMs  |  Navy: {} DDGs / {} Subs / {} CVNs",
                        state.armed_forces.army_divisions,
                        state.armed_forces.tanks_count,
                        state.armed_forces.fighter_squadrons,
                        state.armed_forces.air_defense_batteries,
                        state.armed_forces.destroyers,
                        state.armed_forces.submarines,
                        state.programs.carrier_groups
                    ))
                    .color(text_primary())
                    .size(11.5),
                );
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    if ui.button("🪖 Procure Army Divs ($12B)").clicked() {
                        run_act = Some("procure_army_division");
                    }
                    if ui.button("✈ Procure Air Squadrons ($15B)").clicked() {
                        run_act = Some("procure_airforce_squadron");
                    }
                    if ui.button("⚓ Procure Navy Fleet ($18B)").clicked() {
                        run_act = Some("procure_navy_fleet");
                    }
                });
                ui.add_space(6.0);
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    let est_export = 18.0 + state.military_tech * 0.25 + (state.programs.fighter_jet_tier as f64) * 6.5 + (state.programs.missile_tier as f64) * 4.0;
                    if ui
                        .button(
                            RichText::new(format!("💰 Global Arms Export Contract (+${:.0}B)", est_export))
                                .color(accent_gold())
                                .strong(),
                        )
                        .clicked()
                    {
                        run_act = Some("sell_defense_weapons");
                    }
                    ui.label(RichText::new(format!("Contracts Signed: {}", state.armed_forces.weapons_contracts_signed)).color(text_muted()).size(11.0));
                });
            });
        },
        |ui| {
            hero_card_frame(if state.pandemic.active { accent_red() } else { accent_purple() }).show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("🦠 PUBLIC HEALTH, PANDEMIC & LOCKDOWN COMMAND").color(if state.pandemic.active { accent_red() } else { accent_purple() }).size(metrics.fs(12.8)).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        badge_scaled(
                            ui,
                            if state.pandemic.active { "OUTBREAK ACTIVE" } else { "ALL CLEAR" },
                            if state.pandemic.active { accent_red() } else { accent_green() },
                            metrics.fs(10.2),
                        );
                    });
                });
                ui.add_space(6.0);

                ui.horizontal_wrapped(|ui| {
                    ui.label("Lockdown Directive:");
                    for &lvl in LockdownLevel::all() {
                        let active = state.pandemic.lockdown_level == lvl;
                        if ui.selectable_label(active, lvl.label()).clicked() && !active {
                            *status_msg = Some(set_lockdown_level(state, lvl));
                        }
                    }
                });

                ui.add_space(6.0);
                if state.pandemic.active {
                    ui.label(RichText::new(format!("Active Pathogen: {}", state.pandemic.pathogen_name)).color(accent_red()).strong());
                    meter_bar(ui, "Outbreak Severity", state.pandemic.severity, false);
                    meter_bar(ui, "Population Infected", state.pandemic.infected_pct, false);
                    meter_bar(ui, "Vaccine Development", state.pandemic.vaccine_progress, true);
                    ui.horizontal_wrapped(|ui| {
                        if ui.button(RichText::new("💉 Emergency Vaccine Sprint ($12B)").color(accent_green()).strong()).clicked() {
                            run_act = Some("pandemic_vaccine_sprint");
                        }
                        if ui.button("🏥 Deploy Field Hospitals ($8B)").clicked() {
                            run_act = Some("pandemic_field_hospitals");
                        }
                    });
                } else {
                    ui.label(RichText::new("No active pandemic. Healthcare clinics operating normally. You can set preventive quarantine or run a live outbreak stress-test below:").color(text_muted()).size(11.5));
                    ui.add_space(4.0);
                    if ui.small_button("🧪 Simulate Outbreak Scenario (Test Pandemic/Lockdown)").clicked() {
                        trigger_pandemic_outbreak(state, "Aurelian Respiratory Strain-V4", 46.0, LockdownLevel::None);
                        *status_msg = Some("Pandemic scenario triggered! Manage Lockdowns & Vaccine Sprints.".into());
                    }
                }
            });
        },
    );

    ui.add_space(10.0);

    // Row 3: Missiles/Space/Navy & Executive Speeches
    two_col_responsive!(
        ui,
        |ui| {
            card_frame().show(ui, |ui| {
                ui.label(RichText::new("🚀 MISSILES, ORBITAL SPACE & WAR GAMES").color(accent_blue()).size(metrics.fs(12.5)).strong());
                ui.add_space(6.0);

                panel_frame().show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(format!("🚀 Missiles: {}", state.programs.missile_tier_label())).color(text_primary()).strong());
                        ui.label(RichText::new(format!("(Tests: {})", state.programs.missile_tests)).color(text_muted()).size(11.0));
                    });
                    if state.programs.missile_tier < 3 {
                        meter_bar(ui, "Missile R&D", state.programs.missile_progress, true);
                    }
                    ui.horizontal_wrapped(|ui| {
                        if ui.small_button("Advance Missiles ($12B)").clicked() {
                            run_act = Some("missile_advance");
                        }
                        if ui.small_button(RichText::new("🔥 Live Missile Test ($3B)").color(accent_amber())).clicked() {
                            run_act = Some("missile_test");
                        }
                    });
                });
                ui.add_space(5.0);

                panel_frame().show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(format!("🛰 Space: {}", state.programs.space_tier_label())).color(text_primary()).strong());
                        ui.label(RichText::new(format!("(Launches: {})", state.programs.space_launches)).color(text_muted()).size(11.0));
                    });
                    if state.programs.space_tier < 3 {
                        meter_bar(ui, "Orbital R&D", state.programs.space_progress, true);
                    }
                    ui.horizontal_wrapped(|ui| {
                        if ui.small_button("Advance Space ($16B)").clicked() {
                            run_act = Some("space_advance");
                        }
                        if ui.small_button(RichText::new("🚀 Rocket Launch Test ($5B)").color(accent_cyan())).clicked() {
                            run_act = Some("space_launch");
                        }
                    });
                });
                ui.add_space(5.0);

                panel_frame().show(ui, |ui| {
                    ui.label(RichText::new(format!("🚢 Carriers: {}  |  🎖 Joint War Games: {}", state.programs.carrier_groups, state.programs.military_drills)).color(text_primary()).strong());
                    ui.add_space(4.0);
                    ui.horizontal_wrapped(|ui| {
                        if ui.small_button("🚢 Commission Supercarrier ($22B)").clicked() {
                            run_act = Some("build_carrier");
                        }
                        if ui.small_button("🎖 Run Live Military Drills ($5B)").clicked() {
                            run_act = Some("military_drill");
                        }
                    });
                });
            });
        },
        |ui| {
            card_frame().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("🎙 EXECUTIVE SPEECHES & PUBLIC ADDRESSES").color(accent_gold()).size(metrics.fs(12.5)).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        badge(ui, &format!("Speeches: {} • Visits: {}", state.programs.speeches_delivered, state.programs.diplomatic_visits), accent_blue());
                    });
                });
                ui.add_space(6.0);

                let sp_pc = if god { 0 } else { 6 };
                let speeches: [(&str, &str, &str, Color32); 4] = [
                    ("speech_patriotic", "🎙 Patriotic Rally Speech", "Approval +8, Stability +5, Nationalists +10, Military +8", accent_gold()),
                    ("speech_economic", "🎙 Economic Confidence Keynote", "GDP Growth +1.0%, Inflation -0.6%, Capitalists +10, Approval +6", accent_green()),
                    ("speech_unity", "🎙 National Unity & Reconciliation Address", "Sectarian Tension -10, Extremism -6, Approval +9, Intelligentsia +8", accent_cyan()),
                    ("speech_populist", "🎙 Workers & Citizens Townhall", "Labor Unions +12, Approval +9, Legitimacy +6", accent_purple()),
                ];

                for (id, title, hint, col) in speeches {
                    panel_frame().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(RichText::new(title).color(col).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button(format!("Deliver ({} PC)", sp_pc)).clicked() {
                                    run_act = Some(id);
                                }
                            });
                        });
                        ui.label(RichText::new(hint).color(text_muted()).size(11.0));
                    });
                    ui.add_space(4.0);
                }

                ui.add_space(6.0);
                ui.separator();
                ui.label(RichText::new("🔥 REBELS, SPLITS & NATION FORMATION").color(accent_red()).size(12.0).strong());
                ui.label(
                    RichText::new(format!(
                        "Active Rebel States: {}  |  Total World Nations: {}. Use 'Foment Split' on rival powers in Military & Map to fracture enemies and birth new countries!",
                        state.rebel_states_count(), state.foreign_powers.len()
                    ))
                    .color(text_muted())
                    .size(11.2),
                );
                if state.insurgency_strength >= 35.0 {
                    ui.add_space(4.0);
                    if ui.small_button(RichText::new("⚔ Flush Out Separatist Rebels Into Open War").color(accent_red())).clicked() {
                        match spawn_domestic_rebel_state(state) {
                            Ok(m) => *status_msg = Some(m),
                            Err(e) => *status_msg = Some(e),
                        }
                    }
                }
            });
        },
    );

    if let Some(act) = run_act {
        match run_strategic_activity(state, act) {
            Ok(msg) => *status_msg = Some(msg),
            Err(err) => *status_msg = Some(format!("Operation Blocked: {}", err)),
        }
    }
}

pub fn render_decisions_tab(
    ui: &mut egui::Ui,
    state: &mut NationState,
    status_msg: &mut Option<String>,
    metrics: &UiMetrics,
) {
    ui.label(RichText::new("📜 ROYAL DECREES, STRUCTURAL REFORMS & IDEOLOGICAL SHIFTS").color(accent_blue()).size(metrics.fs(15.5)).strong());
    ui.label(RichText::new("Pivot between Capitalism, Communism, Democracy, Autocracy, and Theocracy — or launch strategic programs.").color(text_muted()).size(metrics.fs(12.0)));
    ui.add_space(8.0);

    let defs = all_decisions();
    let mut to_enact: Option<&'static str> = None;

    for def in &defs {
        let check = can_enact(state, def);
        let cat_col = match def.category {
            LogCategory::Economic => accent_green(),
            LogCategory::Political => accent_blue(),
            LogCategory::Military => accent_red(),
            LogCategory::Religious => accent_purple(),
            _ => accent_amber(),
        };
        card_frame().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                badge(ui, def.category.label(), cat_col);
                ui.label(RichText::new(def.name).color(text_primary()).size(metrics.fs(13.5)).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| match &check {
                    Ok(()) => {
                        if ui.button(RichText::new(format!("▶ ENACT ({:.0} PC | ${:.0}B)", def.political_cost, def.treasury_cost_bn)).color(accent_green()).strong()).clicked() {
                            to_enact = Some(def.id);
                        }
                    }
                    Err(reason) => {
                        ui.add_enabled(false, egui::Button::new(reason));
                    }
                });
            });
            ui.add_space(3.0);
            ui.label(RichText::new(def.description).color(text_primary()).size(12.0));
            ui.label(RichText::new(format!("Effects: {}", def.effects_summary)).color(text_muted()).size(11.5));
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
    ui.label(RichText::new("📖 IMPERIAL CHRONICLES & HISTORICAL ARCHIVES").color(accent_blue()).size(metrics.fs(15.5)).strong());
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
                LogCategory::Crisis => accent_red(),
                LogCategory::Military => accent_amber(),
                LogCategory::Economic => accent_green(),
                LogCategory::Religious => accent_purple(),
                _ => accent_blue(),
            };
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(format!("[{}]", entry.date_label)).color(text_muted()).size(11.5));
                badge(ui, entry.category.label(), col);
                ui.label(RichText::new(&entry.headline).color(text_primary()).size(12.5).strong());
            });
            ui.label(RichText::new(&entry.detail).color(text_muted()).size(11.5));
            ui.separator();
        }
    });
}
