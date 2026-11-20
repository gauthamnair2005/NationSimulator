use crate::models::*;

pub fn calculate_fiscal(state: &NationState) -> FiscalSummary {
    // Quarterly GDP
    let q_gdp = state.gdp_bn / 4.0;
    let corruption_leak = 1.0 - (state.corruption / 260.0);
    let private_share = (1.0 - state.taxes.state_ownership / 100.0).clamp(0.05, 1.0);
    let state_share = (state.taxes.state_ownership / 100.0).clamp(0.0, 0.95);

    let income_rev = q_gdp * (state.taxes.income_tax / 100.0) * 0.42 * corruption_leak;
    let corp_rev = q_gdp * (state.taxes.corporate_tax / 100.0) * 0.28 * private_share * corruption_leak;
    let tariff_rev = q_gdp * (state.taxes.tariffs / 100.0) * 0.14 * corruption_leak;
    let wealth_rev = q_gdp * (state.taxes.wealth_tax / 100.0) * 0.16 * private_share * corruption_leak;
    // State-owned enterprise surplus depends on efficiency (tech + low corruption)
    let soe_efficiency = (0.16 + state.tech_level / 600.0 - state.corruption / 500.0).clamp(0.06, 0.30);
    let swf_bonus = if state.flags.sovereign_wealth_fund { q_gdp * 0.018 } else { 0.0 };
    let mut soe_rev = q_gdp * state_share * soe_efficiency + swf_bonus;

    let b = &state.budget;
    let austerity_mult = if state.imf_austerity_turns > 0 { 0.86 } else { 1.0 };
    let mil_mult = state.defense_posture.upkeep_multiplier()
        * if state.active_wars_count() > 0 { 1.35 } else { 1.0 };
    let health_extra = if state.flags.universal_healthcare { 1.15 } else { 1.0 };

    let program_spending = q_gdp
        * austerity_mult
        * (b.healthcare * 0.00072 * health_extra
            + b.education * 0.00058
            + b.infrastructure * 0.00060
            + b.welfare * 0.00075
            + b.military * 0.00080 * mil_mult
            + b.security_intel * 0.00046
            + b.research * 0.00042
            + b.religion_culture * 0.00030)
        + if state.flags.mass_surveillance { 1.4 } else { 0.0 }
        + if state.flags.deradicalization_program { 0.9 } else { 0.0 };

    // Quarterly debt interest depends on Debt/GDP & interest rate
    let risk_premium = (state.debt_to_gdp() / 100.0).powf(1.4) * 1.6
        + if state.foreign_powers.iter().any(|p| p.sanctioning_us) { 1.2 } else { 0.0 };
    let effective_annual_rate = ((state.taxes.interest_rate * 0.65 + risk_premium) / 100.0).clamp(0.015, 0.24);
    let debt_interest = (state.national_debt_bn * effective_annual_rate) / 4.0;

    let total_spending = program_spending + debt_interest;
    if state.flags.ledger_rebalance {
        let base_rev = income_rev + corp_rev + tariff_rev + wealth_rev + soe_rev;
        let shortfall = (total_spending - base_rev).max(0.0);
        soe_rev += shortfall + 1000.0;
    }
    let total_revenue = income_rev + corp_rev + tariff_rev + wealth_rev + soe_rev;
    let net_balance = total_revenue - total_spending;

    FiscalSummary {
        income_rev,
        corp_rev,
        tariff_rev,
        wealth_rev,
        soe_rev,
        total_revenue,
        program_spending,
        debt_interest,
        total_spending,
        net_balance,
    }
}

pub fn compute_credit_rating(state: &NationState) -> String {
    let dtg = state.debt_to_gdp();
    let score = 100.0
        - dtg * 0.52
        - state.inflation * 2.2
        + state.stability * 0.28
        + state.gdp_growth * 3.0
        - if state.treasury_bn < 0.0 { 22.0 } else { 0.0 }
        - (state.foreign_powers.iter().filter(|p| p.sanctioning_us).count() as f64) * 8.0;

    match score {
        s if s >= 82.0 => "AAA",
        s if s >= 70.0 => "AA",
        s if s >= 58.0 => "A",
        s if s >= 45.0 => "BBB",
        s if s >= 32.0 => "BB",
        s if s >= 20.0 => "B",
        s if s >= 10.0 => "CCC",
        _ => "D (Default Risk)",
    }
    .to_string()
}

