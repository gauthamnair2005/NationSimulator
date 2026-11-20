use crate::models::*;

#[derive(Debug, Clone)]
pub struct DecisionDef {
    pub id: &'static str,
    pub name: &'static str,
    pub category: LogCategory,
    pub political_cost: f64,
    pub treasury_cost_bn: f64,
    pub repeatable_cooldown: Option<u32>, // None = one-time structural reform
    pub description: &'static str,
    pub effects_summary: &'static str,
}

pub fn all_decisions() -> Vec<DecisionDef> {
    vec![
        // ECONOMIC & IDEOLOGICAL TRANSITIONS
        DecisionDef {
            id: "nationalize_commanding_heights",
            name: "Nationalize Commanding Heights",
            category: LogCategory::Economic,
            political_cost: 28.0,
            treasury_cost_bn: 12.0,
            repeatable_cooldown: None,
            description: "Seize energy, banking, and heavy manufacturing into state syndicates, pivoting the country toward Planned Communism.",
            effects_summary: "Economy -> Planned Economy, State Ownership +40%, Labor +18, Capitalists -35, Gini -8, UAR Relations -25",
        },
        DecisionDef {
            id: "shock_therapy_capitalism",
            name: "Free-Market Shock Therapy",
            category: LogCategory::Economic,
            political_cost: 26.0,
            treasury_cost_bn: 0.0,
            repeatable_cooldown: None,
            description: "Mass privatization of state enterprises, sweeping deregulation, and capital liberalization to unleash capitalist growth.",
            effects_summary: "Economy -> Free-Market Capitalism, State Ownership -45%, Treasury +22B, Capitalists +25, Labor -24, Gini +9",
        },
        DecisionDef {
            id: "social_market_compact",
            name: "Tripartite Social Market Compact",
            category: LogCategory::Economic,
            political_cost: 22.0,
            treasury_cost_bn: 8.0,
            repeatable_cooldown: None,
            description: "Broker a national bargain between unions, corporations, and the state for balanced mixed-market prosperity.",
            effects_summary: "Economy -> Mixed Economy, Stability +10, Labor +10, Capitalists +6, Inequality -4",
        },
        DecisionDef {
            id: "five_year_industrial_plan",
            name: "Launch Five-Year Industrial Plan",
            category: LogCategory::Economic,
            political_cost: 18.0,
            treasury_cost_bn: 14.0,
            repeatable_cooldown: Some(12),
            description: "Direct state capital into steel, rail, power grids, and heavy manufacturing quotas.",
            effects_summary: "Infrastructure +10, Unemployment -2.5%, Inflation +1.4%, Labor +8, Five-Year Plan active",
        },
        DecisionDef {
            id: "universal_healthcare_act",
            name: "Universal Public Healthcare Act",
            category: LogCategory::Economic,
            political_cost: 24.0,
            treasury_cost_bn: 16.0,
            repeatable_cooldown: None,
            description: "Guarantee single-payer medical coverage to all citizens funded through progressive revenue.",
            effects_summary: "Healthcare +14, Approval +10, Gini -4, Labor +12, Capitalists -8",
        },
        DecisionDef {
            id: "sovereign_wealth_fund",
            name: "Establish Sovereign Wealth Fund",
            category: LogCategory::Economic,
            political_cost: 20.0,
            treasury_cost_bn: 25.0,
            repeatable_cooldown: None,
            description: "Invest sovereign reserves in global equities and strategic assets for permanent fiscal returns.",
            effects_summary: "Permanent quarterly passive income boost, Credit Rating stability, Capitalists +8",
        },
        // POLITICAL & CONSTITUTIONAL REGIME REFORMS
        DecisionDef {
            id: "constitutional_democratization",
            name: "Constitutional Convention (Democratize)",
            category: LogCategory::Political,
            political_cost: 30.0,
            treasury_cost_bn: 5.0,
            repeatable_cooldown: None,
            description: "Draft a multi-party democratic constitution with term limits, independent courts, and free press.",
            effects_summary: "Regime -> Liberal Democracy, Democracy +35, Liberties +30, Intelligentsia +25, Security/Military Clout -12",
        },
        DecisionDef {
            id: "vanguard_party_decree",
            name: "Proclaim People's Vanguard State",
            category: LogCategory::Political,
            political_cost: 32.0,
            treasury_cost_bn: 6.0,
            repeatable_cooldown: None,
            description: "Dissolve bourgeois parties, consolidate power in the Central Committee, and align with socialist blocs.",
            effects_summary: "Regime -> One-Party Communist, Democracy -35, PSU Relations +30, UAR -35, Labor +12, Capitalists -35",
        },
        DecisionDef {
            id: "declare_autocratic_presidency",
            name: "Enabling Act: Executive Supremacy",
            category: LogCategory::Political,
            political_cost: 28.0,
            treasury_cost_bn: 4.0,
            repeatable_cooldown: None,
            description: "Suspend parliamentary oversight, abolish term limits, and rule via supreme executive decree.",
            effects_summary: "Regime -> Absolute Autocracy, Political Capital +20, Liberties -28, Intelligentsia -30, Security +15",
        },
        DecisionDef {
            id: "clerical_guardianship_amendment",
            name: "Supreme Clerical Guardianship",
            category: LogCategory::Religious,
            political_cost: 28.0,
            treasury_cost_bn: 5.0,
            repeatable_cooldown: None,
            description: "Elevate the Council of Clerics as ultimate constitutional authority over legislation and judiciary.",
            effects_summary: "Regime -> Theocratic Republic, Theocratic Law, Clergy +30, Intelligentsia -28, Crescent Relations +28",
        },
        DecisionDef {
            id: "anti_corruption_tribunal",
            name: "Anti-Corruption Special Tribunal",
            category: LogCategory::Political,
            political_cost: 18.0,
            treasury_cost_bn: 4.0,
            repeatable_cooldown: Some(8),
            description: "Prosecute embezzling ministers, oligarchs, and procurement syndicates.",
            effects_summary: "Corruption -12, Rule of Law +8, Approval +6, Capitalists -6, Security -5",
        },
        DecisionDef {
            id: "purge_officer_corps",
            name: "Loyalty Purge of Officer Corps",
            category: LogCategory::Military,
            political_cost: 20.0,
            treasury_cost_bn: 3.0,
            repeatable_cooldown: Some(8),
            description: "Arrest suspected plotters in the general staff and promote loyalists.",
            effects_summary: "Coup Risk -22 immediately, Military Clout -14, Military Readiness -16, Military Approval -12",
        },
        // RELIGION & COUNTER-EXTREMISM
        DecisionDef {
            id: "deradicalization_initiative",
            name: "National Deradicalization & Civic Education",
            category: LogCategory::Religious,
            political_cost: 16.0,
            treasury_cost_bn: 9.0,
            repeatable_cooldown: None,
            description: "Fund moderate community leaders, counter-extremist curricula, and youth employment pipelines.",
            effects_summary: "Ongoing decay to Religious & Ideological Extremism, Education +6, Intelligentsia +8",
        },
        DecisionDef {
            id: "counter_terror_sweep",
            name: "Joint Counter-Insurgency Offensive",
            category: LogCategory::Military,
            political_cost: 14.0,
            treasury_cost_bn: 7.0,
            repeatable_cooldown: Some(4),
            description: "Deploy special forces and intelligence teams to dismantle militant cells and weapons caches.",
            effects_summary: "Insurgency -18, Extremism -10, Liberties -5, Military +6, Security +8",
        },
        DecisionDef {
            id: "digital_surveillance_grid",
            name: "Biometric & Digital Surveillance Grid",
            category: LogCategory::Political,
            political_cost: 22.0,
            treasury_cost_bn: 15.0,
            repeatable_cooldown: None,
            description: "Deploy AI-driven telecom monitoring nationwide to pre-empt coups, riots, and terror plots.",
            effects_summary: "Coup Risk -10, Insurgency growth suppressed, Civil Liberties -22, Security Clout +15",
        },
        // MILITARY & STRATEGIC
        DecisionDef {
            id: "defense_modernization",
            name: "Armed Forces Modernization Program",
            category: LogCategory::Military,
            political_cost: 15.0,
            treasury_cost_bn: 18.0,
            repeatable_cooldown: Some(8),
            description: "Procure drones, air defense batteries, armored divisions, and encrypted command systems.",
            effects_summary: "Military Tech +10, Readiness +12, Military Approval +12, Nationalists +6",
        },
        DecisionDef {
            id: "nuclear_program_advance",
            name: "Advance Sovereign Nuclear Program",
            category: LogCategory::Military,
            political_cost: 25.0,
            treasury_cost_bn: 24.0,
            repeatable_cooldown: Some(6),
            description: "Accelerate enrichment, warhead miniaturization, or missile delivery systems.",
            effects_summary: "Nuclear Progress +55%, Military +10, Nationalists +10, Severe UAR/GMC diplomatic backlash if weaponizing",
        },
        DecisionDef {
            id: "emergency_imf_bailout",
            name: "Request GMC Emergency Bailout Loan",
            category: LogCategory::Economic,
            political_cost: 12.0,
            treasury_cost_bn: 0.0,
            repeatable_cooldown: Some(12),
            description: "Secure $60B emergency liquidity from the Global Monetary Council in exchange for 8 quarters of austerity mandates.",
            effects_summary: "Treasury +$60B, Debt +$60B, 8 Qtrs Austerity, Approval -10, Labor -14, GMC +15",
        },
        DecisionDef {
            id: "issue_sovereign_bonds",
            name: "Issue Emergency Sovereign Bonds",
            category: LogCategory::Economic,
            political_cost: 6.0,
            treasury_cost_bn: 0.0,
            repeatable_cooldown: Some(2),
            description: "Raise $30B on international debt markets to cover deficits or finance urgent operations.",
            effects_summary: "Treasury +$30B, National Debt +$32B, Inflation +0.5%",
        },
    ]
}

