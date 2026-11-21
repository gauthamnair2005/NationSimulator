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
    let puppet_tribute: f64 = state
        .foreign_powers
        .iter()
        .filter(|p| p.puppet_state && !p.annexed)
        .map(|p| p.economic_power * 0.22)
        .sum();
    let mut soe_rev = q_gdp * state_share * soe_efficiency + swf_bonus + puppet_tribute;

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
        + if state.foreign_powers.iter().any(|p| p.sanctioning_us && !p.annexed) { 1.2 } else { 0.0 };
    let effective_annual_rate = ((state.taxes.interest_rate * 0.65 + risk_premium) / 100.0).clamp(0.015, 0.24);
    let debt_interest = (state.national_debt_bn * effective_annual_rate) / 4.0;

    let total_spending = program_spending + debt_interest;
    let mut god_floor = 0.0;
    if state.flags.ledger_rebalance {
        let base_rev = income_rev + corp_rev + tariff_rev + wealth_rev + soe_rev;
        let shortfall = (total_spending - base_rev).max(0.0);
        let god_add = if state.god_surplus_bn > 0.0 {
            state.god_surplus_bn
        } else {
            1000.0
        };
        god_floor = god_add;
        soe_rev += shortfall + god_add;
    }
    let total_revenue = income_rev + corp_rev + tariff_rev + wealth_rev + soe_rev;
    let net_balance = if state.flags.ledger_rebalance {
        (total_revenue - total_spending).max(god_floor)
    } else {
        total_revenue - total_spending
    };

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
    if state.flags.ledger_rebalance {
        return "AAA".to_string();
    }
    let dtg = state.debt_to_gdp();
    let score = 100.0
        - dtg * 0.52
        - state.inflation * 2.2
        + state.stability * 0.28
        + state.gdp_growth * 3.0
        - if state.treasury_bn < 0.0 { 22.0 } else { 0.0 }
        - (state.foreign_powers.iter().filter(|p| p.sanctioning_us && !p.annexed).count() as f64) * 8.0;

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
    if state.flags.ledger_rebalance {
        return 0.0;
    }
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
        state.leader_profile.age = state.leader_profile.age.saturating_add(1);
    }

    // Decrement cooldowns
    let god_mode = state.flags.ledger_rebalance;
    let cd_step = if god_mode { 2 } else { 1 };
    for cd in state.decision_cooldowns.values_mut() {
        *cd = cd.saturating_sub(cd_step);
    }
    for cd in state.event_cooldowns.values_mut() {
        *cd = cd.saturating_sub(1);
    }
    if state.imf_austerity_turns > 0 {
        state.imf_austerity_turns = if god_mode { 0 } else { state.imf_austerity_turns - 1 };
    }
    if state.regime.is_democratic() && !state.flags.term_limits_abolished {
        state.quarters_until_election = state.quarters_until_election.saturating_sub(1);
    }

    // 1. Political Capital regeneration
    let pc_gain = 6.0
        + (state.approval / 22.0)
        + (state.legitimacy / 28.0)
        + if state.regime.is_authoritarian() { 2.5 } else { 0.0 }
        + if god_mode { 35.0 } else { 0.0 }
        - if state.stability < 35.0 { 3.5 } else { 0.0 };
    state.political_capital = (state.political_capital + pc_gain).clamp(0.0, 100.0);

    // 2. Macroeconomics
    let sanction_count = state.foreign_powers.iter().filter(|p| p.sanctioning_us && !p.annexed).count() as f64;
    let trade_pacts = state.foreign_powers.iter().filter(|p| p.trade_pact && !p.annexed).count() as f64;
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
    let trade_effect = trade_pacts * 0.55 - sanction_count * 1.45 - active_wars * if god_mode { 0.0 } else { 2.1 };
    let five_year_boost = if state.flags.five_year_plan { 0.65 } else { 0.0 };
    let insurgency_drag = state.insurgency_strength * 0.045;
    let lockdown_drag = if state.pandemic.active {
        match state.pandemic.lockdown_level {
            LockdownLevel::None => state.pandemic.severity * 0.035,
            LockdownLevel::Targeted => 0.9 + state.pandemic.severity * 0.015,
            LockdownLevel::Nationwide => 2.4,
        }
    } else {
        match state.pandemic.lockdown_level {
            LockdownLevel::None => 0.0,
            LockdownLevel::Targeted => 0.6,
            LockdownLevel::Nationwide => 1.8,
        }
    };
    let tax_free_boom = if god_mode
        && state.taxes.income_tax <= 0.01
        && state.taxes.corporate_tax <= 0.01
        && state.taxes.tariffs <= 0.01
        && state.taxes.wealth_tax <= 0.01
    {
        2.2
    } else {
        0.0
    };
    let god_growth_boost = if god_mode { 4.8 + tax_free_boom } else { 0.0 };
    let noise = (state.next_rand() - 0.5) * 0.8;

    let raw_target_growth = econ_growth_base + human_cap_boost + stability_effect + trade_effect + five_year_boost + god_growth_boost
        - tax_drag
        - rate_drag
        - corruption_drag
        - insurgency_drag
        - lockdown_drag
        + noise;
    let target_growth = if god_mode {
        raw_target_growth.clamp(4.5, 18.0)
    } else {
        raw_target_growth.clamp(-12.0, 11.5)
    };

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
    if god_mode {
        state.inflation = state.inflation.min(2.2);
    }

    // Unemployment
    let target_unemp = (7.2
        - state.gdp_growth * 0.65
        - (state.taxes.state_ownership * 0.045)
        - if state.flags.conscription { 1.4 } else { 0.0 }
        + if state.imf_austerity_turns > 0 { 2.0 } else { 0.0 }
        + sanction_count * 0.9)
        .clamp(1.2, 34.0);
    state.unemployment = state.unemployment * 0.7 + target_unemp * 0.3;
    if god_mode {
        state.unemployment = state.unemployment.min(2.4);
    }

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
    if god_mode {
        state.corruption = state.corruption.min(6.0);
    }

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
    if god_mode {
        state.religious_extremism = (state.religious_extremism - 6.0).max(0.0);
        state.ideological_extremism = (state.ideological_extremism - 6.0).max(0.0);
        state.sectarian_tension = (state.sectarian_tension - 6.0).max(0.0);
        state.insurgency_strength = (state.insurgency_strength - 8.0).max(0.0);
    }

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
    let troop_cap = (state.population_m * 45.0).max(1800.0);
    state.active_troops_k = drift(state.active_troops_k, target_troops.clamp(20.0, troop_cap), 0.14);
    if god_mode {
        state.active_troops_k = state.active_troops_k.max(850.0);
    }

    let readiness_target = if god_mode {
        100.0
    } else {
        (state.budget.military * 0.75 + state.military_tech * 0.25 - state.war_exhaustion * 0.35).clamp(15.0, 100.0)
    };
    state.military_readiness = drift(state.military_readiness, readiness_target, if god_mode { 0.40 } else { 0.12 });
    state.military_tech = drift(
        state.military_tech,
        if god_mode { 100.0 } else { state.tech_level * 0.6 + state.budget.military * 0.45 },
        if god_mode { 0.35 } else { 0.05 },
    );

    // Sync Army, Air Force & Navy branch readiness
    let branch_target = if god_mode { 100.0 } else { state.military_readiness };
    state.armed_forces.army_readiness = drift(state.armed_forces.army_readiness, branch_target, 0.15);
    state.armed_forces.airforce_readiness = drift(state.armed_forces.airforce_readiness, (branch_target * 0.7 + state.military_tech * 0.3).clamp(15.0, 100.0), 0.15);
    state.armed_forces.navy_readiness = drift(state.armed_forces.navy_readiness, (branch_target * 0.75 + state.infrastructure_level * 0.25).clamp(15.0, 100.0), 0.15);

    // 5b. Pandemic & Lockdown Quarterly Progression
    if state.pandemic.active {
        if god_mode {
            state.pandemic.active = false;
            state.pandemic.severity = 0.0;
            state.pandemic.infected_pct = 0.0;
            state.pandemic.vaccine_progress = 100.0;
            state.pandemic.lockdown_level = LockdownLevel::None;
            state.log(
                LogCategory::Crisis,
                "Pandemic Eradicated Under Golden Mandate",
                "Rapid sovereign biodefense protocols eliminated the outbreak completely.",
            );
        } else {
            let hc_suppression = state.healthcare_level * 0.09 + state.pandemic.vaccine_progress * 0.08;
            let (sev_delta, inf_delta) = match state.pandemic.lockdown_level {
                LockdownLevel::None => (5.0 - hc_suppression, 6.0 - hc_suppression * 0.8),
                LockdownLevel::Targeted => (-4.0 - hc_suppression * 0.6, -3.5 - hc_suppression * 0.5),
                LockdownLevel::Nationwide => (-10.0 - hc_suppression * 0.7, -8.5 - hc_suppression * 0.6),
            };
            state.pandemic.severity = (state.pandemic.severity + sev_delta).clamp(0.0, 100.0);
            state.pandemic.infected_pct = (state.pandemic.infected_pct + inf_delta).clamp(0.0, 100.0);
            let vac_gain = (state.healthcare_level * 0.14 + state.budget.research * 0.14).max(4.0);
            state.pandemic.vaccine_progress = (state.pandemic.vaccine_progress + vac_gain).clamp(0.0, 100.0);

            if state.pandemic.lockdown_level == LockdownLevel::Nationwide {
                state.civil_liberties = (state.civil_liberties - 2.5).max(5.0);
            }
            if state.pandemic.severity > 30.0 {
                state.approval = (state.approval - (state.pandemic.severity * 0.06)).max(4.0);
                state.population_m = (state.population_m * (1.0 - state.pandemic.severity * 0.00012)).max(1.0);
            }
            if state.pandemic.vaccine_progress >= 100.0 || state.pandemic.severity <= 1.0 {
                let pname = state.pandemic.pathogen_name.clone();
                state.pandemic.active = false;
                state.pandemic.severity = 0.0;
                state.pandemic.infected_pct = 0.0;
                state.pandemic.lockdown_level = LockdownLevel::None;
                state.approval = (state.approval + 8.0).min(100.0);
                state.stability = (state.stability + 6.0).min(100.0);
                state.log(
                    LogCategory::Crisis,
                    format!("Pandemic Ended: {} Eradicated!", pname),
                    "Nationwide immunization and medical campaigns ended the outbreak; lockdowns lifted.",
                );
            }
        }
    }

    if state.nuclear_status != NuclearStatus::None && state.nuclear_status != NuclearStatus::StrategicTriad {
        state.nuclear_progress += (state.budget.research * 0.05 + state.tech_level * 0.03).max(1.0);
        if state.nuclear_progress >= 100.0 {
            if let Some(next) = state.nuclear_status.next() {
                state.nuclear_status = next;
                state.nuclear_progress = 0.0;
                if next.stage() >= 3 {
                    state.programs.nuclear_warheads = state.programs.nuclear_warheads.max(12);
                }
                state.log(
                    LogCategory::Military,
                    format!("Nuclear Milestone: {}", next.label()),
                    "Strategic deterrence capabilities reached a new operational tier.",
                );
            }
        }
    }

    // Ongoing Defense & Space R&D passive tick
    let rd_tick = (state.budget.research * 0.045 + state.tech_level * 0.03 + if god_mode { 8.0 } else { 0.0 }).max(1.0);
    if state.programs.fighter_jet_tier > 0 && state.programs.fighter_jet_tier < 4 {
        state.programs.fighter_jet_progress += rd_tick;
        if state.programs.fighter_jet_progress >= 100.0 {
            state.programs.fighter_jet_tier += 1;
            state.programs.fighter_jet_progress = 0.0;
            let jname = state.programs.fighter_jet_name.clone();
            let tier_lbl = state.programs.fighter_tier_label();
            state.log(
                LogCategory::Military,
                format!("Aerospace Milestone: {} ({})", jname, tier_lbl),
                "Indigenous fighter jet program advanced to the next combat generation.",
            );
        }
    }
    if state.programs.missile_tier > 0 && state.programs.missile_tier < 3 {
        state.programs.missile_progress += rd_tick * 0.9;
        if state.programs.missile_progress >= 100.0 {
            state.programs.missile_tier += 1;
            state.programs.missile_progress = 0.0;
            let tier_lbl = state.programs.missile_tier_label();
            state.log(
                LogCategory::Military,
                format!("Missile Milestone: {}", tier_lbl),
                "Strategic missile forces commissioned a new operational tier.",
            );
        }
    }
    if state.programs.space_tier > 0 && state.programs.space_tier < 3 {
        state.programs.space_progress += rd_tick * 0.85;
        if state.programs.space_progress >= 100.0 {
            state.programs.space_tier += 1;
            state.programs.space_progress = 0.0;
            let tier_lbl = state.programs.space_tier_label();
            state.log(
                LogCategory::Economic,
                format!("Space Milestone: {}", tier_lbl),
                "National aerospace command reached a new orbital tier.",
            );
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
    let god_mood_bonus = if god_mode { 18.0 } else { 0.0 };
    let approval_target = (weighted_faction_app * 0.52 + econ_mood * 0.48 - state.war_exhaustion * 0.32 - state.corruption * 0.18 + god_mood_bonus).clamp(4.0, 99.0);
    state.approval = drift(state.approval, approval_target, 0.22);
    if god_mode {
        state.approval = state.approval.max(85.0);
    }

    let stability_target = (50.0
        + (state.approval - 50.0) * 0.48
        + (state.rule_of_law - 50.0) * 0.24
        - state.insurgency_strength * 0.45
        - state.total_extremism() * 0.28
        - state.war_exhaustion * 0.30
        + if state.flags.secret_police { 4.0 } else { 0.0 }
        + god_mood_bonus)
        .clamp(2.0, 99.0);
    state.stability = drift(state.stability, stability_target, 0.20);
    if god_mode {
        state.stability = state.stability.max(88.0);
    }

    let legit_target = (state.approval * 0.45
        + state.rule_of_law * 0.25
        + if state.regime.is_democratic() { state.democracy_index * 0.30 } else { state.stability * 0.30 }
        + god_mood_bonus * 0.7)
        .clamp(5.0, 99.0);
    state.legitimacy = drift(state.legitimacy, legit_target, 0.15);
    if god_mode {
        state.legitimacy = state.legitimacy.max(88.0);
    }

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
    let god_mode = state.flags.ledger_rebalance;
    let puppet_power: f64 = state
        .foreign_powers
        .iter()
        .filter(|p| p.puppet_state && !p.annexed)
        .map(|p| p.military_power * 0.18)
        .sum();
    let program_power = (state.programs.fighter_jet_tier as f64) * 9.0
        + (state.programs.missile_tier as f64) * 7.5
        + (state.programs.space_tier as f64) * 5.0
        + (state.programs.carrier_groups as f64) * 6.5
        + ((state.programs.nuclear_warheads as f64) * 0.22).min(28.0);
    let branch_power = ((state.armed_forces.army_divisions as f64) * 0.65
        + (state.armed_forces.fighter_squadrons as f64) * 1.1
        + (state.armed_forces.destroyers as f64) * 0.7
        + (state.armed_forces.submarines as f64) * 1.0)
        .min(65.0);
    let my_power = ((state.active_troops_k * 0.12 + state.military_readiness * 0.45 + state.military_tech * 0.45 + program_power + branch_power)
        * if state.nuclear_status.stage() >= 3 { 1.35 } else { 1.0 }
        + puppet_power)
        * if god_mode { 2.25 } else { 1.0 };
    let dtg = state.debt_to_gdp();
    let infl = state.inflation;
    let initial_active_wars = state.active_wars_count();

    let mut any_war = false;
    let mut conquered_by: Option<String> = None;
    let mut won_war_target: Option<ForeignPowerId> = None;
    let mut war_logs: Vec<(LogCategory, String, String)> = Vec::new();

    for p in &mut state.foreign_powers {
        if p.annexed {
            p.at_war = false;
            p.sanctioning_us = false;
            p.we_sanction_them = false;
            p.trade_pact = false;
            p.defense_alliance = false;
            p.proxy_funding = false;
            continue;
        }

        if p.puppet_state {
            p.at_war = false;
            p.sanctioning_us = false;
            p.we_sanction_them = false;
            p.trade_pact = true;
            p.defense_alliance = true;
            p.relation = drift(p.relation, 92.0, 0.20).clamp(50.0, 100.0);
            continue;
        }

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
            ForeignPowerId::BharatUnion => {
                (if state.regime.is_democratic() { 0.8 } else { 0.1 })
                    + (if state.tech_level > 60.0 { 0.5 } else { 0.0 })
            }
            ForeignPowerId::NordicConfederation => {
                (if state.regime.is_democratic() && state.civil_liberties > 60.0 { 1.1 } else { -0.6 })
                    + (if state.gini < 36.0 { 0.5 } else { -0.3 })
            }
            ForeignPowerId::PacificSyndicate => {
                if state.economy == EconomicSystem::FreeMarketCapitalism || state.economy == EconomicSystem::MixedEconomy { 0.9 } else { -0.5 }
            }
            ForeignPowerId::AndeanFederation => {
                if state.taxes.tariffs < 20.0 { 0.6 } else { -0.4 }
            }
            ForeignPowerId::SaharaAlliance => {
                if state.economy == EconomicSystem::StateCapitalism || state.economy == EconomicSystem::MixedEconomy { 0.6 } else { 0.1 }
            }
            ForeignPowerId::AlpineRepublic => {
                if initial_active_wars == 0 && state.stability > 55.0 { 0.8 } else { -0.7 }
            }
            ForeignPowerId::VesperaEmpire => {
                if state.armed_forces.destroyers >= 10 || state.programs.carrier_groups >= 1 { 0.6 } else { 0.1 }
            }
            ForeignPowerId::KharosDesertLeague => {
                if state.taxes.tariffs < 25.0 { 0.7 } else { -0.5 }
            }
            ForeignPowerId::CaledonArchipelago => {
                if state.economy == EconomicSystem::FreeMarketCapitalism || state.economy == EconomicSystem::MixedEconomy { 0.8 } else { -0.3 }
            }
            ForeignPowerId::DrakoriaState => {
                if state.regime.is_authoritarian() || state.defense_posture == DefensePosture::PowerProjection { 0.7 } else { -0.8 }
            }
            ForeignPowerId::LuminaRepublic => {
                if state.tech_level >= 65.0 && state.education_level >= 60.0 { 1.0 } else { 0.0 }
            }
            ForeignPowerId::OakhavenUnion => {
                if state.stability >= 55.0 { 0.5 } else { -0.2 }
            }
            ForeignPowerId::Emergent(_) => {
                if p.is_rebel_state { -1.2 } else { 0.3 }
            }
        };

        if !p.at_war {
            if god_mode {
                p.sanctioning_us = false;
            }
            let pact_bonus = if p.trade_pact { 0.6 } else { 0.0 } + if p.defense_alliance { 0.9 } else { 0.0 };
            let sanc_pen = if p.sanctioning_us || p.we_sanction_them { -1.0 } else { 0.0 };
            let proxy_pen = if p.proxy_funding { -2.2 } else { 0.0 };
            let god_charm = if god_mode { 1.5 } else { 0.0 };
            p.relation = (p.relation + ideo_drift + pact_bonus + sanc_pen + proxy_pen + god_charm).clamp(-100.0, 100.0);

            // Auto sanctions if relations plummet with major blocs (suppressed in God Mode)
            if !god_mode
                && matches!(p.id, ForeignPowerId::AtlanticRepublic | ForeignPowerId::GlobalMonetaryCouncil)
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
            let delta = if god_mode {
                ((my_power - enemy_power) * 0.30).clamp(12.0, 32.0)
            } else {
                ((my_power - enemy_power) * 0.24).clamp(-16.0, 16.0)
            };
            p.war_score = (p.war_score + delta).clamp(-100.0, 100.0);
            if p.war_score >= 95.0 {
                p.war_score = 100.0;
                if won_war_target.is_none() {
                    won_war_target = Some(p.id);
                    war_logs.push((
                        LogCategory::Military,
                        format!("Total Victory Over {}!", p.name),
                        format!(
                            "Our armed forces captured the capital of {}! Enemy command surrendered unconditionally; peace & annexation conference convened.",
                            p.name
                        ),
                    ));
                }
            } else if p.war_score <= -95.0 && !god_mode {
                conquered_by = Some(p.name.clone());
            }
        }
    }

    if god_mode {
        state.war_exhaustion = 0.0;
    } else if any_war {
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
    } else if let Some(target_id) = won_war_target {
        state.pending_event_target = Some(target_id);
        state.active_event = Some(crate::events::build_post_war_victory_event(state, target_id));
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
        let adjusted_target = if flags.ledger_rebalance {
            (target_app + 24.0).clamp(80.0, 99.0)
        } else {
            target_app.clamp(4.0, 96.0)
        };
        f.approval = drift(f.approval, adjusted_target, if flags.ledger_rebalance { 0.30 } else { 0.16 });
        if flags.ledger_rebalance {
            f.approval = f.approval.max(78.0);
        }
    }
}