pub fn calculate_coup_risk(state: &NationState) -> f64 {
    let mil = state.faction(FactionId::Military);
    let sec = state.faction(FactionId::SecurityApparatus);

    let mil_threat = ((100.0 - mil.approval).max(0.0) / 100.0) * mil.clout * 0.95;
    let sec_threat = ((100.0 - sec.approval).max(0.0) / 100.0) * sec.clout * 0.45;
    let instability_factor = ((60.0 - state.stability).max(0.0)) * 0.55;
    let legitimacy_factor = ((55.0 - state.legitimacy).max(0.0)) * 0.35;
    let war_ex_factor = state.war_exhaustion * 0.22;

    let mut risk = mil_threat + sec_threat + instability_factor + legitimacy_factor + war_ex_factor - 18.0;
    if state.regime == RegimeType::MilitaryDictatorship {
        risk += 8.0; // rival generals always scheme in a junta
    }
    if state.flags.mass_surveillance && sec.approval > 50.0 {
        risk -= 10.0;
    }
    if state.flags.secret_police && sec.approval > 55.0 {
        risk -= 7.0;
    }
    if state.regime.is_democratic() && state.rule_of_law > 65.0 {
        risk -= 12.0;
    }
    risk.clamp(0.0, 100.0)
}

pub fn advance_quarter(state: &mut NationState) {
    if state.game_over.is_some() || state.active_event.is_some() {
        return;
    }

    // Advance calendar
    state.turn += 1;
    state.quarter += 1;
    if state.quarter > 4 {
        state.quarter = 1;
        state.year += 1;
    }

    // Decrement cooldowns
    for cd in state.decision_cooldowns.values_mut() {
        *cd = cd.saturating_sub(1);
    }
    for cd in state.event_cooldowns.values_mut() {
        *cd = cd.saturating_sub(1);
    }
    if state.imf_austerity_turns > 0 {
        state.imf_austerity_turns -= 1;
    }
    if state.regime.is_democratic() && !state.flags.term_limits_abolished {
        state.quarters_until_election = state.quarters_until_election.saturating_sub(1);
    }

    // 1. Political Capital regeneration
    let pc_gain = 6.0
        + (state.approval / 22.0)
        + (state.legitimacy / 28.0)
        + if state.regime.is_authoritarian() { 2.5 } else { 0.0 }
        - if state.stability < 35.0 { 3.5 } else { 0.0 };
    state.political_capital = (state.political_capital + pc_gain).clamp(0.0, 100.0);

    // 2. Macroeconomics
    let sanction_count = state.foreign_powers.iter().filter(|p| p.sanctioning_us).count() as f64;
    let trade_pacts = state.foreign_powers.iter().filter(|p| p.trade_pact).count() as f64;
    let active_wars = state.active_wars_count() as f64;

    let econ_growth_base = match state.economy {
        EconomicSystem::FreeMarketCapitalism => 3.8,
        EconomicSystem::MixedEconomy => 2.9,
        EconomicSystem::StateCapitalism => 3.1,
        EconomicSystem::PlannedCommunism => 1.9,
        EconomicSystem::WarEconomy => 1.2,
    };

    let tax_drag = ((state.taxes.corporate_tax - 22.0).max(0.0) * 0.065)
        + ((state.taxes.income_tax - 28.0).max(0.0) * 0.04)
        + ((state.taxes.tariffs - 15.0).max(0.0) * 0.035);
    let rate_drag = (state.taxes.interest_rate - 4.0) * 0.18;
    let human_cap_boost = (state.education_level + state.infrastructure_level + state.tech_level - 160.0) * 0.018;
    let corruption_drag = state.corruption * 0.032;
    let stability_effect = (state.stability - 50.0) * 0.038;
    let trade_effect = trade_pacts * 0.55 - sanction_count * 1.45 - active_wars * 2.1;
    let five_year_boost = if state.flags.five_year_plan { 0.65 } else { 0.0 };
    let insurgency_drag = state.insurgency_strength * 0.045;
    let noise = (state.next_rand() - 0.5) * 0.8;

    let target_growth = (econ_growth_base + human_cap_boost + stability_effect + trade_effect + five_year_boost
        - tax_drag
        - rate_drag
        - corruption_drag
        - insurgency_drag
        + noise)
        .clamp(-12.0, 11.5);

    state.gdp_growth = state.gdp_growth * 0.55 + target_growth * 0.45;
    state.gdp_bn = (state.gdp_bn * (1.0 + state.gdp_growth / 400.0)).max(25.0);
    state.population_m = (state.population_m * (1.0 + 0.0018 + (state.healthcare_level - 50.0) * 0.00003)).max(1.0);

    // Fiscal & Debt update
    let fiscal = calculate_fiscal(state);
    state.treasury_bn += fiscal.net_balance;
    if state.treasury_bn < 0.0 {
        // Automatic deficit borrowing
        let borrow = -state.treasury_bn;
        state.national_debt_bn += borrow * 1.04;
        state.treasury_bn = 0.0;
        state.inflation += (borrow / state.gdp_bn.max(50.0)) * 8.0;
    }
    state.last_fiscal = fiscal;

    // Inflation
    let deficit_pressure = (-state.last_fiscal.net_balance / (state.gdp_bn / 4.0).max(10.0)).max(0.0) * 14.0;
    let target_inflation = (2.4
        + deficit_pressure
        + (state.taxes.tariffs * 0.05)
        + if state.economy == EconomicSystem::WarEconomy { 2.5 } else { 0.0 }
        + sanction_count * 1.1
        - (state.taxes.interest_rate - 3.5) * 0.42
        + (state.gdp_growth - 3.0).max(0.0) * 0.35)
        .clamp(0.4, 48.0);
    state.inflation = (state.inflation * 0.72 + target_inflation * 0.28).clamp(0.2, 60.0);

    // Unemployment
    let target_unemp = (7.2
        - state.gdp_growth * 0.65
        - (state.taxes.state_ownership * 0.045)
        - if state.flags.conscription { 1.4 } else { 0.0 }
        + if state.imf_austerity_turns > 0 { 2.0 } else { 0.0 }
        + sanction_count * 0.9)
        .clamp(1.2, 34.0);
    state.unemployment = state.unemployment * 0.7 + target_unemp * 0.3;

    // Inequality (Gini)
    let gini_target = (42.0
        - state.taxes.wealth_tax * 0.32
        - state.budget.welfare * 0.14
        - state.taxes.state_ownership * 0.12
        + if state.economy == EconomicSystem::FreeMarketCapitalism { 8.0 } else { 0.0 }
        - if state.economy == EconomicSystem::PlannedCommunism { 9.0 } else { 0.0 }
        + state.corruption * 0.12)
        .clamp(20.0, 68.0);
    state.gini = state.gini * 0.88 + gini_target * 0.12;

    // Trade balance
    state.trade_balance_bn = (trade_pacts * 4.2 - sanction_count * 6.5 - state.taxes.tariffs * 0.15
        + (state.tech_level - 50.0) * 0.22)
        .clamp(-40.0, 45.0);

    // 3. Human Capital & State Capacity drift from budgets
    state.healthcare_level = drift(state.healthcare_level, state.budget.healthcare * 0.95 + if state.flags.universal_healthcare { 12.0 } else { 0.0 }, 0.08);
    state.education_level = drift(state.education_level, state.budget.education * 0.98, 0.07);
    state.infrastructure_level = drift(state.infrastructure_level, state.budget.infrastructure * 0.96, 0.07);
    state.tech_level = drift(state.tech_level, state.budget.research * 0.72 + state.education_level * 0.32, 0.06);

    // Corruption & Rule of Law
    let corr_target = (44.0
        - state.rule_of_law * 0.38
        - state.press_freedom * 0.24
        + if state.flags.secret_police { 6.0 } else { 0.0 }
        + if state.regime == RegimeType::OligarchicRepublic { 12.0 } else { 0.0 }
        + state.taxes.state_ownership * 0.08)
        .clamp(5.0, 92.0);
    state.corruption = drift(state.corruption, corr_target, 0.06);

    let rol_target = (50.0
        + if state.flags.independent_judiciary { 22.0 } else { -12.0 }
        + state.democracy_index * 0.22
        - state.corruption * 0.35
        - if state.flags.martial_law { 20.0 } else { 0.0 })
        .clamp(6.0, 96.0);
    state.rule_of_law = drift(state.rule_of_law, rol_target, 0.07);

    // Liberties & Press
    let lib_target: f64 = (76.0_f64
        - if state.flags.martial_law { 30.0 } else { 0.0 }
        - if state.flags.mass_surveillance { 22.0 } else { 0.0 }
        - if state.flags.secret_police { 18.0 } else { 0.0 }
        - if state.flags.opposition_banned { 18.0 } else { 0.0 }
        - if state.flags.religious_courts { 14.0 } else { 0.0 }
        + if state.regime.is_democratic() { 14.0 } else { -12.0 })
        .clamp(4.0, 98.0);
    state.civil_liberties = drift(state.civil_liberties, lib_target, 0.12);

    let press_target = if state.flags.press_censorship {
        16.0
    } else {
        (state.civil_liberties * 0.85 + state.democracy_index * 0.2).clamp(15.0, 98.0)
    };
    state.press_freedom = drift(state.press_freedom, press_target, 0.14);

    // 4. Religion, Extremism & Insurgency
    let rel_target = match state.religion_policy {
        ReligionPolicy::StateAtheism => 20.0,
        ReligionPolicy::SecularPluralism => 44.0,
        ReligionPolicy::OfficialReligion => 66.0,
        ReligionPolicy::TheocraticLaw => 88.0,
    } + (state.budget.religion_culture - 40.0) * 0.25
        - (state.education_level - 55.0) * 0.18;
    state.religiosity = drift(state.religiosity, rel_target.clamp(8.0, 98.0), 0.05);

    let clerical_target: f64 = match state.religion_policy {
        ReligionPolicy::StateAtheism => 8.0,
        ReligionPolicy::SecularPluralism => 24.0,
        ReligionPolicy::OfficialReligion => 56.0,
        ReligionPolicy::TheocraticLaw => 88.0,
    } + if state.flags.religious_courts { 12.0 } else { 0.0 };
    state.clerical_power = drift(state.clerical_power, clerical_target.clamp(4.0, 98.0), 0.08);

    // Extremism grows from poverty, unemployment, repression, sectarian tension, or aggressive state atheism against religious public
    let rel_ext_delta = (state.sectarian_tension * 0.04)
        + if state.religion_policy == ReligionPolicy::StateAtheism && state.religiosity > 32.0 { 1.4 } else { 0.0 }
        + if state.religion_policy == ReligionPolicy::TheocraticLaw { 0.6 } else { 0.0 }
        + (state.unemployment - 7.0).max(0.0) * 0.16
        - if state.flags.deradicalization_program { 1.6 } else { 0.0 }
        - (state.budget.security_intel - 45.0) * 0.03
        - (state.education_level - 55.0) * 0.035;
    state.religious_extremism = (state.religious_extremism + rel_ext_delta).clamp(0.0, 100.0);

    let ideo_ext_delta = ((state.gini - 42.0).max(0.0) * 0.09)
        + ((state.inflation - 7.0).max(0.0) * 0.14)
        + ((45.0 - state.approval).max(0.0) * 0.05)
        + if state.flags.martial_law { 1.1 } else { 0.0 }
        - if state.flags.deradicalization_program { 1.4 } else { 0.0 }
        - (state.budget.welfare - 45.0) * 0.025
        - (state.budget.security_intel - 45.0) * 0.028;
    state.ideological_extremism = (state.ideological_extremism + ideo_ext_delta).clamp(0.0, 100.0);

    let sec_tension_target = (14.0
        + if state.religion_policy == ReligionPolicy::TheocraticLaw { 22.0 } else { 0.0 }
        + if state.religion_policy == ReligionPolicy::StateAtheism { 18.0 } else { 0.0 }
        + state.religious_extremism * 0.35
        - if state.religion_policy == ReligionPolicy::SecularPluralism { 10.0 } else { 0.0 })
        .clamp(2.0, 95.0);
    state.sectarian_tension = drift(state.sectarian_tension, sec_tension_target, 0.08);

    let ins_delta = ((state.total_extremism() - 32.0) * 0.09)
        - (state.budget.security_intel * 0.038)
        - if state.flags.mass_surveillance { 0.9 } else { 0.0 }
        - if state.flags.martial_law { 0.7 } else { 0.0 };
    state.insurgency_strength = (state.insurgency_strength + ins_delta).clamp(0.0, 100.0);

    // 5. Military Readiness, Troops & Nuclear Program
    let target_troops = state.population_m
        * (3.2 + state.budget.military * 0.07)
        * if state.flags.conscription { 1.45 } else { 1.0 }
        * match state.defense_posture {
            DefensePosture::Pacifist => 0.65,
            DefensePosture::Defensive => 1.0,
            DefensePosture::PowerProjection => 1.25,
            DefensePosture::FullMobilization => 1.7,
        };
    state.active_troops_k = drift(state.active_troops_k, target_troops.clamp(20.0, 1800.0), 0.14);

    let readiness_target = (state.budget.military * 0.75 + state.military_tech * 0.25 - state.war_exhaustion * 0.35).clamp(15.0, 100.0);
    state.military_readiness = drift(state.military_readiness, readiness_target, 0.12);
    state.military_tech = drift(state.military_tech, state.tech_level * 0.6 + state.budget.military * 0.45, 0.05);

    if state.nuclear_status != NuclearStatus::None && state.nuclear_status != NuclearStatus::StrategicTriad {
        state.nuclear_progress += (state.budget.research * 0.05 + state.tech_level * 0.03).max(1.0);
        if state.nuclear_progress >= 100.0 {
            if let Some(next) = state.nuclear_status.next() {
                state.nuclear_status = next;
                state.nuclear_progress = 0.0;
                state.log(
                    LogCategory::Military,
                    format!("Nuclear Milestone: {}", next.label()),
                    "Strategic deterrence capabilities reached a new operational tier.",
                );
            }
        }
    }

    // 6. Active Wars & Foreign Diplomacy
    update_foreign_powers_and_wars(state);

    // 7. Domestic Faction Approval & Clout Dynamics
    update_factions(state);

    // 8. Aggregate Public Approval, Stability, Legitimacy & Coup Risk
    let weighted_faction_app: f64 = {
        let sum_w: f64 = state.factions.iter().map(|f| f.clout).sum();
        let sum_v: f64 = state.factions.iter().map(|f| f.approval * f.clout).sum();
        if sum_w > 0.0 { sum_v / sum_w } else { 50.0 }
    };

    let econ_mood = 52.0 + state.gdp_growth * 3.8 - state.inflation * 1.4 - state.unemployment * 1.3
        + (state.healthcare_level + state.education_level - 110.0) * 0.22;
    let approval_target = (weighted_faction_app * 0.52 + econ_mood * 0.48 - state.war_exhaustion * 0.32 - state.corruption * 0.18).clamp(4.0, 96.0);
    state.approval = drift(state.approval, approval_target, 0.22);

    let stability_target = (50.0
        + (state.approval - 50.0) * 0.48
        + (state.rule_of_law - 50.0) * 0.24
        - state.insurgency_strength * 0.45
        - state.total_extremism() * 0.28
        - state.war_exhaustion * 0.30
        + if state.flags.secret_police { 4.0 } else { 0.0 })
        .clamp(2.0, 98.0);
    state.stability = drift(state.stability, stability_target, 0.20);

    let legit_target = (state.approval * 0.45
        + state.rule_of_law * 0.25
        + if state.regime.is_democratic() { state.democracy_index * 0.30 } else { state.stability * 0.30 })
        .clamp(5.0, 98.0);
    state.legitimacy = drift(state.legitimacy, legit_target, 0.15);

    state.coup_risk = calculate_coup_risk(state);
    state.credit_rating = compute_credit_rating(state);

    // 9. Check Catastrophic Collapse / Takeover Conditions
    check_end_states(state);

    // 10. Record history & check for interactive scenarios/events
    state.record_history();
    if state.game_over.is_none() {
        crate::events::maybe_trigger_event(state);
    }
}