pub fn can_enact(state: &NationState, def: &DecisionDef) -> Result<(), String> {
    if state.political_capital < def.political_cost {
        return Err(format!("Need {:.0} Political Capital", def.political_cost));
    }
    if state.treasury_bn < def.treasury_cost_bn {
        return Err(format!("Need ${:.1}B Treasury", def.treasury_cost_bn));
    }
    if def.repeatable_cooldown.is_none() && state.enacted_decisions.iter().any(|d| d == def.id) {
        return Err("Already enacted".into());
    }
    if let Some(&cd) = state.decision_cooldowns.get(def.id) {
        if cd > 0 {
            return Err(format!("Cooldown: {} qtr(s)", cd));
        }
    }
    if def.id == "nuclear_program_advance" && state.nuclear_status == NuclearStatus::StrategicTriad {
        return Err("Strategic Triad already complete".into());
    }
    Ok(())
}

pub fn enact_decision(state: &mut NationState, id: &str) -> Result<String, String> {
    let defs = all_decisions();
    let def = defs
        .iter()
        .find(|d| d.id == id)
        .ok_or_else(|| "Unknown decision".to_string())?;
    can_enact(state, def)?;

    state.political_capital = (state.political_capital - def.political_cost).max(0.0);
    state.treasury_bn -= def.treasury_cost_bn;

    if let Some(cd) = def.repeatable_cooldown {
        state.decision_cooldowns.insert(def.id.to_string(), cd);
    } else {
        state.enacted_decisions.push(def.id.to_string());
    }

    let summary = match def.id {
        "nationalize_commanding_heights" => {
            state.economy = EconomicSystem::PlannedCommunism;
            state.taxes.state_ownership = (state.taxes.state_ownership + 40.0).min(95.0);
            state.gini = (state.gini - 8.0).max(20.0);
            state.stability = (state.stability - 8.0).max(0.0);
            state.adjust_faction(FactionId::LaborUnions, 18.0, 14.0);
            state.adjust_faction(FactionId::Capitalists, -35.0, -24.0);
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 25.0;
            state.foreign_mut(ForeignPowerId::PeoplesUnion).relation += 24.0;
            "Major banks, energy, and heavy industries have been nationalized into state syndicates."
        }
        "shock_therapy_capitalism" => {
            state.economy = EconomicSystem::FreeMarketCapitalism;
            state.taxes.state_ownership = (state.taxes.state_ownership - 45.0).max(4.0);
            state.taxes.corporate_tax = state.taxes.corporate_tax.min(18.0);
            state.treasury_bn += 22.0;
            state.gini = (state.gini + 9.0).min(70.0);
            state.unemployment = (state.unemployment + 2.2).min(35.0);
            state.adjust_faction(FactionId::Capitalists, 25.0, 20.0);
            state.adjust_faction(FactionId::LaborUnions, -24.0, -14.0);
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation += 20.0;
            state.foreign_mut(ForeignPowerId::GlobalMonetaryCouncil).relation += 18.0;
            "State monopolies were auctioned to private capital; markets surge amid fierce labor protests."
        }
        "social_market_compact" => {
            state.economy = EconomicSystem::MixedEconomy;
            state.taxes.state_ownership = state.taxes.state_ownership.clamp(22.0, 42.0);
            state.stability = (state.stability + 10.0).min(100.0);
            state.gini = (state.gini - 4.0).max(22.0);
            state.adjust_faction(FactionId::LaborUnions, 10.0, 4.0);
            state.adjust_faction(FactionId::Capitalists, 6.0, 2.0);
            "Labor federations and industrialists ratified a mixed-market social compact."
        }
        "five_year_industrial_plan" => {
            state.flags.five_year_plan = true;
            state.infrastructure_level = (state.infrastructure_level + 10.0).min(100.0);
            state.unemployment = (state.unemployment - 2.5).max(1.5);
            state.inflation += 1.4;
            state.adjust_faction(FactionId::LaborUnions, 8.0, 5.0);
            "Central planners launched a sweeping Five-Year Industrial Drive."
        }
        "universal_healthcare_act" => {
            state.flags.universal_healthcare = true;
            state.healthcare_level = (state.healthcare_level + 14.0).min(100.0);
            state.approval = (state.approval + 10.0).min(100.0);
            state.gini = (state.gini - 4.0).max(20.0);
            state.budget.healthcare = state.budget.healthcare.max(68.0);
            state.adjust_faction(FactionId::LaborUnions, 12.0, 5.0);
            state.adjust_faction(FactionId::Capitalists, -8.0, -2.0);
            "Universal single-payer healthcare is now guaranteed nationwide."
        }
        "sovereign_wealth_fund" => {
            state.flags.sovereign_wealth_fund = true;
            state.adjust_faction(FactionId::Capitalists, 8.0, 5.0);
            "The Sovereign Wealth Fund has begun compounding global investment dividends."
        }
        "constitutional_democratization" => {
            state.regime = RegimeType::LiberalDemocracy;
            state.democracy_index = (state.democracy_index + 35.0).min(95.0);
            state.civil_liberties = (state.civil_liberties + 30.0).min(95.0);
            state.press_freedom = (state.press_freedom + 32.0).min(95.0);
            state.legitimacy = (state.legitimacy + 16.0).min(100.0);
            state.flags.independent_judiciary = true;
            state.flags.press_censorship = false;
            state.flags.opposition_banned = false;
            state.flags.term_limits_abolished = false;
            state.flags.martial_law = false;
            state.quarters_until_election = 12;
            state.adjust_faction(FactionId::Intelligentsia, 25.0, 15.0);
            state.adjust_faction(FactionId::Military, -8.0, -12.0);
            state.adjust_faction(FactionId::SecurityApparatus, -12.0, -12.0);
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation += 25.0;
            "A new democratic constitution restored multiparty elections, free media, and judicial independence."
        }
        "vanguard_party_decree" => {
            state.regime = RegimeType::OnePartyCommunist;
            state.democracy_index = (state.democracy_index - 35.0).max(8.0);
            state.civil_liberties = (state.civil_liberties - 25.0).max(10.0);
            state.press_freedom = (state.press_freedom - 30.0).max(8.0);
            state.flags.opposition_banned = true;
            state.flags.press_censorship = true;
            state.flags.secret_police = true;
            state.flags.independent_judiciary = false;
            state.adjust_faction(FactionId::LaborUnions, 12.0, 16.0);
            state.adjust_faction(FactionId::SecurityApparatus, 14.0, 14.0);
            state.adjust_faction(FactionId::Capitalists, -35.0, -20.0);
            state.adjust_faction(FactionId::Intelligentsia, -22.0, -8.0);
            state.foreign_mut(ForeignPowerId::PeoplesUnion).relation += 30.0;
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 35.0;
            "Opposition parties were dissolved as the Vanguard Party proclaimed a Socialist Republic."
        }
        "declare_autocratic_presidency" => {
            state.regime = RegimeType::AbsoluteAutocracy;
            state.political_capital = (state.political_capital + 20.0).min(100.0);
            state.democracy_index = (state.democracy_index - 35.0).max(6.0);
            state.civil_liberties = (state.civil_liberties - 28.0).max(10.0);
            state.flags.term_limits_abolished = true;
            state.flags.independent_judiciary = false;
            state.adjust_faction(FactionId::SecurityApparatus, 15.0, 15.0);
            state.adjust_faction(FactionId::Intelligentsia, -30.0, -10.0);
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 22.0;
            state.foreign_mut(ForeignPowerId::EurasianFederation).relation += 16.0;
            "Parliament rubber-stamped the Enabling Act, concentrating supreme power in your office."
        }
        "clerical_guardianship_amendment" => {
            state.regime = RegimeType::TheocraticRepublic;
            state.religion_policy = ReligionPolicy::TheocraticLaw;
            state.flags.religious_courts = true;
            state.clerical_power = (state.clerical_power + 32.0).min(98.0);
            state.religiosity = (state.religiosity + 14.0).min(98.0);
            state.civil_liberties = (state.civil_liberties - 24.0).max(8.0);
            state.adjust_faction(FactionId::Clergy, 30.0, 26.0);
            state.adjust_faction(FactionId::Intelligentsia, -28.0, -12.0);
            state.foreign_mut(ForeignPowerId::CrescentCoalition).relation += 28.0;
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 24.0;
            "The Supreme Clerical Council now presides over constitutional and moral law."
        }
        "anti_corruption_tribunal" => {
            state.corruption = (state.corruption - 12.0).max(4.0);
            state.rule_of_law = (state.rule_of_law + 8.0).min(100.0);
            state.approval = (state.approval + 6.0).min(100.0);
            state.adjust_faction(FactionId::Capitalists, -6.0, -4.0);
            state.adjust_faction(FactionId::SecurityApparatus, -5.0, -3.0);
            state.adjust_faction(FactionId::Intelligentsia, 8.0, 3.0);
            "High-profile corruption prosecutions seized illicit assets and bolstered public trust."
        }
        "purge_officer_corps" => {
            state.military_readiness = (state.military_readiness - 16.0).max(15.0);
            state.adjust_faction(FactionId::Military, -12.0, -16.0);
            state.adjust_faction(FactionId::SecurityApparatus, 8.0, 6.0);
            state.stability = (state.stability - 4.0).max(0.0);
            state.coup_risk = (state.coup_risk - 22.0).max(0.0);
            "Suspected coup-plotters in the High Command were detained and replaced by loyal officers."
        }
        "deradicalization_initiative" => {
            state.flags.deradicalization_program = true;
            state.education_level = (state.education_level + 6.0).min(100.0);
            state.religious_extremism = (state.religious_extremism - 8.0).max(0.0);
            state.ideological_extremism = (state.ideological_extremism - 8.0).max(0.0);
            state.adjust_faction(FactionId::Intelligentsia, 8.0, 4.0);
            "Nationwide deradicalization centers and civic dialogue programs are now operational."
        }
        "counter_terror_sweep" => {
            state.insurgency_strength = (state.insurgency_strength - 18.0).max(0.0);
            state.religious_extremism = (state.religious_extremism - 7.0).max(0.0);
            state.ideological_extremism = (state.ideological_extremism - 7.0).max(0.0);
            state.civil_liberties = (state.civil_liberties - 5.0).max(5.0);
            state.adjust_faction(FactionId::Military, 6.0, 3.0);
            state.adjust_faction(FactionId::SecurityApparatus, 8.0, 4.0);
            "Special forces raids dismantled militant cells and seized stockpiles of explosives."
        }
        "digital_surveillance_grid" => {
            state.flags.mass_surveillance = true;
            state.civil_liberties = (state.civil_liberties - 22.0).max(4.0);
            state.press_freedom = (state.press_freedom - 14.0).max(4.0);
            state.adjust_faction(FactionId::SecurityApparatus, 16.0, 15.0);
            state.adjust_faction(FactionId::Intelligentsia, -20.0, -6.0);
            "A nationwide biometric and telecom surveillance grid now monitors dissidents and conspirators."
        }
        "defense_modernization" => {
            state.military_tech = (state.military_tech + 10.0).min(100.0);
            state.military_readiness = (state.military_readiness + 12.0).min(100.0);
            state.adjust_faction(FactionId::Military, 12.0, 5.0);
            state.adjust_faction(FactionId::Nationalists, 6.0, 3.0);
            "Next-generation drones, air defenses, and armored brigades entered active service."
        }
        "nuclear_program_advance" => {
            state.nuclear_progress += 55.0;
            if state.nuclear_progress >= 100.0 {
                if let Some(next) = state.nuclear_status.next() {
                    state.nuclear_status = next;
                    state.nuclear_progress = 0.0;
                }
            }
            state.adjust_faction(FactionId::Military, 10.0, 4.0);
            state.adjust_faction(FactionId::Nationalists, 10.0, 4.0);
            if state.nuclear_status.stage() >= 2 {
                state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 24.0;
                state.foreign_mut(ForeignPowerId::GlobalMonetaryCouncil).relation -= 18.0;
            }
            "Centrifuge cascades and missile engineering teams hit major milestones."
        }
        "emergency_imf_bailout" => {
            state.treasury_bn += 60.0;
            state.national_debt_bn += 60.0;
            state.imf_austerity_turns = 8;
            state.approval = (state.approval - 10.0).max(0.0);
            state.adjust_faction(FactionId::LaborUnions, -14.0, -3.0);
            state.adjust_faction(FactionId::Nationalists, -10.0, 2.0);
            state.foreign_mut(ForeignPowerId::GlobalMonetaryCouncil).relation += 15.0;
            "The GMC disbursed $60B in emergency liquidity subject to 8 quarters of strict austerity."
        }
        "issue_sovereign_bonds" => {
            state.treasury_bn += 30.0;
            state.national_debt_bn += 32.0;
            state.inflation += 0.5;
            "The Treasury issued $30B in sovereign bonds to bolster liquidity."
        }
        _ => "Executive decree enacted.",
    };

    state.last_fiscal = crate::simulation::calculate_fiscal(state);
    state.coup_risk = crate::simulation::calculate_coup_risk(state);
    state.credit_rating = crate::simulation::compute_credit_rating(state);
    state.log(def.category, format!("Enacted: {}", def.name), summary);
    Ok(summary.to_string())
}