fn check_end_states(state: &mut NationState) {
    if state.game_over.is_some() || state.flags.ledger_rebalance {
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

pub fn annex_foreign_power(
    state: &mut NationState,
    target: ForeignPowerId,
    peaceful: bool,
) -> Result<String, String> {
    if target == ForeignPowerId::GlobalMonetaryCouncil {
        return Err("Cannot annex a multilateral financial council".into());
    }
    let (pname, econ_p, mil_p) = {
        let p = state.foreign(target);
        if p.annexed {
            return Err(format!("{} is already annexed", p.name));
        }
        (p.name.clone(), p.economic_power, p.military_power)
    };

    let pop_gain = (econ_p * 0.28 + mil_p * 0.14).max(5.0);
    let gdp_gain = (econ_p * 6.5).max(80.0);
    let treasury_seized = (econ_p * 1.1).max(20.0);
    let troops_gain = (mil_p * 1.4).max(15.0);
    let tech_boost = if econ_p >= 70.0 { 8.0 } else { 3.5 };
    let nation_name = state.nation_name.clone();
    let god_mode = state.flags.ledger_rebalance;

    {
        let p = state.foreign_mut(target);
        p.annexed = true;
        p.puppet_state = false;
        p.at_war = false;
        p.war_score = 0.0;
        p.sanctioning_us = false;
        p.we_sanction_them = false;
        p.trade_pact = false;
        p.defense_alliance = false;
        p.proxy_funding = false;
        p.spy_network = false;
        p.relation = 100.0;
        p.military_power = 0.0;
        p.economic_power = 0.0;
        p.ideology = format!("Annexed Province of {}", nation_name);
    }

    state.population_m += pop_gain;
    state.gdp_bn += gdp_gain;
    state.treasury_bn += treasury_seized;
    state.active_troops_k += troops_gain;
    state.tech_level = (state.tech_level + tech_boost).min(100.0);
    state.military_tech = (state.military_tech + tech_boost).min(100.0);
    state.approval = (state.approval + 10.0).min(100.0);
    state.legitimacy = (state.legitimacy + 6.0).min(100.0);
    state.adjust_faction(FactionId::Nationalists, 18.0, 10.0);
    state.adjust_faction(FactionId::Military, 14.0, 8.0);

    if !state.annexed_territories.iter().any(|t| t == &pname) {
        state.annexed_territories.push(pname.clone());
    }

    if peaceful || god_mode {
        state.stability = (state.stability + 8.0).min(100.0);
        state.war_exhaustion = 0.0;
    } else {
        state.insurgency_strength = (state.insurgency_strength + 12.0).min(100.0);
        state.sectarian_tension = (state.sectarian_tension + 10.0).min(100.0);
        state.war_exhaustion = (state.war_exhaustion - 15.0).max(0.0);
        for other in &mut state.foreign_powers {
            if other.id != target && !other.annexed && !other.puppet_state {
                other.relation = (other.relation - 20.0).max(-100.0);
                if matches!(
                    other.id,
                    ForeignPowerId::AtlanticRepublic | ForeignPowerId::GlobalMonetaryCouncil
                ) && other.relation < -45.0
                {
                    other.sanctioning_us = true;
                }
            }
        }
    }

    let msg = format!(
        "Annexed {} into {}! (+{:.1}M Pop, +${:.0}B GDP, +${:.0}B Treasury, +{:.0}K Troops)",
        pname, state.nation_name, pop_gain, gdp_gain, treasury_seized, troops_gain
    );
    state.log(LogCategory::Military, format!("ANNEXATION: {}", pname), &msg);

    if state.is_global_hegemon() {
        state.log(
            LogCategory::Political,
            "GLOBAL HEGEMONY ACHIEVED!",
            format!(
                "All rival sovereign powers have been annexed or brought under the banner of {}. History enters a new unipolar era.",
                state.nation_name
            ),
        );
    }

    state.last_fiscal = calculate_fiscal(state);
    state.coup_risk = calculate_coup_risk(state);
    state.credit_rating = compute_credit_rating(state);
    Ok(msg)
}

pub fn install_puppet_regime(
    state: &mut NationState,
    target: ForeignPowerId,
) -> Result<String, String> {
    if target == ForeignPowerId::GlobalMonetaryCouncil {
        return Err("Cannot install a puppet regime on a multilateral financial council".into());
    }
    let (pname, econ_p) = {
        let p = state.foreign(target);
        if p.annexed {
            return Err(format!("{} is already annexed", p.name));
        }
        (p.name.clone(), p.economic_power)
    };

    let nation_name = state.nation_name.clone();
    let indemnity = 22.0 + econ_p * 0.45;
    let tribute = econ_p * 0.22;
    let god_mode = state.flags.ledger_rebalance;

    {
        let p = state.foreign_mut(target);
        p.puppet_state = true;
        p.at_war = false;
        p.war_score = 0.0;
        p.sanctioning_us = false;
        p.we_sanction_them = false;
        p.proxy_funding = false;
        p.trade_pact = true;
        p.defense_alliance = true;
        p.relation = 90.0;
        p.ideology = format!("Client State of {}", nation_name);
    }

    state.treasury_bn += indemnity;
    state.approval = (state.approval + 8.0).min(100.0);
    state.adjust_faction(FactionId::Nationalists, 10.0, 5.0);
    state.adjust_faction(FactionId::Military, 10.0, 5.0);
    if !god_mode {
        state.insurgency_strength = (state.insurgency_strength + 3.0).min(100.0);
        for other in &mut state.foreign_powers {
            if other.id != target && !other.annexed && !other.puppet_state {
                other.relation = (other.relation - 10.0).max(-100.0);
            }
        }
    } else {
        state.war_exhaustion = 0.0;
    }

    let msg = format!(
        "Installed compliant client government in {} (+${:.0}B indemnity, +${:.1}B/qtr tribute, Trade & Defense Pacts locked).",
        pname, indemnity, tribute
    );
    state.log(LogCategory::Diplomacy, format!("Client State: {}", pname), &msg);

    if state.is_global_hegemon() {
        state.log(
            LogCategory::Political,
            "GLOBAL HEGEMONY ACHIEVED!",
            format!(
                "All rival sovereign powers have been annexed or brought under the banner of {}. History enters a new unipolar era.",
                state.nation_name
            ),
        );
    }

    state.last_fiscal = calculate_fiscal(state);
    state.coup_risk = calculate_coup_risk(state);
    state.credit_rating = compute_credit_rating(state);
    Ok(msg)
}

pub fn impose_war_reparations(
    state: &mut NationState,
    target: ForeignPowerId,
) -> Result<String, String> {
    let (pname, econ_p) = {
        let p = state.foreign(target);
        if p.annexed {
            return Err(format!("{} is already annexed", p.name));
        }
        (p.name.clone(), p.economic_power)
    };
    let reparations = 28.0 + econ_p * 0.45;
    {
        let p = state.foreign_mut(target);
        p.at_war = false;
        p.war_score = 0.0;
        p.relation = -35.0;
        p.military_power = (p.military_power * 0.65).max(15.0);
    }
    state.treasury_bn += reparations;
    state.approval = (state.approval + 12.0).min(100.0);
    state.war_exhaustion = (state.war_exhaustion - 20.0).max(0.0);
    let msg = format!(
        "Imposed armistice and demilitarization on {} (+${:.0}B war reparations, enemy military slashed 35%).",
        pname, reparations
    );
    state.log(LogCategory::Military, format!("Victory Treaty: {}", pname), &msg);
    state.last_fiscal = calculate_fiscal(state);
    state.coup_risk = calculate_coup_risk(state);
    state.credit_rating = compute_credit_rating(state);
    Ok(msg)
}

pub fn spawn_split_country(
    state: &mut NationState,
    parent_id: ForeignPowerId,
    backed_by_us: bool,
) -> Result<String, String> {
    if parent_id == ForeignPowerId::GlobalMonetaryCouncil {
        return Err("Cannot split a multilateral financial council".into());
    }
    let (parent_name, parent_short, parent_mil, parent_econ) = {
        let p = state.foreign(parent_id);
        if p.annexed {
            return Err("Cannot split an already-annexed territory".into());
        }
        if p.military_power < 18.0 && p.economic_power < 18.0 {
            return Err(format!("{} is too small to fracture further", p.name));
        }
        (p.name.clone(), p.short_name.clone(), p.military_power, p.economic_power)
    };

    let emergent_num = state.next_emergent_id.max(1);
    state.next_emergent_id = emergent_num + 1;

    let prefixes = [
        "Free Republic of",
        "Sovereign State of",
        "Democratic Front of",
        "United Provinces of",
        "Independent Syndicate of",
        "People's Republic of",
    ];
    let regions = [
        "North", "East", "Highland", "Coastal", "New", "Southern",
    ];
    let ideologies = [
        "Breakaway Republic / Mixed",
        "Secessionist Federation / Free Market",
        "Revolutionary People's State",
        "Autonomous Regional Coalition",
    ];
    let idx = (emergent_num as usize) % prefixes.len();
    let r_idx = ((emergent_num as usize) + 2) % regions.len();
    let i_idx = (emergent_num as usize) % ideologies.len();

    let clean_parent = parent_short.split(' ').next().unwrap_or(&parent_short);
    let new_name = format!("{} {} {}", prefixes[idx], regions[r_idx], clean_parent);
    let new_short = format!("{} {}", regions[r_idx], clean_parent);

    // Weaken parent nation
    {
        let p = state.foreign_mut(parent_id);
        p.military_power = (p.military_power * 0.65).max(12.0);
        p.economic_power = (p.economic_power * 0.68).max(12.0);
        p.proxy_funding = false;
        if backed_by_us {
            p.relation = (p.relation - 35.0).max(-100.0);
        }
    }

    let child = ForeignPower {
        id: ForeignPowerId::Emergent(emergent_num),
        name: new_name.clone(),
        short_name: new_short.clone(),
        ideology: ideologies[i_idx].into(),
        military_power: (parent_mil * 0.42).clamp(18.0, 58.0),
        economic_power: (parent_econ * 0.40).clamp(18.0, 58.0),
        relation: if backed_by_us { 68.0 } else { 12.0 },
        trade_pact: backed_by_us,
        defense_alliance: false,
        sanctioning_us: false,
        we_sanction_them: false,
        at_war: false,
        war_score: 0.0,
        proxy_funding: false,
        spy_network: false,
        annexed: false,
        puppet_state: false,
        is_rebel_state: false,
        parent_origin: Some(parent_name.clone()),
        flag_seed: ((emergent_num * 7 + 3) % 12) as u8,
    };

    state.foreign_powers.push(child);

    let msg = format!(
        "NEW COUNTRY FORMED: {} broke away from {}! ({} weakened by ~35%)",
        new_name, parent_name, parent_short
    );
    state.log(
        LogCategory::Diplomacy,
        format!("Nation Split: {} Formed!", new_short),
        &msg,
    );
    Ok(msg)
}

pub fn spawn_domestic_rebel_state(state: &mut NationState) -> Result<String, String> {
    let emergent_num = state.next_emergent_id.max(1);
    state.next_emergent_id = emergent_num + 1;

    let rebel_names = [
        ("Free Separatist Republic", "Separatists"),
        ("National Liberation Front", "Rebel Front"),
        ("Breakaway Highland Junta", "Highland Rebels"),
        ("People's Revolutionary Canton", "Red Canton"),
    ];
    let (rname, rshort) = rebel_names[(emergent_num as usize) % rebel_names.len()];
    let full_name = format!("{} (ex-{})", rname, state.nation_name);

    let rebel_mil = (26.0 + state.insurgency_strength * 0.35).clamp(24.0, 62.0);
    let rebel_econ = (20.0 + state.gdp_bn * 0.03).clamp(20.0, 48.0);
    let pop_lost = (state.population_m * 0.08).clamp(1.2, 8.0);
    let gdp_lost = (state.gdp_bn * 0.07).clamp(15.0, 90.0);

    state.population_m = (state.population_m - pop_lost).max(2.0);
    state.gdp_bn = (state.gdp_bn - gdp_lost).max(25.0);
    state.insurgency_strength = (state.insurgency_strength - 28.0).max(10.0);

    let rebel = ForeignPower {
        id: ForeignPowerId::Emergent(emergent_num),
        name: full_name.clone(),
        short_name: rshort.into(),
        ideology: "Armed Breakaway Rebel State".into(),
        military_power: rebel_mil,
        economic_power: rebel_econ,
        relation: -100.0,
        trade_pact: false,
        defense_alliance: false,
        sanctioning_us: false,
        we_sanction_them: false,
        at_war: true,
        war_score: -10.0,
        proxy_funding: false,
        spy_network: false,
        annexed: false,
        puppet_state: false,
        is_rebel_state: true,
        parent_origin: Some(state.nation_name.clone()),
        flag_seed: ((emergent_num * 5 + 1) % 12) as u8,
    };

    state.foreign_powers.push(rebel);
    let msg = format!(
        "REBEL SECESSION: {} seized border provinces (-{:.1}M Pop, -${:.0}B GDP) and declared war!",
        full_name, pop_lost, gdp_lost
    );
    state.log(LogCategory::Crisis, format!("REBEL STATE: {}", rshort), &msg);
    Ok(msg)
}

pub fn release_annexed_as_ally(state: &mut NationState, target: ForeignPowerId) -> Result<String, String> {
    let pname = {
        let p = state.foreign(target);
        if !p.annexed {
            return Err("Territory must be annexed first to release as an allied nation".into());
        }
        p.name.clone()
    };
    let nation_name = state.nation_name.clone();
    {
        let p = state.foreign_mut(target);
        p.annexed = false;
        p.puppet_state = true;
        p.at_war = false;
        p.relation = 95.0;
        p.trade_pact = true;
        p.defense_alliance = true;
        p.military_power = 42.0;
        p.economic_power = 48.0;
        p.ideology = format!("Allied Sovereign Client of {}", nation_name);
    }
    state.annexed_territories.retain(|t| t != &pname);
    state.legitimacy = (state.legitimacy + 6.0).min(100.0);
    state.insurgency_strength = (state.insurgency_strength - 8.0).max(0.0);
    let msg = format!(
        "Granted sovereign client independence to {} (Allied Client State with Trade, Defense & Tribute).",
        pname
    );
    state.log(LogCategory::Diplomacy, format!("New Client Formed: {}", pname), &msg);
    state.last_fiscal = calculate_fiscal(state);
    Ok(msg)
}

pub fn set_lockdown_level(state: &mut NationState, level: LockdownLevel) -> String {
    state.pandemic.lockdown_level = level;
    let msg = match level {
        LockdownLevel::None => "Lifted all pandemic lockdowns; commerce and travel fully open.",
        LockdownLevel::Targeted => "Activated Targeted Regional Quarantine to slow pathogen spread with moderate economic impact.",
        LockdownLevel::Nationwide => "Enacted Nationwide Lockdown! Rapidly suppresses pandemic severity at higher GDP & liberty cost.",
    };
    state.log(
        LogCategory::Crisis,
        format!("Public Health Order: {}", level.label()),
        msg,
    );
    msg.to_string()
}

pub fn trigger_pandemic_outbreak(state: &mut NationState, pathogen: impl Into<String>, initial_severity: f64, lockdown: LockdownLevel) {
    let name = pathogen.into();
    state.pandemic.active = true;
    state.pandemic.pathogen_name = name.clone();
    state.pandemic.severity = initial_severity.clamp(10.0, 90.0);
    state.pandemic.infected_pct = (initial_severity * 0.65).clamp(5.0, 80.0);
    state.pandemic.vaccine_progress = state.pandemic.vaccine_progress.max(10.0);
    state.pandemic.lockdown_level = lockdown;
    state.log(
        LogCategory::Crisis,
        format!("🦠 PANDEMIC ALERT: {}", name),
        format!(
            "Public health emergency declared (Severity {:.0}%, {}). Fund Vaccine Sprints or adjust Lockdown level.",
            state.pandemic.severity,
            lockdown.label()
        ),
    );
}

pub fn run_strategic_activity(state: &mut NationState, activity: &str) -> Result<String, String> {
    let god_mode = state.flags.ledger_rebalance;
    let (pc_cost, money_cost, cd_turns) = match activity {
        "fighter_advance" => (if god_mode { 5.0 } else { 12.0 }, 14.0, 2),
        "fighter_test" => (if god_mode { 3.0 } else { 6.0 }, 4.0, 2),
        "missile_advance" => (if god_mode { 5.0 } else { 12.0 }, 12.0, 2),
        "missile_test" => (if god_mode { 3.0 } else { 6.0 }, 3.0, 2),
        "nuke_warheads" => (if god_mode { 4.0 } else { 10.0 }, 14.0, 2),
        "nuke_test" => (if god_mode { 6.0 } else { 14.0 }, 10.0, 3),
        "space_advance" => (if god_mode { 5.0 } else { 12.0 }, 16.0, 2),
        "space_launch" => (if god_mode { 3.0 } else { 6.0 }, 5.0, 2),
        "build_carrier" => (if god_mode { 6.0 } else { 14.0 }, 22.0, 3),
        "military_drill" => (if god_mode { 4.0 } else { 8.0 }, 5.0, 2),
        "procure_army_division" => (if god_mode { 3.0 } else { 8.0 }, 12.0, 1),
        "procure_airforce_squadron" => (if god_mode { 4.0 } else { 9.0 }, 15.0, 1),
        "procure_navy_fleet" => (if god_mode { 4.0 } else { 10.0 }, 18.0, 2),
        "sell_defense_weapons" => (if god_mode { 2.0 } else { 6.0 }, 0.0, 2),
        "pandemic_vaccine_sprint" => (if god_mode { 2.0 } else { 8.0 }, 12.0, 1),
        "pandemic_field_hospitals" => (if god_mode { 2.0 } else { 6.0 }, 8.0, 1),
        "speech_patriotic" | "speech_economic" | "speech_unity" | "speech_populist" => {
            (if god_mode { 2.0 } else { 6.0 }, 0.0, 2)
        }
        _ => return Err("Unknown activity".into()),
    };

    if let Some(&cd) = state.decision_cooldowns.get(activity) {
        if cd > 0 && !god_mode {
            return Err(format!("Cooldown: {} quarter(s)", cd));
        }
    }
    if !state.has_pc(pc_cost) {
        return Err(format!("Need {:.0} Political Capital", pc_cost));
    }
    if state.treasury_bn < money_cost {
        return Err(format!("Need ${:.1}B Treasury", money_cost));
    }

    // Activity-specific prerequisites
    match activity {
        "fighter_advance" if state.programs.fighter_jet_tier >= 4 => {
            return Err("Fighter Jet Programme already at Hypersonic Aerospace Apex".into());
        }
        "fighter_test" if state.programs.fighter_jet_tier == 0 => {
            return Err("Launch the Fighter Jet Programme first before flight testing".into());
        }
        "missile_advance" if state.programs.missile_tier >= 3 => {
            return Err("Missile Programme already at Mach-12 Hypersonic Apex".into());
        }
        "missile_test" if state.programs.missile_tier == 0 => {
            return Err("Launch the Missile Programme first before test launches".into());
        }
        "nuke_warheads" if state.nuclear_status.stage() < 3 => {
            return Err("Requires Tactical Nuclear Arsenal or Strategic Triad".into());
        }
        "nuke_test" if state.nuclear_status == NuclearStatus::None => {
            return Err("Initiate the Nuclear Program before conducting nuclear tests".into());
        }
        "space_advance" if state.programs.space_tier >= 3 => {
            return Err("Space Programme already at Sovereign Orbital Command Apex".into());
        }
        "space_launch" if state.programs.space_tier == 0 => {
            return Err("Initiate the Space Programme before rocket test launches".into());
        }
        "pandemic_vaccine_sprint" | "pandemic_field_hospitals" if !state.pandemic.active => {
            return Err("No active pandemic outbreak right now".into());
        }
        _ => {}
    }

    state.spend_pc(pc_cost);
    state.treasury_bn -= money_cost;
    if !god_mode {
        state.decision_cooldowns.insert(activity.to_string(), cd_turns);
    }

    let (cat, headline, msg) = match activity {
        "fighter_advance" => {
            if state.programs.fighter_jet_tier == 0 {
                state.programs.fighter_jet_tier = 1;
                state.programs.fighter_jet_progress = 20.0;
            } else {
                state.programs.fighter_jet_progress += 50.0;
                if state.programs.fighter_jet_progress >= 100.0 {
                    state.programs.fighter_jet_tier = (state.programs.fighter_jet_tier + 1).min(4);
                    state.programs.fighter_jet_progress = 0.0;
                }
            }
            state.military_tech = (state.military_tech + 6.0).min(100.0);
            state.tech_level = (state.tech_level + 4.0).min(100.0);
            state.adjust_faction(FactionId::Military, 8.0, 3.0);
            state.adjust_faction(FactionId::Nationalists, 7.0, 2.0);
            let jn = state.programs.fighter_jet_name.clone();
            let tl = state.programs.fighter_tier_label();
            (
                LogCategory::Military,
                format!("Fighter Programme: {} Advanced", jn),
                format!("{} aerospace programme hit major breakthroughs ({}).", jn, tl),
            )
        }
        "fighter_test" => {
            state.programs.fighter_tests += 1;
            state.military_tech = (state.military_tech + 5.0).min(100.0);
            state.military_readiness = (state.military_readiness + 6.0).min(100.0);
            state.approval = (state.approval + 4.5).min(100.0);
            state.treasury_bn += 12.0; // export licensing & airshow deals
            state.adjust_faction(FactionId::Military, 6.0, 2.0);
            state.adjust_faction(FactionId::Nationalists, 6.0, 2.0);
            let jn = state.programs.fighter_jet_name.clone();
            let n = state.programs.fighter_tests;
            (
                LogCategory::Military,
                format!("Flight Test #{}: {} Stuns Observers!", n, jn),
                format!(
                    "{} completed supersonic stealth flight test #{} (+5 Mil Tech, +6 Readiness, +$12B defense export orders).",
                    jn, n
                ),
            )
        }
        "missile_advance" => {
            if state.programs.missile_tier == 0 {
                state.programs.missile_tier = 1;
                state.programs.missile_progress = 25.0;
            } else {
                state.programs.missile_progress += 55.0;
                if state.programs.missile_progress >= 100.0 {
                    state.programs.missile_tier = (state.programs.missile_tier + 1).min(3);
                    state.programs.missile_progress = 0.0;
                }
            }
            state.military_tech = (state.military_tech + 5.5).min(100.0);
            state.adjust_faction(FactionId::Military, 7.0, 2.0);
            let tl = state.programs.missile_tier_label();
            (
                LogCategory::Military,
                format!("Missile R&D: {}", tl),
                format!("Strategic rocket forces advanced propulsion and guidance ({}).", tl),
            )
        }
        "missile_test" => {
            state.programs.missile_tests += 1;
            state.military_readiness = (state.military_readiness + 7.0).min(100.0);
            state.adjust_faction(FactionId::Nationalists, 8.0, 3.0);
            state.adjust_faction(FactionId::Military, 7.0, 2.0);
            for p in &mut state.foreign_powers {
                if p.at_war && !p.annexed {
                    p.war_score = (p.war_score + 8.0).min(100.0);
                } else if !p.annexed && !p.puppet_state && !god_mode {
                    p.relation = (p.relation - 3.0).max(-100.0);
                }
            }
            let n = state.programs.missile_tests;
            (
                LogCategory::Military,
                format!("Live Missile Test #{} Successful!", n),
                format!(
                    "Precision test launch #{} struck target 4,000km downrange (+7 Readiness, +8 War Score in active wars).",
                    n
                ),
            )
        }
        "nuke_warheads" => {
            let added = if god_mode { 25 } else { 15 };
            state.programs.nuclear_warheads += added;
            state.adjust_faction(FactionId::Military, 7.0, 3.0);
            state.adjust_faction(FactionId::Nationalists, 7.0, 3.0);
            let total = state.programs.nuclear_warheads;
            (
                LogCategory::Military,
                format!("Nuclear Stockpile Expanded ({} Warheads)", total),
                format!(
                    "Strategic weapons plants commissioned +{} thermonuclear warheads (Total Arsenal: {}).",
                    added, total
                ),
            )
        }
        "nuke_test" => {
            state.programs.nuke_tests_conducted += 1;
            if state.nuclear_status != NuclearStatus::StrategicTriad {
                state.nuclear_progress += 45.0;
                if state.nuclear_progress >= 100.0 {
                    if let Some(next) = state.nuclear_status.next() {
                        state.nuclear_status = next;
                        state.nuclear_progress = 0.0;
                        if next.stage() >= 3 {
                            state.programs.nuclear_warheads = state.programs.nuclear_warheads.max(15);
                        }
                    }
                }
            } else {
                state.programs.nuclear_warheads += 10;
            }
            state.military_tech = (state.military_tech + 8.0).min(100.0);
            state.adjust_faction(FactionId::Military, 14.0, 5.0);
            state.adjust_faction(FactionId::Nationalists, 14.0, 5.0);
            for p in &mut state.foreign_powers {
                if p.at_war && !p.annexed {
                    p.war_score = (p.war_score + 14.0).min(100.0);
                } else if !p.annexed && !p.puppet_state && !god_mode {
                    p.relation = (p.relation - 10.0).max(-100.0);
                }
            }
            let n = state.programs.nuke_tests_conducted;
            (
                LogCategory::Military,
                format!("☢ LIVE NUCLEAR TEST #{} DETONATED!", n),
                format!(
                    "Seismic stations worldwide detected underground thermonuclear test #{}! Nuclear deterrent surged (+14 War Score in active wars, +8 Mil Tech).",
                    n
                ),
            )
        }
        "space_advance" => {
            if state.programs.space_tier == 0 {
                state.programs.space_tier = 1;
                state.programs.space_progress = 25.0;
            } else {
                state.programs.space_progress += 55.0;
                if state.programs.space_progress >= 100.0 {
                    state.programs.space_tier = (state.programs.space_tier + 1).min(3);
                    state.programs.space_progress = 0.0;
                }
            }
            state.tech_level = (state.tech_level + 6.0).min(100.0);
            state.adjust_faction(FactionId::Intelligentsia, 10.0, 3.0);
            let tl = state.programs.space_tier_label();
            (
                LogCategory::Economic,
                format!("Space Programme: {}", tl),
                format!("National space agency unlocked major orbital capabilities ({}).", tl),
            )
        }
        "space_launch" => {
            state.programs.space_launches += 1;
            state.tech_level = (state.tech_level + 6.0).min(100.0);
            state.military_tech = (state.military_tech + 4.0).min(100.0);
            state.approval = (state.approval + 6.0).min(100.0);
            state.legitimacy = (state.legitimacy + 5.0).min(100.0);
            let n = state.programs.space_launches;
            (
                LogCategory::Economic,
                format!("🛰 Orbital Rocket Launch #{} Flawless!", n),
                format!(
                    "Heavy-lift launch #{} deployed reconnaissance and telecom satellites (+6 Tech, +6 Approval, +5 Legitimacy).",
                    n
                ),
            )
        }
        "build_carrier" => {
            state.programs.carrier_groups += 1;
            state.active_troops_k += 20.0;
            state.military_readiness = (state.military_readiness + 5.0).min(100.0);
            state.armed_forces.navy_readiness = (state.armed_forces.navy_readiness + 8.0).min(100.0);
            state.adjust_faction(FactionId::Military, 9.0, 4.0);
            state.adjust_faction(FactionId::Nationalists, 8.0, 3.0);
            let total = state.programs.carrier_groups;
            (
                LogCategory::Military,
                format!("🚢 Supercarrier Strike Group #{} Commissioned!", total),
                format!(
                    "Flagship nuclear-powered carrier entered fleet service (Total Carrier Groups: {}, +20K Troops).",
                    total
                ),
            )
        }
        "procure_army_division" => {
            state.armed_forces.army_divisions += 2;
            state.armed_forces.tanks_count += 200;
            state.armed_forces.army_readiness = (state.armed_forces.army_readiness + 7.0).min(100.0);
            state.active_troops_k += 25.0;
            state.adjust_faction(FactionId::Military, 6.0, 2.0);
            (
                LogCategory::Military,
                format!("🪖 Army Procurement: {} Armored Divisions Active", state.armed_forces.army_divisions),
                format!(
                    "Procured +2 Armored Divisions & +200 Main Battle Tanks (Total: {} Divs, {} Tanks, +25K Troops).",
                    state.armed_forces.army_divisions, state.armed_forces.tanks_count
                ),
            )
        }
        "procure_airforce_squadron" => {
            state.armed_forces.fighter_squadrons += 2;
            state.armed_forces.air_defense_batteries += 3;
            state.armed_forces.airforce_readiness = (state.armed_forces.airforce_readiness + 8.0).min(100.0);
            state.military_tech = (state.military_tech + 3.0).min(100.0);
            state.adjust_faction(FactionId::Military, 7.0, 2.0);
            (
                LogCategory::Military,
                format!("✈ Air Force Expansion: {} Fighter Squadrons", state.armed_forces.fighter_squadrons),
                format!(
                    "Commissioned +2 Combat Fighter Squadrons & +3 SAM Air-Defense Batteries (Total: {} Squadrons, {} SAMs).",
                    state.armed_forces.fighter_squadrons, state.armed_forces.air_defense_batteries
                ),
            )
        }
        "procure_navy_fleet" => {
            state.armed_forces.destroyers += 3;
            state.armed_forces.submarines += 2;
            state.armed_forces.navy_readiness = (state.armed_forces.navy_readiness + 8.0).min(100.0);
            state.active_troops_k += 10.0;
            state.adjust_faction(FactionId::Military, 7.0, 2.0);
            (
                LogCategory::Military,
                format!("⚓ Navy Fleet Procurement: {} Destroyers & {} Subs", state.armed_forces.destroyers, state.armed_forces.submarines),
                format!(
                    "Launched +3 Guided-Missile Destroyers & +2 Attack Submarines (Fleet: {} DDGs, {} Subs, {} Carriers).",
                    state.armed_forces.destroyers, state.armed_forces.submarines, state.programs.carrier_groups
                ),
            )
        }
        "sell_defense_weapons" => {
            let deal_val = 18.0
                + (state.military_tech * 0.25)
                + (state.programs.fighter_jet_tier as f64) * 6.5
                + (state.programs.missile_tier as f64) * 4.0;
            state.treasury_bn += deal_val;
            state.gdp_bn += deal_val * 0.45;
            state.trade_balance_bn = (state.trade_balance_bn + 5.0).min(100.0);
            state.armed_forces.total_arms_sales_bn += deal_val;
            state.armed_forces.weapons_contracts_signed += 1;
            state.adjust_faction(FactionId::Capitalists, 8.0, 2.0);
            state.adjust_faction(FactionId::Military, 5.0, 1.0);
            (
                LogCategory::Economic,
                format!("💰 Global Defense Arms Export (+${:.1}B)", deal_val),
                format!(
                    "Exported indigenous fighter jets, armored vehicles, and air-defense systems (+${:.1}B Treasury, Contract #{}).",
                    deal_val, state.armed_forces.weapons_contracts_signed
                ),
            )
        }
        "pandemic_vaccine_sprint" => {
            state.pandemic.vaccine_progress = (state.pandemic.vaccine_progress + 38.0).min(100.0);
            state.pandemic.severity = (state.pandemic.severity - 8.0).max(0.0);
            state.healthcare_level = (state.healthcare_level + 4.0).min(100.0);
            let cured = state.pandemic.vaccine_progress >= 100.0;
            if cured {
                state.pandemic.active = false;
                state.pandemic.severity = 0.0;
                state.pandemic.infected_pct = 0.0;
                state.pandemic.lockdown_level = LockdownLevel::None;
                state.approval = (state.approval + 10.0).min(100.0);
                state.stability = (state.stability + 8.0).min(100.0);
            }
            (
                LogCategory::Crisis,
                if cured {
                    "💉 Vaccine Completed — Pandemic Eradicated!".into()
                } else {
                    format!("💉 Emergency Vaccine Sprint ({:.0}%)", state.pandemic.vaccine_progress)
                },
                if cured {
                    "Mass immunization eradicated the pandemic completely and lifted all lockdowns!".into()
                } else {
                    format!(
                        "Biomedical labs accelerated vaccine trials (+38% progress, now {:.0}%, severity -8%).",
                        state.pandemic.vaccine_progress
                    )
                },
            )
        }
        "pandemic_field_hospitals" => {
            state.pandemic.severity = (state.pandemic.severity - 15.0).max(0.0);
            state.pandemic.infected_pct = (state.pandemic.infected_pct - 10.0).max(0.0);
            state.healthcare_level = (state.healthcare_level + 6.0).min(100.0);
            state.approval = (state.approval + 6.0).min(100.0);
            (
                LogCategory::Crisis,
                "🏥 Emergency Field Hospitals Deployed".into(),
                format!(
                    "Mobile ICU clinics slashed outbreak severity to {:.0}% and boosted Healthcare (+6).",
                    state.pandemic.severity
                ),
            )
        }
        "military_drill" => {
            state.programs.military_drills += 1;
            state.military_readiness = (state.military_readiness + 10.0).min(100.0);
            state.insurgency_strength = (state.insurgency_strength - 5.0).max(0.0);
            state.adjust_faction(FactionId::Military, 8.0, 3.0);
            let n = state.programs.military_drills;
            (
                LogCategory::Military,
                format!("🎖 Joint War Games Exercise #{} Completed", n),
                "Combined land, air, and naval maneuvers sharpened combat readiness (+10 Readiness, +8 Mil Loyalty).".into(),
            )
        }
        "speech_patriotic" => {
            state.programs.speeches_delivered += 1;
            state.approval = (state.approval + 8.0).min(100.0);
            state.stability = (state.stability + 5.0).min(100.0);
            state.adjust_faction(FactionId::Nationalists, 10.0, 3.0);
            state.adjust_faction(FactionId::Military, 8.0, 2.0);
            (
                LogCategory::Political,
                format!("🎙 {} {} Delivers Patriotic Rally Speech", state.leader_title, state.leader_name),
                "Rousing televised address galvanized national pride (+8 Approval, +10 Nationalists, +8 Military).".into(),
            )
        }
        "speech_economic" => {
            state.programs.speeches_delivered += 1;
            state.gdp_growth += 1.0;
            state.inflation = (state.inflation - 0.6).max(0.4);
            state.approval = (state.approval + 6.0).min(100.0);
            state.adjust_faction(FactionId::Capitalists, 10.0, 3.0);
            (
                LogCategory::Economic,
                format!("🎙 {} {} Addresses Markets & Investors", state.leader_title, state.leader_name),
                "Confidence-building economic keynote calmed inflation (-0.6%) and boosted growth mood (+1.0%).".into(),
            )
        }
        "speech_unity" => {
            state.programs.speeches_delivered += 1;
            state.sectarian_tension = (state.sectarian_tension - 10.0).max(0.0);
            state.religious_extremism = (state.religious_extremism - 6.0).max(0.0);
            state.ideological_extremism = (state.ideological_extremism - 6.0).max(0.0);
            state.approval = (state.approval + 9.0).min(100.0);
            state.stability = (state.stability + 6.0).min(100.0);
            state.adjust_faction(FactionId::Intelligentsia, 8.0, 2.0);
            (
                LogCategory::Political,
                format!("🎙 National Unity Address by {} {}", state.leader_title, state.leader_name),
                "Historic reconciliation speech cooled sectarian tension (-10) and lowered extremism (-6).".into(),
            )
        }
        "speech_populist" => {
            state.programs.speeches_delivered += 1;
            state.approval = (state.approval + 9.0).min(100.0);
            state.legitimacy = (state.legitimacy + 6.0).min(100.0);
            state.adjust_faction(FactionId::LaborUnions, 12.0, 3.0);
            (
                LogCategory::Political,
                format!("🎙 Workers & Citizens Townhall by {} {}", state.leader_title, state.leader_name),
                "Grassroots address energized working families (+9 Approval, +12 Labor Unions, +6 Legitimacy).".into(),
            )
        }
        _ => unreachable!(),
    };

    state.last_fiscal = calculate_fiscal(state);
    state.coup_risk = calculate_coup_risk(state);
    state.credit_rating = compute_credit_rating(state);
    state.log(cat, headline, &msg);
    Ok(msg)
}

pub fn diplomatic_action(state: &mut NationState, target: ForeignPowerId, action: &str) -> Result<String, String> {
    let god_mode = state.flags.ledger_rebalance;
    let pc_cost = if god_mode {
        0.0
    } else {
        match action {
            "improve" => 6.0,
            "diplomatic_visit" => 10.0,
            "sell_arms" => 8.0,
            "trade_pact" => 12.0,
            "defense_alliance" => 18.0,
            "sanction" => 8.0,
            "espionage" => 14.0,
            "fund_proxies" => 16.0,
            "foment_split" => 18.0,
            "release_nation" => 10.0,
            "declare_war" => 20.0,
            "sue_peace" => 12.0,
            "annex_country" => 24.0,
            "install_puppet" => 18.0,
            _ => 10.0,
        }
    };

    if !state.has_pc(pc_cost) {
        return Err(format!("Need {:.0} Political Capital", pc_cost));
    }

    if action == "release_nation" {
        state.spend_pc(pc_cost);
        return release_annexed_as_ally(state, target);
    }

    if state.foreign(target).annexed {
        return Err("Territory is already annexed into your nation".into());
    }

    let p = state.foreign_mut(target);
    let pname = p.name.clone();

    match action {
        "improve" => {
            if p.at_war {
                return Err("Cannot send diplomatic envoys while at war".into());
            }
            p.relation = (p.relation + 14.0).min(100.0);
            state.spend_pc(pc_cost);
            let msg = format!("Diplomatic summit warmed bilateral ties with {}.", pname);
            state.log(LogCategory::Diplomacy, format!("Summit with {}", pname), &msg);
            Ok(msg)
        }
        "diplomatic_visit" => {
            if p.at_war {
                return Err("Cannot conduct an official state visit while at war".into());
            }
            if state.treasury_bn < 3.0 {
                return Err("Need $3.0B for state delegation & summit grants".into());
            }
            state.treasury_bn -= 3.0;
            state.spend_pc(pc_cost);
            state.legitimacy = (state.legitimacy + 4.0).min(100.0);
            state.approval = (state.approval + 3.0).min(100.0);
            state.programs.diplomatic_visits += 1;
            let lifted_sanctions = {
                let fp = state.foreign_mut(target);
                fp.relation = (fp.relation + 22.0).min(100.0);
                if fp.sanctioning_us && fp.relation >= 0.0 {
                    fp.sanctioning_us = false;
                    true
                } else {
                    false
                }
            };
            let bonus_note = if lifted_sanctions {
                " Sanctions lifted!"
            } else {
                state.treasury_bn += 6.0;
                " Signed +$6B bilateral trade agreements."
            };
            let msg = format!(
                "{} {} completed a landmark State Visit to {} (+22 Relations, +4 Legitimacy).{}",
                state.leader_title, state.leader_name, pname, bonus_note
            );
            state.log(LogCategory::Diplomacy, format!("✈ State Visit: {}", pname), &msg);
            Ok(msg)
        }
        "foment_split" => {
            if target == ForeignPowerId::GlobalMonetaryCouncil {
                return Err("Cannot foment a separatist split in a financial council".into());
            }
            let eligible = p.proxy_funding || p.spy_network || p.at_war || god_mode;
            if !eligible {
                return Err("Requires active Proxy Rebels, Spy Network, or War inside target nation".into());
            }
            if state.treasury_bn < 12.0 {
                return Err("Need $12.0B to back separatist independence movement".into());
            }
            state.treasury_bn -= 12.0;
            state.spend_pc(pc_cost);
            spawn_split_country(state, target, true)
        }
        "sell_arms" => {
            if target == ForeignPowerId::GlobalMonetaryCouncil {
                return Err("Cannot sell defense hardware to a financial council".into());
            }
            if p.at_war || p.sanctioning_us {
                return Err("Cannot sell weapons during war or active sanctions".into());
            }
            if !p.puppet_state && p.relation < 0.0 && !god_mode {
                return Err("Requires Relations >= 0 to sign defense arms contract".into());
            }
            let contract_bn = 20.0 + (state.military_tech * 0.22) + (state.programs.fighter_jet_tier as f64) * 5.0;
            state.spend_pc(pc_cost);
            state.treasury_bn += contract_bn;
            state.gdp_bn += contract_bn * 0.35;
            state.armed_forces.total_arms_sales_bn += contract_bn;
            state.armed_forces.weapons_contracts_signed += 1;
            state.adjust_faction(FactionId::Capitalists, 6.0, 2.0);
            state.adjust_faction(FactionId::Military, 4.0, 1.0);
            {
                let fp = state.foreign_mut(target);
                fp.relation = (fp.relation + 10.0).min(100.0);
                fp.military_power = (fp.military_power + 3.0).min(100.0);
            }
            let msg = format!(
                "Sold defense weapons & aircraft package to {} (+${:.1}B Treasury, +10 Relations).",
                pname, contract_bn
            );
            state.log(LogCategory::Economic, format!("💰 Arms Sale to {}", pname), &msg);
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
            state.spend_pc(pc_cost);
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
            state.spend_pc(pc_cost);
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
            state.spend_pc(pc_cost);
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
            state.spend_pc(pc_cost);
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
            state.spend_pc(pc_cost);
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
            if p.puppet_state {
                return Err("Already a client state — use Annex Client State directly".into());
            }
            if target == ForeignPowerId::GlobalMonetaryCouncil {
                return Err("Cannot declare conventional war on a multilateral financial council".into());
            }
            p.at_war = true;
            p.trade_pact = false;
            p.defense_alliance = false;
            p.relation = -100.0;
            p.war_score = 0.0;
            state.spend_pc(pc_cost);
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
            state.spend_pc(pc_cost);
            if state.pending_event_target == Some(target) {
                state.pending_event_target = None;
            }
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
        "annex_country" => {
            if target == ForeignPowerId::GlobalMonetaryCouncil {
                return Err("Cannot annex a multilateral financial council".into());
            }
            let is_puppet = p.puppet_state;
            let can_annex = is_puppet || (p.at_war && p.war_score >= 70.0);
            if !can_annex {
                return Err("Requires War Score >= +70 (or Client State status) to annex".into());
            }
            state.spend_pc(pc_cost);
            if state.pending_event_target == Some(target) {
                state.pending_event_target = None;
            }
            annex_foreign_power(state, target, is_puppet)
        }
        "install_puppet" => {
            if target == ForeignPowerId::GlobalMonetaryCouncil {
                return Err("Cannot install a puppet regime on a multilateral financial council".into());
            }
            if !p.at_war || p.war_score < 65.0 {
                return Err("Requires active war with War Score >= +65 to install client regime".into());
            }
            state.spend_pc(pc_cost);
            if state.pending_event_target == Some(target) {
                state.pending_event_target = None;
            }
            install_puppet_regime(state, target)
        }
        _ => Err("Unknown action".into()),
    }
}

pub fn rebalance_sovereign_ledger(state: &mut NationState) {
    let was_active = state.flags.ledger_rebalance;
    state.national_debt_bn = 0.0;
    state.imf_austerity_turns = 0;
    state.flags.ledger_rebalance = true;

    // First activation grants +$1.00T/qtr; every subsequent trigger multiplies it by 50x (1T -> 50T -> 2.5Q -> ...)
    if !was_active || state.god_surplus_bn <= 0.0 {
        state.god_surplus_bn = 1000.0;
    } else {
        state.god_surplus_bn = (state.god_surplus_bn * 50.0).min(1.0e18);
    }

    let boost = state.god_surplus_bn;
    state.treasury_bn = (state.treasury_bn + boost).min(1.0e21);
    state.gdp_bn = (state.gdp_bn + boost).min(1.0e21);
    state.gdp_growth = state.gdp_growth.max(8.5);
    state.inflation = state.inflation.min(1.5);
    state.unemployment = state.unemployment.min(2.0);
    state.political_capital = 100.0;

    // Governance, Stability & Development Apex
    state.approval = state.approval.max(92.0);
    state.stability = state.stability.max(94.0);
    state.legitimacy = state.legitimacy.max(95.0);
    state.rule_of_law = state.rule_of_law.max(90.0);
    state.corruption = state.corruption.min(5.0);
    state.education_level = state.education_level.max(92.0);
    state.healthcare_level = state.healthcare_level.max(92.0);
    state.infrastructure_level = state.infrastructure_level.max(92.0);
    state.tech_level = state.tech_level.max(95.0);

    // Zero Extremism, Coup Immunity & Military Supremacy
    state.religious_extremism = state.religious_extremism.min(3.0);
    state.ideological_extremism = state.ideological_extremism.min(3.0);
    state.sectarian_tension = state.sectarian_tension.min(4.0);
    state.insurgency_strength = 0.0;
    state.war_exhaustion = 0.0;
    state.coup_risk = 0.0;
    state.military_readiness = 100.0;
    state.military_tech = 100.0;
    state.active_troops_k = state.active_troops_k.max(850.0);
    state.nuclear_status = NuclearStatus::StrategicTriad;
    state.nuclear_progress = 100.0;
    state.programs.nuclear_warheads = state.programs.nuclear_warheads.max(120);
    state.programs.fighter_jet_tier = state.programs.fighter_jet_tier.max(3);
    state.programs.missile_tier = state.programs.missile_tier.max(2);
    state.programs.space_tier = state.programs.space_tier.max(2);
    state.programs.carrier_groups = state.programs.carrier_groups.max(3);
    state.armed_forces.army_divisions = state.armed_forces.army_divisions.max(36);
    state.armed_forces.tanks_count = state.armed_forces.tanks_count.max(2400);
    state.armed_forces.army_readiness = 100.0;
    state.armed_forces.fighter_squadrons = state.armed_forces.fighter_squadrons.max(24);
    state.armed_forces.air_defense_batteries = state.armed_forces.air_defense_batteries.max(30);
    state.armed_forces.airforce_readiness = 100.0;
    state.armed_forces.destroyers = state.armed_forces.destroyers.max(24);
    state.armed_forces.submarines = state.armed_forces.submarines.max(16);
    state.armed_forces.navy_readiness = 100.0;
    state.pandemic.active = false;
    state.pandemic.severity = 0.0;
    state.pandemic.infected_pct = 0.0;
    state.pandemic.lockdown_level = LockdownLevel::None;

    // Domestic Faction Loyalty & Sanctions Relief
    for f in &mut state.factions {
        f.approval = f.approval.max(88.0);
    }
    for p in &mut state.foreign_powers {
        if !p.annexed && !p.at_war {
            p.sanctioning_us = false;
            p.relation = (p.relation + 15.0).min(100.0);
        }
    }

    state.last_fiscal = calculate_fiscal(state);
    state.credit_rating = compute_credit_rating(state);
    state.coup_risk = calculate_coup_risk(state);

    if !was_active {
        state.log(
            LogCategory::Political,
            "Sovereign Golden Mandate Activated",
            "Sovereign debt wiped, +$1.00T surplus locked, 0% tax ceiling unlocked, Strategic Triad armed, domestic factions united, and Golden Age incidents enabled.",
        );
    }
}

pub fn zero_all_taxes(state: &mut NationState) {
    if !state.flags.ledger_rebalance {
        return;
    }
    state.taxes.income_tax = 0.0;
    state.taxes.corporate_tax = 0.0;
    state.taxes.tariffs = 0.0;
    state.taxes.wealth_tax = 0.0;
    state.taxes.interest_rate = 0.0;
    state.approval = (state.approval + 6.0).min(100.0);
    for f in &mut state.factions {
        f.approval = (f.approval + 6.0).min(100.0);
    }
    state.last_fiscal = calculate_fiscal(state);
    state.credit_rating = compute_credit_rating(state);
    state.log(
        LogCategory::Economic,
        "Tax-Free Sovereign Decree Enacted",
        "All national taxes and central bank interest rates set to 0.0% under the Golden Mandate.",
    );
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

    #[test]
    fn war_victory_triggers_conquest_conference_and_annexation() {
        let mut state = NationState::from_preset(ScenarioPreset::SolariaDictatorship);
        state.political_capital = 80.0;
        state.active_troops_k = 600.0;
        state.military_readiness = 95.0;
        state.military_tech = 90.0;

        let init_gdp = state.gdp_bn;
        let init_pop = state.population_m;

        assert!(diplomatic_action(&mut state, ForeignPowerId::BorderlandRepublic, "declare_war").is_ok());
        state.foreign_mut(ForeignPowerId::BorderlandRepublic).war_score = 92.0;

        advance_quarter(&mut state);
        let ev = state.active_event.as_ref().expect("post_war_victory event should trigger");
        assert_eq!(ev.id, "post_war_victory");

        crate::events::resolve_event_choice(&mut state, "annex_territory");
        let borderland = state.foreign(ForeignPowerId::BorderlandRepublic);
        assert!(borderland.annexed);
        assert!(!borderland.at_war);
        assert_eq!(state.annexed_count(), 1);
        assert!(state.gdp_bn > init_gdp + 100.0);
        assert!(state.population_m > init_pop + 10.0);
        // Further diplomatic actions on annexed territory are rejected
        assert!(diplomatic_action(&mut state, ForeignPowerId::BorderlandRepublic, "declare_war").is_err());
    }

    #[test]
    fn direct_puppet_and_subsequent_annexation_work() {
        let mut state = NationState::from_preset(ScenarioPreset::AureliaDemocracy);
        state.political_capital = 100.0;
        assert!(diplomatic_action(&mut state, ForeignPowerId::BorderlandRepublic, "declare_war").is_ok());
        // Cannot annex before war_score >= 70
        assert!(diplomatic_action(&mut state, ForeignPowerId::BorderlandRepublic, "annex_country").is_err());

        state.foreign_mut(ForeignPowerId::BorderlandRepublic).war_score = 72.0;
        let fiscal_before = calculate_fiscal(&state);
        assert!(diplomatic_action(&mut state, ForeignPowerId::BorderlandRepublic, "install_puppet").is_ok());
        assert!(state.foreign(ForeignPowerId::BorderlandRepublic).puppet_state);
        assert_eq!(state.puppet_count(), 1);
        let fiscal_after = calculate_fiscal(&state);
        assert!(fiscal_after.soe_rev > fiscal_before.soe_rev);

        // Now peacefully annex the puppet state
        assert!(diplomatic_action(&mut state, ForeignPowerId::BorderlandRepublic, "annex_country").is_ok());
        assert!(state.foreign(ForeignPowerId::BorderlandRepublic).annexed);
        assert!(!state.foreign(ForeignPowerId::BorderlandRepublic).puppet_state);
    }

    #[test]
    fn god_mode_boosts_player_and_triggers_favorable_incidents() {
        let mut state = NationState::from_preset(ScenarioPreset::SolariaDictatorship);
        rebalance_sovereign_ledger(&mut state);

        assert!(state.flags.ledger_rebalance);
        assert_eq!(state.god_surplus_bn, 1000.0);
        assert!(state.last_fiscal.net_balance >= 1000.0);
        assert_eq!(state.national_debt_bn, 0.0);
        assert_eq!(state.coup_risk, 0.0);
        assert_eq!(state.credit_rating, "AAA");
        assert_eq!(state.nuclear_status, NuclearStatus::StrategicTriad);
        assert!(state.military_readiness >= 99.0);
        assert!(state.military_tech >= 99.0);
        assert!(state.factions.iter().all(|f| f.approval >= 85.0));

        // Subsequent triggers multiply quarterly surplus by 50x (1T -> 50T -> 2,500T)
        rebalance_sovereign_ledger(&mut state);
        assert_eq!(state.god_surplus_bn, 50_000.0);
        assert!(state.last_fiscal.net_balance >= 50_000.0);
        rebalance_sovereign_ledger(&mut state);
        assert_eq!(state.god_surplus_bn, 2_500_000.0);
        assert!(state.last_fiscal.net_balance >= 2_500_000.0);

        // Verify infinite Political Capital in God Mode across repeated actions
        for _ in 0..15 {
            assert!(run_strategic_activity(&mut state, "speech_patriotic").is_ok());
            assert_eq!(state.political_capital, 100.0);
        }
        assert!(diplomatic_action(&mut state, ForeignPowerId::LuminaRepublic, "diplomatic_visit").is_ok());
        assert_eq!(state.political_capital, 100.0);

        let mut seen_god_ids = std::collections::HashSet::new();
        for _ in 0..24 {
            if let Some(ev) = state.active_event.clone() {
                assert!(
                    ev.id.starts_with("god_") || ev.id == "national_election" || ev.id == "post_war_victory",
                    "Unexpected negative crisis in god mode: {}",
                    ev.id
                );
                if ev.id.starts_with("god_") {
                    seen_god_ids.insert(ev.id.clone());
                }
                let first_choice = ev.choices[0].id.clone();
                crate::events::resolve_event_choice(&mut state, &first_choice);
            }
            advance_quarter(&mut state);
            assert!(state.game_over.is_none());
            assert_eq!(state.coup_risk, 0.0);
        }
        assert!(
            seen_god_ids.len() >= 4,
            "Expected multiple distinct player-favoring god mode incidents, saw {:?}",
            seen_god_ids
        );
    }

    #[test]
    fn god_mode_allows_zero_taxes_with_positive_surplus() {
        let mut state = NationState::from_preset(ScenarioPreset::AureliaDemocracy);
        // Should do nothing before God Mode is active
        zero_all_taxes(&mut state);
        assert!(state.taxes.income_tax > 0.0);

        rebalance_sovereign_ledger(&mut state);
        zero_all_taxes(&mut state);
        assert_eq!(state.taxes.income_tax, 0.0);
        assert_eq!(state.taxes.corporate_tax, 0.0);
        assert_eq!(state.taxes.tariffs, 0.0);
        assert_eq!(state.taxes.wealth_tax, 0.0);
        assert_eq!(state.taxes.interest_rate, 0.0);

        advance_quarter(&mut state);
        assert!(state.last_fiscal.net_balance >= 999.0);
        assert!(state.gdp_growth > 6.0);
        assert_eq!(state.credit_rating, "AAA");
    }

    #[test]
    fn existing_save_file_deserializes_with_defaults() {
        if let Ok(json) = std::fs::read_to_string("sovereign_save.json") {
            let loaded: NationState = serde_json::from_str(&json).expect("save file should deserialize");
            assert!(!loaded.nation_name.is_empty());
        }
        // Also verify a legacy payload missing annexed/puppet fields deserializes cleanly
        let state = NationState::from_preset(ScenarioPreset::AureliaDemocracy);
        let mut val = serde_json::to_value(&state).expect("to_value");
        if let Some(obj) = val.as_object_mut() {
            obj.remove("annexed_territories");
            obj.remove("pending_event_target");
            if let Some(fps) = obj.get_mut("foreign_powers").and_then(|v| v.as_array_mut()) {
                for fp in fps {
                    if let Some(fp_obj) = fp.as_object_mut() {
                        fp_obj.remove("annexed");
                        fp_obj.remove("puppet_state");
                    }
                }
            }
        }
        let legacy_json = serde_json::to_string(&val).expect("to_string");
        let mut restored: NationState = serde_json::from_str(&legacy_json).expect("legacy json should deserialize");
        assert_eq!(restored.annexed_count(), 0);
        assert_eq!(restored.puppet_count(), 0);
        restored.ensure_expanded_world();
        assert!(restored.foreign_powers.len() >= 12);
    }

    #[test]
    fn country_splits_rebel_formation_and_programs_work() {
        let mut state = NationState::from_preset(ScenarioPreset::AureliaDemocracy);
        state.political_capital = 100.0;
        state.treasury_bn = 200.0;
        assert!(state.foreign_powers.len() >= 12);

        // Diplomatic visit
        assert!(diplomatic_action(&mut state, ForeignPowerId::BharatUnion, "diplomatic_visit").is_ok());
        assert_eq!(state.programs.diplomatic_visits, 1);

        // Foment separatist split -> spawns a brand-new emergent country
        let before_n = state.foreign_powers.len();
        assert!(diplomatic_action(&mut state, ForeignPowerId::EurasianFederation, "fund_proxies").is_ok());
        assert!(diplomatic_action(&mut state, ForeignPowerId::EurasianFederation, "foment_split").is_ok());
        assert_eq!(state.foreign_powers.len(), before_n + 1);
        assert!(state.foreign_powers.iter().any(|p| matches!(p.id, ForeignPowerId::Emergent(_))));

        // Fighter Jet Programme, Flight Test, Nuclear Test & Speech activities
        assert!(run_strategic_activity(&mut state, "fighter_advance").is_ok());
        assert_eq!(state.programs.fighter_jet_tier, 1);
        assert!(run_strategic_activity(&mut state, "fighter_test").is_ok());
        assert_eq!(state.programs.fighter_tests, 1);

        state.nuclear_status = NuclearStatus::TacticalArsenal;
        assert!(run_strategic_activity(&mut state, "nuke_test").is_ok());
        assert_eq!(state.programs.nuke_tests_conducted, 1);

        assert!(run_strategic_activity(&mut state, "speech_patriotic").is_ok());
        assert_eq!(state.programs.speeches_delivered, 1);

        // Domestic rebel secession spawns hostile rebel state at war
        assert!(spawn_domestic_rebel_state(&mut state).is_ok());
        assert_eq!(state.rebel_states_count(), 1);
        assert_eq!(state.active_wars_count(), 1);
    }

    #[test]
    fn fictional_world_branches_arms_sales_and_pandemic_lockdown_work() {
        let mut state = NationState::from_preset(ScenarioPreset::AureliaDemocracy);
        state.political_capital = 100.0;
        state.treasury_bn = 250.0;

        // 18 fictional countries total, zero real-world "Bharat" name
        assert_eq!(state.foreign_powers.len(), 18);
        assert!(state.foreign_powers.iter().all(|p| !p.name.contains("Bharat")));

        // Army, Air Force & Navy procurement
        let d0 = state.armed_forces.army_divisions;
        let s0 = state.armed_forces.fighter_squadrons;
        let n0 = state.armed_forces.destroyers;
        assert!(run_strategic_activity(&mut state, "procure_army_division").is_ok());
        assert!(run_strategic_activity(&mut state, "procure_airforce_squadron").is_ok());
        assert!(run_strategic_activity(&mut state, "procure_navy_fleet").is_ok());
        assert!(state.armed_forces.army_divisions > d0);
        assert!(state.armed_forces.fighter_squadrons > s0);
        assert!(state.armed_forces.destroyers > n0);

        // Global & bilateral weapons sales
        assert!(run_strategic_activity(&mut state, "sell_defense_weapons").is_ok());
        assert!(diplomatic_action(&mut state, ForeignPowerId::LuminaRepublic, "sell_arms").is_ok());
        assert_eq!(state.armed_forces.weapons_contracts_signed, 2);
        assert!(state.armed_forces.total_arms_sales_bn > 40.0);

        // Pandemic outbreak, lockdown switch & vaccine sprint eradication
        trigger_pandemic_outbreak(&mut state, "Test Strain-Z", 48.0, LockdownLevel::None);
        assert!(state.pandemic.active);
        set_lockdown_level(&mut state, LockdownLevel::Nationwide);
        assert_eq!(state.pandemic.lockdown_level, LockdownLevel::Nationwide);
        state.pandemic.vaccine_progress = 75.0;
        assert!(run_strategic_activity(&mut state, "pandemic_vaccine_sprint").is_ok());
        assert!(!state.pandemic.active);
        assert_eq!(state.pandemic.lockdown_level, LockdownLevel::None);
    }

    #[test]
    fn custom_leader_profile_applies_to_presets_and_ages_each_year() {
        let mut state = NationState::from_preset(ScenarioPreset::AureliaDemocracy);
        let base_tech = state.tech_level;
        let profile = LeaderProfile {
            age: 35,
            education: "Ph.D. Aerospace Engineering".into(),
            background: LeaderBackground::Scientist,
            bio: "Built the nation's first orbital launch system.".into(),
            motto: "To the Stars".into(),
        };
        state.apply_leader_profile("Gautham Nair", "Prime Minister", profile);
        assert_eq!(state.leader_name, "Gautham Nair");
        assert_eq!(state.leader_title, "Prime Minister");
        assert_eq!(state.leader_profile.age, 35);
        assert!(state.tech_level > base_tech);

        // Advance 4 quarters -> leader turns 36
        for _ in 0..4 {
            state.active_event = None;
            advance_quarter(&mut state);
        }
        assert_eq!(state.leader_profile.age, 36);
    }
}