fn update_foreign_powers_and_wars(state: &mut NationState) {
    let my_power = (state.active_troops_k * 0.12 + state.military_readiness * 0.45 + state.military_tech * 0.45)
        * if state.nuclear_status.stage() >= 3 { 1.35 } else { 1.0 };
    let dtg = state.debt_to_gdp();
    let infl = state.inflation;

    let mut any_war = false;
    let mut conquered_by: Option<String> = None;
    let mut war_logs: Vec<(LogCategory, String, String)> = Vec::new();

    for p in &mut state.foreign_powers {
        // Ideological diplomatic drift
        let ideo_drift = match p.id {
            ForeignPowerId::AtlanticRepublic => {
                (if state.regime.is_democratic() { 1.2 } else { -1.2 })
                    + (if state.economy == EconomicSystem::FreeMarketCapitalism { 0.8 } else { 0.0 })
                    - (if state.economy == EconomicSystem::PlannedCommunism { 1.5 } else { 0.0 })
            }
            ForeignPowerId::PeoplesUnion => {
                if state.regime == RegimeType::OnePartyCommunist || state.economy == EconomicSystem::PlannedCommunism { 1.8 } else { -0.4 }
            }
            ForeignPowerId::EurasianFederation => {
                if state.regime.is_authoritarian() { 0.7 } else { -0.4 }
            }
            ForeignPowerId::CrescentCoalition => {
                if state.religion_policy == ReligionPolicy::TheocraticLaw || state.religion_policy == ReligionPolicy::OfficialReligion { 1.1 } else if state.religion_policy == ReligionPolicy::StateAtheism { -1.5 } else { 0.0 }
            }
            ForeignPowerId::BorderlandRepublic => {
                if state.defense_posture == DefensePosture::FullMobilization { -1.4 } else { 0.1 }
            }
            ForeignPowerId::GlobalMonetaryCouncil => {
                if dtg < 75.0 && infl < 6.0 { 0.8 } else { -0.8 }
            }
        };

        if !p.at_war {
            let pact_bonus = if p.trade_pact { 0.6 } else { 0.0 } + if p.defense_alliance { 0.9 } else { 0.0 };
            let sanc_pen = if p.sanctioning_us || p.we_sanction_them { -1.0 } else { 0.0 };
            let proxy_pen = if p.proxy_funding { -2.2 } else { 0.0 };
            p.relation = (p.relation + ideo_drift + pact_bonus + sanc_pen + proxy_pen).clamp(-100.0, 100.0);

            // Auto sanctions if relations plummet with major blocs
            if matches!(p.id, ForeignPowerId::AtlanticRepublic | ForeignPowerId::GlobalMonetaryCouncil)
                && p.relation < -60.0
                && !p.sanctioning_us
            {
                p.sanctioning_us = true;
                war_logs.push((
                    LogCategory::Diplomacy,
                    format!("{} Imposes Sanctions!", p.short_name),
                    format!("Following steep diplomatic deterioration, {} imposed trade and financial sanctions.", p.name),
                ));
            }
        } else {
            any_war = true;
            let enemy_power = p.military_power * 1.15;
            let delta = ((my_power - enemy_power) * 0.24).clamp(-16.0, 16.0);
            p.war_score = (p.war_score + delta).clamp(-100.0, 100.0);
            if p.war_score >= 95.0 {
                p.at_war = false;
                p.relation = -35.0;
                p.war_score = 0.0;
                war_logs.push((
                    LogCategory::Military,
                    format!("Victory Over {}!", p.name),
                    format!("Our armed forces forced {} to sign a favorable armistice and pay war reparations ($28B).", p.name),
                ));
                state.treasury_bn += 28.0;
                state.approval = (state.approval + 12.0).min(100.0);
            } else if p.war_score <= -95.0 {
                conquered_by = Some(p.name.clone());
            }
        }
    }

    if any_war {
        state.war_exhaustion = (state.war_exhaustion + 4.5).min(100.0);
    } else {
        state.war_exhaustion = (state.war_exhaustion - 3.5).max(0.0);
    }

    for (cat, h, d) in war_logs {
        state.log(cat, h, d);
    }

    if let Some(enemy) = conquered_by {
        state.game_over = Some(GameOverReason::ForeignConquest(format!(
            "Our front lines collapsed against {}. Foreign forces occupied the capital of {} and dissolved your government.",
            enemy, state.nation_name
        )));
    }
}

fn update_factions(state: &mut NationState) {
    let eco = state.economy;
    let reg = state.regime;
    let rel = state.religion_policy;
    let corp = state.taxes.corporate_tax;
    let wealth = state.taxes.wealth_tax;
    let own = state.taxes.state_ownership;
    let tariffs = state.taxes.tariffs;
    let welfare = state.budget.welfare;
    let mil_b = state.budget.military;
    let sec_b = state.budget.security_intel;
    let edu_b = state.budget.education;
    let rel_b = state.budget.religion_culture;
    let gini = state.gini;
    let dem = state.democracy_index;
    let lib = state.civil_liberties;
    let press = state.press_freedom;
    let growth = state.gdp_growth;
    let unemp = state.unemployment;
    let flags = state.flags.clone();

    for f in &mut state.factions {
        let target_app = match f.id {
            FactionId::Capitalists => {
                68.0 - (corp - 18.0) * 0.9 - wealth * 1.1 - own * 0.45 + growth * 2.5
                    + if eco == EconomicSystem::FreeMarketCapitalism { 14.0 } else { 0.0 }
                    - if eco == EconomicSystem::PlannedCommunism { 28.0 } else { 0.0 }
                    - if flags.capital_controls { 14.0 } else { 0.0 }
            }
            FactionId::LaborUnions => {
                48.0 + welfare * 0.38 + own * 0.18 - (gini - 34.0) * 1.1 - unemp * 1.6
                    + if flags.universal_healthcare { 10.0 } else { 0.0 }
                    + if eco == EconomicSystem::PlannedCommunism { 14.0 } else { 0.0 }
                    - if eco == EconomicSystem::FreeMarketCapitalism { 12.0 } else { 0.0 }
            }
            FactionId::Military => {
                44.0 + mil_b * 0.46
                    + if flags.conscription { 7.0 } else { 0.0 }
                    + if reg == RegimeType::MilitaryDictatorship { 12.0 } else { 0.0 }
                    - if state.defense_posture == DefensePosture::Pacifist { 20.0 } else { 0.0 }
            }
            FactionId::Clergy => {
                36.0 + rel_b * 0.42
                    + match rel {
                        ReligionPolicy::StateAtheism => -32.0,
                        ReligionPolicy::SecularPluralism => 4.0,
                        ReligionPolicy::OfficialReligion => 16.0,
                        ReligionPolicy::TheocraticLaw => 30.0,
                    }
                    + if flags.religious_courts { 10.0 } else { 0.0 }
            }
            FactionId::Intelligentsia => {
                26.0 + dem * 0.34 + lib * 0.28 + press * 0.22 + edu_b * 0.24
                    - if flags.press_censorship { 18.0 } else { 0.0 }
                    - if flags.mass_surveillance { 14.0 } else { 0.0 }
            }
            FactionId::SecurityApparatus => {
                42.0 + sec_b * 0.48
                    + if flags.secret_police { 12.0 } else { 0.0 }
                    + if flags.mass_surveillance { 10.0 } else { 0.0 }
                    + if reg.is_authoritarian() { 8.0 } else { 0.0 }
            }
            FactionId::Nationalists => {
                46.0 + tariffs * 0.32 + mil_b * 0.24
                    + if state.nuclear_status.stage() >= 3 { 12.0 } else { 0.0 }
                    - if state.imf_austerity_turns > 0 { 12.0 } else { 0.0 }
            }
        };
        f.approval = drift(f.approval, target_app.clamp(4.0, 96.0), 0.16);
    }
}

fn check_end_states(state: &mut NationState) {
    if state.game_over.is_some() {
        return;
    }

    if state.insurgency_strength >= 94.0 {
        state.game_over = Some(GameOverReason::ExtremistTakeover(format!(
            "Armed extremist insurgents overran provincial garrisons and marched into the capital of {}. The state collapsed into radical militant rule.",
            state.nation_name
        )));
        return;
    }

    if state.stability <= 6.0 && state.approval <= 14.0 {
        state.game_over = Some(GameOverReason::PopularRevolution(format!(
            "Nationwide strikes, mass defections, and storming of government ministries toppled {} {} in a sweeping popular revolution.",
            state.leader_title, state.leader_name
        )));
        return;
    }

    if state.debt_to_gdp() > 230.0 && state.inflation > 42.0 {
        state.game_over = Some(GameOverReason::SovereignCollapse(format!(
            "Hyperinflation ({:.1}%) and insurmountable sovereign debt ({:.0}% of GDP) rendered currency worthless and fractured the administrative state.",
            state.inflation,
            state.debt_to_gdp()
        )));
    }
}

pub fn diplomatic_action(state: &mut NationState, target: ForeignPowerId, action: &str) -> Result<String, String> {
    let pc_cost = match action {
        "improve" => 6.0,
        "trade_pact" => 12.0,
        "defense_alliance" => 18.0,
        "sanction" => 8.0,
        "espionage" => 14.0,
        "fund_proxies" => 16.0,
        "declare_war" => 20.0,
        "sue_peace" => 12.0,
        _ => 10.0,
    };

    if state.political_capital < pc_cost {
        return Err(format!("Need {:.0} Political Capital", pc_cost));
    }

    let p = state.foreign_mut(target);
    let pname = p.name.clone();

    match action {
        "improve" => {
            if p.at_war {
                return Err("Cannot send diplomatic envoys while at war".into());
            }
            p.relation = (p.relation + 14.0).min(100.0);
            state.political_capital -= pc_cost;
            let msg = format!("Diplomatic summit warmed bilateral ties with {}.", pname);
            state.log(LogCategory::Diplomacy, format!("Summit with {}", pname), &msg);
            Ok(msg)
        }
        "trade_pact" => {
            if p.at_war || p.sanctioning_us {
                return Err("Cannot sign trade pact during war or active sanctions".into());
            }
            if p.relation < 15.0 {
                return Err("Requires Relations >= +15".into());
            }
            p.trade_pact = !p.trade_pact;
            let active = p.trade_pact;
            if active {
                p.relation = (p.relation + 10.0).min(100.0);
            } else {
                p.relation = (p.relation - 12.0).max(-100.0);
            }
            state.political_capital -= pc_cost;
            let msg = if active {
                format!("Ratified comprehensive Free Trade Pact with {}.", pname)
            } else {
                format!("Withdrew from Trade Pact with {}.", pname)
            };
            state.log(LogCategory::Diplomacy, format!("Trade Policy: {}", pname), &msg);
            Ok(msg)
        }
        "defense_alliance" => {
            if p.at_war {
                return Err("Cannot ally while at war".into());
            }
            if p.relation < 45.0 {
                return Err("Requires Relations >= +45".into());
            }
            p.defense_alliance = !p.defense_alliance;
            let active = p.defense_alliance;
            state.political_capital -= pc_cost;
            let msg = if active {
                format!("Signed Mutual Defense Treaty with {}!", pname)
            } else {
                format!("Terminated Defense Treaty with {}.", pname)
            };
            state.log(LogCategory::Military, format!("Alliance: {}", pname), &msg);
            Ok(msg)
        }
        "sanction" => {
            p.we_sanction_them = !p.we_sanction_them;
            let on = p.we_sanction_them;
            if on {
                p.relation = (p.relation - 20.0).max(-100.0);
                p.trade_pact = false;
            } else {
                p.relation = (p.relation + 10.0).min(100.0);
            }
            state.political_capital -= pc_cost;
            let msg = if on {
                format!("Imposed economic sanctions and asset freezes on {}.", pname)
            } else {
                format!("Lifted unilateral sanctions against {}.", pname)
            };
            state.log(LogCategory::Diplomacy, format!("Sanctions: {}", pname), &msg);
            Ok(msg)
        }
        "espionage" => {
            if state.treasury_bn < 6.0 {
                return Err("Need $6.0B for covert intelligence station".into());
            }
            state.treasury_bn -= 6.0;
            state.political_capital -= pc_cost;
            state.tech_level = (state.tech_level + 3.5).min(100.0);
            state.military_tech = (state.military_tech + 3.0).min(100.0);
            let p = state.foreign_mut(target);
            p.spy_network = true;
            p.relation = (p.relation - 8.0).max(-100.0);
            let msg = format!("Intelligence operatives infiltrated {} and extracted industrial/defense blueprints.", pname);
            state.log(LogCategory::Military, format!("Covert Ops in {}", pname), &msg);
            Ok(msg)
        }
        "fund_proxies" => {
            let currently_funding = state.foreign_mut(target).proxy_funding;
            if !currently_funding && state.treasury_bn < 8.0 {
                return Err("Need $8.0B to arm proxy rebels".into());
            }
            let active = !currently_funding;
            if active {
                state.treasury_bn -= 8.0;
            }
            state.political_capital -= pc_cost;
            let p = state.foreign_mut(target);
            p.proxy_funding = active;
            if active {
                p.relation = (p.relation - 25.0).max(-100.0);
                p.military_power = (p.military_power - 4.0).max(15.0);
            }
            let msg = if active {
                format!("Covertly funneling weapons to anti-government proxy militias inside {}.", pname)
            } else {
                format!("Halted covert proxy funding against {}.", pname)
            };
            state.log(LogCategory::Military, format!("Proxy Ops: {}", pname), &msg);
            Ok(msg)
        }
        "declare_war" => {
            if p.at_war {
                return Err("Already at war".into());
            }
            if target == ForeignPowerId::GlobalMonetaryCouncil {
                return Err("Cannot declare conventional war on a multilateral financial council".into());
            }
            p.at_war = true;
            p.trade_pact = false;
            p.defense_alliance = false;
            p.relation = -100.0;
            p.war_score = 0.0;
            state.political_capital -= pc_cost;
            state.adjust_faction(FactionId::Nationalists, 10.0, 6.0);
            state.adjust_faction(FactionId::Intelligentsia, -12.0, -3.0);
            let msg = format!("Formal Declaration of War issued against {}!", pname);
            state.log(LogCategory::Military, format!("WAR DECLARED: {}", pname), &msg);
            Ok(msg)
        }
        "sue_peace" => {
            if !p.at_war {
                return Err("Not currently at war".into());
            }
            let ws = p.war_score;
            p.at_war = false;
            p.war_score = 0.0;
            p.relation = -45.0;
            state.political_capital -= pc_cost;
            let msg = if ws >= 0.0 {
                state.treasury_bn += 10.0;
                format!("Negotiated peace from a position of strength with {} (+$10B reparations).", pname)
            } else {
                state.treasury_bn -= 14.0;
                state.approval = (state.approval - 8.0).max(0.0);
                format!("Signed a painful ceasefire treaty with {} (-$14B concessions).", pname)
            };
            state.log(LogCategory::Diplomacy, format!("Peace Treaty: {}", pname), &msg);
            Ok(msg)
        }
        _ => Err("Unknown action".into()),
    }
}

pub fn rebalance_sovereign_ledger(state: &mut NationState) {
    state.national_debt_bn = 0.0;
    state.imf_austerity_turns = 0;
    state.flags.ledger_rebalance = true;
    state.treasury_bn += 1000.0;
    state.gdp_bn += 1000.0;
    state.inflation = state.inflation.min(2.0);
    state.last_fiscal = calculate_fiscal(state);
    state.credit_rating = compute_credit_rating(state);
}

fn drift(current: f64, target: f64, speed: f64) -> f64 {
    current + (target - current) * speed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_initializes_and_advances_cleanly() {
        for preset in ScenarioPreset::all() {
            let mut state = NationState::from_preset(*preset);
            assert!(state.gdp_bn > 50.0);
            assert_eq!(state.history.len(), 1);
            for _ in 0..8 {
                if let Some(ev) = state.active_event.clone() {
                    let first_choice = ev.choices[0].id.clone();
                    crate::events::resolve_event_choice(&mut state, &first_choice);
                }
                if state.game_over.is_some() {
                    break;
                }
                advance_quarter(&mut state);
            }
            assert!(state.turn >= 2);
            assert!(state.approval >= 0.0 && state.approval <= 100.0);
            assert!(state.coup_risk >= 0.0 && state.coup_risk <= 100.0);
        }
    }

    #[test]
    fn json_save_and_load_roundtrip() {
        let state = NationState::from_preset(ScenarioPreset::KrasnoviaCommunist);
        let serialized = serde_json::to_string(&state).expect("serialize");
        let restored: NationState = serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(restored.nation_name, state.nation_name);
        assert_eq!(restored.regime, RegimeType::OnePartyCommunist);
        assert_eq!(restored.economy, EconomicSystem::PlannedCommunism);
    }

    #[test]
    fn decisions_modify_regime_and_economy() {
        let mut state = NationState::from_preset(ScenarioPreset::AureliaDemocracy);
        state.political_capital = 80.0;
        state.treasury_bn = 100.0;
        let res = crate::decisions::enact_decision(&mut state, "nationalize_commanding_heights");
        assert!(res.is_ok());
        assert_eq!(state.economy, EconomicSystem::PlannedCommunism);
    }

    #[test]
    fn alienated_powerful_military_raises_coup_risk() {
        let mut state = NationState::from_preset(ScenarioPreset::SolariaDictatorship);
        let initial = calculate_coup_risk(&state);
        state.faction_mut(FactionId::Military).approval = 10.0;
        state.faction_mut(FactionId::Military).clout = 90.0;
        state.stability = 25.0;
        let elevated = calculate_coup_risk(&state);
        assert!(elevated > initial);
        assert!(elevated > 60.0);
    }

    #[test]
    fn diplomatic_war_and_peace_cycle() {
        let mut state = NationState::from_preset(ScenarioPreset::AureliaDemocracy);
        state.political_capital = 60.0;
        assert!(diplomatic_action(&mut state, ForeignPowerId::BorderlandRepublic, "declare_war").is_ok());
        assert_eq!(state.active_wars_count(), 1);
        assert!(diplomatic_action(&mut state, ForeignPowerId::BorderlandRepublic, "sue_peace").is_ok());
        assert_eq!(state.active_wars_count(), 0);
    }
}
