use crate::models::*;

pub fn maybe_trigger_event(state: &mut NationState) {
    if state.active_event.is_some() || state.game_over.is_some() {
        return;
    }

    // 0. Check for any pending victorious war capitulation
    if let Some(target_id) = state
        .foreign_powers
        .iter()
        .find(|p| p.at_war && !p.annexed && p.war_score >= 95.0)
        .map(|p| p.id)
    {
        state.pending_event_target = Some(target_id);
        state.active_event = Some(build_post_war_victory_event(state, target_id));
        return;
    }

    // 1. Election Cycle in Democratic / Semi-Democratic regimes
    if state.regime.is_democratic() && state.quarters_until_election == 0 && !state.flags.term_limits_abolished {
        state.active_event = Some(build_election_event(state));
        return;
    }

    // 1b. Easter Egg God Mode: suppress negative crises & roll player-favoring Golden Mandate incidents!
    if state.flags.ledger_rebalance {
        maybe_trigger_god_mode_incident(state);
        return;
    }

    // 2. High-priority Crisis Events driven by real state thresholds
    let mut candidates: Vec<(&'static str, f64)> = Vec::new();

    if state.coup_risk > 58.0 && cooldown_ready(state, "military_coup_attempt") {
        let chance = ((state.coup_risk - 50.0) / 70.0).clamp(0.15, 0.85);
        candidates.push(("military_coup_attempt", chance * 3.0));
    }

    if (state.gini > 48.0 || state.faction(FactionId::LaborUnions).approval < 32.0)
        && state.economy != EconomicSystem::PlannedCommunism
        && cooldown_ready(state, "communist_general_strike")
    {
        candidates.push(("communist_general_strike", 0.55));
    }

    if (state.religious_extremism > 45.0 || state.sectarian_tension > 48.0)
        && cooldown_ready(state, "extremist_insurgency_attack")
    {
        candidates.push(("extremist_insurgency_attack", 0.65));
    }

    if (state.inflation > 11.0 || state.debt_to_gdp() > 115.0)
        && cooldown_ready(state, "debt_currency_crisis")
    {
        candidates.push(("debt_currency_crisis", 0.65));
    }

    if state.regime.is_authoritarian()
        && state.faction(FactionId::Intelligentsia).approval < 36.0
        && state.approval < 48.0
        && cooldown_ready(state, "pro_democracy_uprising")
    {
        candidates.push(("pro_democracy_uprising", 0.55));
    }

    if state.taxes.corporate_tax > 42.0
        && state.economy != EconomicSystem::PlannedCommunism
        && state.faction(FactionId::Capitalists).approval < 35.0
        && cooldown_ready(state, "capital_flight_panic")
    {
        candidates.push(("capital_flight_panic", 0.50));
    }

    if state.nuclear_status == NuclearStatus::SecretEnrichment
        && cooldown_ready(state, "nuclear_satellite_leak")
    {
        candidates.push(("nuclear_satellite_leak", 0.60));
    }

    if state.foreign_powers.iter().any(|p| {
        p.id == ForeignPowerId::BorderlandRepublic
            && p.relation < -25.0
            && !p.at_war
            && !p.annexed
            && !p.puppet_state
    }) && cooldown_ready(state, "border_flashpoint")
    {
        candidates.push(("border_flashpoint", 0.45));
    }

    if state.religion_policy == ReligionPolicy::StateAtheism
        && state.religiosity > 35.0
        && cooldown_ready(state, "clerical_fatwa_resistance")
    {
        candidates.push(("clerical_fatwa_resistance", 0.50));
    }

    if state.religion_policy == ReligionPolicy::TheocraticLaw
        && state.faction(FactionId::Intelligentsia).approval < 40.0
        && cooldown_ready(state, "blasphemy_law_protests")
    {
        candidates.push(("blasphemy_law_protests", 0.50));
    }

    if state.economy == EconomicSystem::PlannedCommunism
        && state.taxes.state_ownership > 75.0
        && state.gdp_growth < 1.2
        && cooldown_ready(state, "supply_shortage_breadlines")
    {
        candidates.push(("supply_shortage_breadlines", 0.50));
    }

    // 3. General real-world geopolitical & domestic events (occur periodically)
    let ambient = [
        "cyberattack_grid",
        "whistleblower_corruption",
        "rare_earth_discovery",
        "global_oil_shock",
        "far_right_militia_rally",
        "foreign_propaganda_campaign",
        "pandemic_outbreak",
        "oligarch_media_buyout",
    ];

    if candidates.is_empty() {
        // 38% chance of an ambient event each quarter
        if state.next_rand() < 0.38 {
            let ready: Vec<&'static str> = ambient
                .iter()
                .copied()
                .filter(|id| cooldown_ready(state, id))
                .collect();
            if !ready.is_empty() {
                let idx = (state.next_rand() * ready.len() as f64) as usize % ready.len();
                let chosen = ready[idx];
                state.event_cooldowns.insert(chosen.to_string(), 14);
                state.active_event = Some(build_event(chosen, state));
            }
        }
        return;
    }

    // Pick highest-weight crisis stochastically
    let total_w: f64 = candidates.iter().map(|(_, w)| *w).sum();
    let mut roll = state.next_rand() * (total_w + 0.35);
    for (id, w) in candidates {
        if roll <= w {
            state.event_cooldowns.insert(id.to_string(), 10);
            state.active_event = Some(build_event(id, state));
            return;
        }
        roll -= w;
    }
}

fn maybe_trigger_god_mode_incident(state: &mut NationState) {
    // High-priority wartime battlefield miracle when engaged in war
    if state.active_wars_count() > 0 && cooldown_ready(state, "god_enemy_command_defection") {
        if state.next_rand() < 0.72 {
            state.event_cooldowns.insert("god_enemy_command_defection".into(), 4);
            state.active_event = Some(build_event("god_enemy_command_defection", state));
            return;
        }
    }

    // ~85% chance at quarter end for an exciting player-favoring Golden Mandate scenario/incident
    if state.next_rand() >= 0.85 {
        return;
    }

    let mut pool: Vec<&'static str> = vec![
        "god_sovereign_tech_singularity",
        "god_energy_fusion_miracle",
        "god_global_brain_drain_inflow",
        "god_historic_diplomatic_triumph",
        "god_supergiant_resource_discovery",
        "god_orbital_space_program",
        "god_biotech_longevity_cure",
        "god_tax_haven_capital_tsunami",
        "god_rival_spy_ring_turned",
        "god_hypersonic_armada_unveiled",
        "god_global_olympics_expo",
        "god_climate_geoengineering_triumph",
        "god_cyber_dominance_windfall",
        "god_border_voluntary_federation",
        "god_sovereign_golden_constitution",
    ];
    if state.active_wars_count() == 0 {
        pool.push("god_cultural_renaissance");
    }

    let conquerable_targets: Vec<ForeignPowerId> = state
        .foreign_powers
        .iter()
        .filter(|p| p.is_conquerable() && !p.puppet_state && !p.at_war)
        .map(|p| p.id)
        .collect();
    if !conquerable_targets.is_empty() {
        pool.push("god_rival_regime_collapse");
    }

    let ready: Vec<&'static str> = pool
        .into_iter()
        .filter(|id| cooldown_ready(state, id))
        .collect();
    if ready.is_empty() {
        return;
    }

    let idx = (state.next_rand() * ready.len() as f64) as usize % ready.len();
    let chosen = ready[idx];
    if chosen == "god_rival_regime_collapse" && !conquerable_targets.is_empty() {
        let tidx = (state.next_rand() * conquerable_targets.len() as f64) as usize
            % conquerable_targets.len();
        state.pending_event_target = Some(conquerable_targets[tidx]);
    }
    state.event_cooldowns.insert(chosen.to_string(), 5);
    state.active_event = Some(build_event(chosen, state));
}

pub fn build_post_war_victory_event(state: &NationState, target: ForeignPowerId) -> ActiveEvent {
    let p = state.foreign(target);
    let pop_gain = (p.economic_power * 0.28 + p.military_power * 0.14).max(5.0);
    let gdp_gain = (p.economic_power * 6.5).max(80.0);
    let seized = (p.economic_power * 1.1).max(20.0);
    let troops = (p.military_power * 1.4).max(15.0);
    let indemnity = 22.0 + p.economic_power * 0.45;
    let tribute = p.economic_power * 0.22;
    let reparations = 28.0 + p.economic_power * 0.45;

    let annex_friction = if state.flags.ledger_rebalance {
        "Golden Mandate active: smooth integration, Stability +8, zero insurgency or diplomatic backlash."
    } else {
        "Insurgency +12, Sectarian Tension +10, independent rivals Relations -20."
    };

    ActiveEvent {
        id: "post_war_victory".into(),
        title: format!("TOTAL VICTORY: {} CAPITULATES!", p.name.to_uppercase()),
        category: LogCategory::Military,
        urgency: "CONQUEST & PEACE CONFERENCE".into(),
        briefing: format!(
            "Our armed forces have secured total military victory over {}! Enemy generals have signed an unconditional surrender in their capital. As supreme commander of {}, you must decide the historic fate of the defeated nation.",
            p.name, state.nation_name
        ),
        choices: vec![
            EventChoice {
                id: "annex_territory".into(),
                label: format!("Conquer & Annex {} Completely", p.name),
                description: format!(
                    "Dissolve {}'s sovereignty and incorporate all provinces, industry, and population permanently into {}.",
                    p.name, state.nation_name
                ),
                impact_hint: format!(
                    "+{:.1}M Pop, +${:.0}B GDP, +${:.0}B Treasury, +{:.0}K Troops, Nationalists +18, Military +14. {}",
                    pop_gain, gdp_gain, seized, troops, annex_friction
                ),
            },
            EventChoice {
                id: "install_puppet".into(),
                label: format!("Install Compliant Client Regime in {}", p.short_name),
                description: "Replace hostile leadership with a loyal puppet government bound by permanent trade, defense, and tribute treaties.".into(),
                impact_hint: format!(
                    "Client State created (Ally + Trade Pact + +${:.1}B/qtr tribute), Treasury +${:.0}B indemnity. Can be annexed peacefully later.",
                    tribute, indemnity
                ),
            },
            EventChoice {
                id: "impose_reparations".into(),
                label: "Enforce Harsh Armistice, Demilitarization & Reparations".into(),
                description: "Withdraw occupation troops in exchange for massive cash indemnities and dismantling of enemy armed forces.".into(),
                impact_hint: format!(
                    "Treasury +${:.0}B, Enemy Military -35%, Approval +12, War Exhaustion -20, avoids occupation unrest.",
                    reparations
                ),
            },
        ],
    }
}

fn cooldown_ready(state: &NationState, id: &str) -> bool {
    state.event_cooldowns.get(id).copied().unwrap_or(0) == 0
}

fn build_election_event(state: &NationState) -> ActiveEvent {
    let win_prob = if state.flags.ledger_rebalance {
        100.0
    } else {
        (state.approval + state.legitimacy * 0.25 - 12.0).clamp(5.0, 95.0)
    };
    ActiveEvent {
        id: "national_election".into(),
        title: format!("{} National General Election", state.year),
        category: LogCategory::Political,
        urgency: "CONSTITUTIONAL MANDATE".into(),
        briefing: format!(
            "Citizens are heading to the ballot boxes across {}. Pre-election polling places your coalition's projected victory probability at {:.0}% (Public Approval: {:.1}%, Economy Growth: {:+.1}%). Your advisers await your final directive on the electoral commission.",
            state.nation_name, win_prob, state.approval, state.gdp_growth
        ),
        choices: vec![
            EventChoice {
                id: "fair_election".into(),
                label: "Guarantee Free & Fair Elections".into(),
                description: "Allow international observers and respect the sovereign verdict of the voters.".into(),
                impact_hint: format!("~{:.0}% chance to win re-election; losing ends your term peacefully. Democracy +6, Legitimacy +8.", win_prob),
            },
            EventChoice {
                id: "stimulus_campaign".into(),
                label: "Emergency Pre-Election Populist Spending ($15B)".into(),
                description: "Slash fuel prices, issue cash rebates, and saturate state media before polling day.".into(),
                impact_hint: "Treasury -$15B, Inflation +1.2%, boosts win odds by +18%.".into(),
            },
            EventChoice {
                id: "rig_ballots".into(),
                label: "Covertly Manipulate Ballot Counts".into(),
                description: "Instruct the Security Apparatus to stuff swing-district tallies and disqualify key rivals.".into(),
                impact_hint: "Guaranteed victory, but Democracy -24, Legitimacy -20, Stability -14, UAR -20, risk of uprising.".into(),
            },
            EventChoice {
                id: "cancel_election_emergency".into(),
                label: "Invoke State of Emergency & Suspend Elections".into(),
                description: "Deploy troops to parliament and dissolve the electoral schedule indefinitely.".into(),
                impact_hint: "Regime -> Absolute Autocracy, Martial Law ON, Liberties -32, Coup Risk +12.".into(),
            },
        ],
    }
}

fn build_event(id: &str, state: &NationState) -> ActiveEvent {
    match id {
        "military_coup_attempt" => ActiveEvent {
            id: id.into(),
            title: "FLASH ALERT: ARMORED COLUMNS MARCH ON THE CAPITAL!".into(),
            category: LogCategory::Crisis,
            urgency: "EXISTENTIAL THREAT".into(),
            briefing: format!(
                "Rogue generals from the 4th Armored Division have seized the state broadcast tower and surrounded the presidential compound in {}! They cite declining military prestige and national instability, demanding your immediate resignation.",
                state.nation_name
            ),
            choices: vec![
                EventChoice {
                    id: "rally_loyalists".into(),
                    label: "Order Loyalist Guards & Intelligence to Crush the Putsch".into(),
                    description: "Engage rebel tank units in street fighting around the ministry district.".into(),
                    impact_hint: "Success scales with Security Clout & Approval. Failure results in Military Coup!".into(),
                },
                EventChoice {
                    id: "call_citizens_streets".into(),
                    label: "Broadcast Emergency Call for Citizens to Block Tanks".into(),
                    description: "Use mobile networks to summon hundreds of thousands of civilians onto the avenues.".into(),
                    impact_hint: "High success if Approval > 50%. Boosts Democracy & Legitimacy if victorious; bloody crackdown if fails.".into(),
                },
                EventChoice {
                    id: "power_sharing_junta".into(),
                    label: "Capitulate to a Military Power-Sharing Junta".into(),
                    description: "Remain nominal head of state while granting the High Command veto power and budget control.".into(),
                    impact_hint: "Survive in power, but Regime -> Military Dictatorship, Military Budget +20, Democracy -35.".into(),
                },
            ],
        },
        "communist_general_strike" => ActiveEvent {
            id: id.into(),
            title: "Revolutionary General Strike & Factory Occupations".into(),
            category: LogCategory::Economic,
            urgency: "HIGH PRIORITY".into(),
            briefing: "Coordinated syndicates and radical left vanguard cadres have shut down ports, rail hubs, and steel mills across the nation. Red banners demand immediate nationalization of major industries and wealth redistribution.".into(),
            choices: vec![
                EventChoice {
                    id: "concede_socialist_reforms".into(),
                    label: "Enact Sweeping Labor & Nationalization Concessions".into(),
                    description: "Raise corporate taxes, expand welfare, and give workers board representation.".into(),
                    impact_hint: "Labor +20, Capitalists -18, Gini -5, Stability +8, State Ownership +15%.".into(),
                },
                EventChoice {
                    id: "embrace_revolution".into(),
                    label: "Align with the Vanguard — Proclaim a Socialist Republic".into(),
                    description: "Transform the strike into a systemic revolution led by your administration.".into(),
                    impact_hint: "Regime -> One-Party Communist, Economy -> Planned Economy, Capitalists purged.".into(),
                },
                EventChoice {
                    id: "send_riot_police".into(),
                    label: "Deploy Riot Police & Break the Picket Lines".into(),
                    description: "Arrest union leaders under emergency economic sabotage statutes.".into(),
                    impact_hint: "Capitalists +12, Labor -22, Liberties -12, Ideological Extremism +10.".into(),
                },
            ],
        },
        "extremist_insurgency_attack" => ActiveEvent {
            id: id.into(),
            title: "Coordinated Extremist Bombings Shake Provincial Capitals".into(),
            category: LogCategory::Religious,
            urgency: "NATIONAL SECURITY".into(),
            briefing: "A radical fundamentalist insurgent network has claimed responsibility for simultaneous attacks on transit hubs and security outposts. Sectarian fury is boiling over and citizens demand decisive leadership.".into(),
            choices: vec![
                EventChoice {
                    id: "declare_martial_crackdown".into(),
                    label: "Authorize Full Counter-Terror Crackdown & Martial Law".into(),
                    description: "Suspend habeas corpus in restive provinces and launch sweeping military raids.".into(),
                    impact_hint: "Insurgency -16, Martial Law ON, Liberties -16, Security +10, risk of radicalizing youth.".into(),
                },
                EventChoice {
                    id: "targeted_intel_ops".into(),
                    label: "Precision Intelligence Raids + Interfaith Unity Summit".into(),
                    description: "Use covert teams to eliminate cell commanders while rallying moderate clergy ($8B).".into(),
                    impact_hint: "Treasury -$8B, Insurgency -10, Sectarian Tension -12, Extremism -8.".into(),
                },
                EventChoice {
                    id: "appease_clerical_hardliners".into(),
                    label: "Concede Moral Policing & Religious Laws to Pacify Hardliners".into(),
                    description: "Grant clerical authorities broader censorship and judicial power in exchange for condemning militants.".into(),
                    impact_hint: "Clergy +15, Clerical Power +14, Insurgency -6, Intelligentsia -18, Liberties -12.".into(),
                },
            ],
        },
        "debt_currency_crisis" => ActiveEvent {
            id: id.into(),
            title: "Sovereign Debt Panic & Currency Freefall".into(),
            category: LogCategory::Economic,
            urgency: "FINANCIAL EMERGENCY".into(),
            briefing: format!(
                "Bond yields have spiked past 14% as international investors dump national debt (Debt/GDP: {:.0}%, Inflation: {:.1}%). Banks warn of imminent insolvency unless confidence is restored immediately.",
                state.debt_to_gdp(), state.inflation
            ),
            choices: vec![
                EventChoice {
                    id: "imf_austerity_deal".into(),
                    label: "Sign Emergency GMC Structural Adjustment Package".into(),
                    description: "Slash public spending and raise interest rates in exchange for $50B credit line.".into(),
                    impact_hint: "Treasury +$50B, Inflation -3%, Approval -12, Labor -14, Austerity 6 Qtrs.".into(),
                },
                EventChoice {
                    id: "capital_controls_print".into(),
                    label: "Impose Capital Controls & Monetize Debt".into(),
                    description: "Freeze foreign withdrawals and order the central bank to print liquidity.".into(),
                    impact_hint: "Treasury +$25B, Capital Controls ON, Inflation +6%, Capitalists -22, Credit Rating hit.".into(),
                },
                EventChoice {
                    id: "selective_default".into(),
                    label: "Declare Selective Sovereign Debt Restructuring".into(),
                    description: "Force foreign bondholders to take a 35% haircut on sovereign obligations.".into(),
                    impact_hint: "National Debt -30%, GMC Relations -40, UAR -25, GDP -4%, Nationalists +10.".into(),
                },
            ],
        },
        "pro_democracy_uprising" => ActiveEvent {
            id: id.into(),
            title: "Mass Pro-Democracy Protests Fill Central Square".into(),
            category: LogCategory::Political,
            urgency: "REGIME CRISIS".into(),
            briefing: "Over 250,000 students, professionals, and civic activists have occupied the capital's main plaza demanding free elections, media freedom, and an end to authoritarian rule.".into(),
            choices: vec![
                EventChoice {
                    id: "open_democratic_reforms".into(),
                    label: "Promise Constitutional Reform & Lift Censorship".into(),
                    description: "Legalize opposition parties and schedule open parliamentary elections.".into(),
                    impact_hint: "Regime shifts toward Democracy, Approval +14, Intelligentsia +22, Security -15.".into(),
                },
                EventChoice {
                    id: "tiananmen_crackdown".into(),
                    label: "Clear the Square with Armored Security Divisions".into(),
                    description: "Impose total curfew, cut internet access, and crush the protest movement by force.".into(),
                    impact_hint: "Stability +6 short-term, Democracy -20, Liberties -22, UAR Sanctions, Coup/Extremism risk.".into(),
                },
                EventChoice {
                    id: "staged_concessions".into(),
                    label: "Dismiss Unpopular Ministers & Offer Youth Subsidies ($10B)".into(),
                    description: "Scapegoat cabinet officials and fund urban housing without surrendering core power.".into(),
                    impact_hint: "Treasury -$10B, Approval +6, Corruption -4, Intelligentsia +4.".into(),
                },
            ],
        },
        "capital_flight_panic" => ActiveEvent {
            id: id.into(),
            title: "Oligarch Capital Flight & Industrial Lockout".into(),
            category: LogCategory::Economic,
            urgency: "ECONOMIC SHOCK".into(),
            briefing: "Major conglomerates are wiring billions offshore and halting factory expansion in protest against steep taxation and state intervention.".into(),
            choices: vec![
                EventChoice {
                    id: "freeze_oligarch_assets".into(),
                    label: "Freeze Offshore Transfers & Seize Delinquent Assets".into(),
                    description: "Enforce strict capital controls and prosecute tax-evading billionaires.".into(),
                    impact_hint: "Treasury +$16B, Capital Controls ON, Capitalists -18, Labor +12, State Ownership +10%.".into(),
                },
                EventChoice {
                    id: "cut_corp_taxes".into(),
                    label: "Offer Corporate Tax Amnesty & Deregulation".into(),
                    description: "Lower corporate rates by 6% to coax domestic and foreign capital back.".into(),
                    impact_hint: "Corporate Tax -6%, Capitalists +18, GDP Growth +1.2%, Labor -10.".into(),
                },
            ],
        },
        "nuclear_satellite_leak" => ActiveEvent {
            id: id.into(),
            title: "Spy Satellites Expose Clandestine Enrichment Facility!".into(),
            category: LogCategory::Military,
            urgency: "DIPLOMATIC CRISIS".into(),
            briefing: "The United Atlantic Republic has released satellite imagery of your underground centrifuge complex before the UN Security Council, threatening crippling multilateral sanctions unless inspectors are admitted.".into(),
            choices: vec![
                EventChoice {
                    id: "defy_world_test".into(),
                    label: "Defy the Ultimatum & Accelerate Weaponization".into(),
                    description: "Expel monitors and sprint toward a functioning nuclear deterrent.".into(),
                    impact_hint: "Nuclear Progress +45%, UAR & GMC Sanctions imposed, Nationalists +16, Military +14.".into(),
                },
                EventChoice {
                    id: "negotiate_nuclear_deal".into(),
                    label: "Sign Nuclear Inspection Accord for Sanctions Relief".into(),
                    description: "Freeze enrichment at civilian levels in return for lifted sanctions and trade.".into(),
                    impact_hint: "Nuclear -> Civilian, UAR +24, GMC +20, Sanctions lifted, Nationalists -14.".into(),
                },
            ],
        },
        "border_flashpoint" => ActiveEvent {
            id: id.into(),
            title: "Artillery Exchange Along the Borderland Frontier!".into(),
            category: LogCategory::Military,
            urgency: "MILITARY FLASHPOINT".into(),
            briefing: "Borderland Republic patrols crossed a contested ridge, leaving six of our soldiers dead in an artillery skirmish. Generals demand authorization for a decisive retaliatory push.".into(),
            choices: vec![
                EventChoice {
                    id: "launch_full_war".into(),
                    label: "Authorize Full-Scale Military Intervention".into(),
                    description: "Cross the border in force to secure disputed provinces and topple hostile garrisons.".into(),
                    impact_hint: "Starts War with Borderland Republic! Nationalists +16, Military +12, War costs & diplomatic fallout.".into(),
                },
                EventChoice {
                    id: "limited_airstrikes".into(),
                    label: "Conduct Limited Retaliatory Drone & Artillery Strikes".into(),
                    description: "Destroy forward enemy depots without full invasion ($5B).".into(),
                    impact_hint: "Treasury -$5B, Borderland -18, Nationalists +6, Military +6, avoids full war.".into(),
                },
                EventChoice {
                    id: "diplomatic_deescalation".into(),
                    label: "Call Emergency Ceasefire & International Mediation".into(),
                    description: "Pull back units 5km and invite multilateral peacekeepers.".into(),
                    impact_hint: "Borderland +14, UAR +10, Nationalists -12, Military -8, Stability +4.".into(),
                },
            ],
        },
        "clerical_fatwa_resistance" => ActiveEvent {
            id: id.into(),
            title: "Underground Clerical Network Issues Decree of Resistance".into(),
            category: LogCategory::Religious,
            urgency: "SOCIAL UNREST".into(),
            briefing: "Defying State Atheism restrictions, prominent clerics have circulated a widely followed condemnation of the regime, sparking mass prayer protests across rural provinces.".into(),
            choices: vec![
                EventChoice {
                    id: "relax_to_secular".into(),
                    label: "Transition to Secular Pluralism & Reopen Houses of Worship".into(),
                    description: "Decriminalize private worship while keeping government institutions secular.".into(),
                    impact_hint: "Policy -> Secular Pluralism, Clergy +18, Religious Extremism -10, Stability +8.".into(),
                },
                EventChoice {
                    id: "arrest_clerics".into(),
                    label: "Arrest Dissident Clerics & Seize Underground Seminaries".into(),
                    description: "Enforce strict ideological conformity via state security.".into(),
                    impact_hint: "Clerical Power -12, Religious Extremism +14, Insurgency +8, Liberties -10.".into(),
                },
            ],
        },
        "blasphemy_law_protests" => ActiveEvent {
            id: id.into(),
            title: "Youth & Women's Movement Protests Morality Police".into(),
            category: LogCategory::Religious,
            urgency: "DOMESTIC CRISIS".into(),
            briefing: "Following a high-profile arrest by clerical morality enforcers, thousands of university students and urban professionals have launched nightly demonstrations demanding personal freedom.".into(),
            choices: vec![
                EventChoice {
                    id: "curb_morality_police".into(),
                    label: "Curb Morality Police & Reform Clerical Courts".into(),
                    description: "Downgrade to Official State Religion with civil protections for citizens.".into(),
                    impact_hint: "Policy -> Official Religion, Intelligentsia +16, Liberties +14, Clergy -16.".into(),
                },
                EventChoice {
                    id: "back_clerical_council".into(),
                    label: "Stand Firm with the Clerical Council & Suppress Riots".into(),
                    description: "Deploy Basij-style loyalist militias to restore traditional order.".into(),
                    impact_hint: "Clergy +14, Intelligentsia -18, Approval -8, Ideological Extremism +8.".into(),
                },
            ],
        },
        "supply_shortage_breadlines" => ActiveEvent {
            id: id.into(),
            title: "Consumer Shortages & Breadlines Hit Industrial Cities".into(),
            category: LogCategory::Economic,
            urgency: "ECONOMIC CRISIS".into(),
            briefing: "Rigid central quotas and bureaucratic bottlenecks have emptied grocery shelves across urban districts. Black markets are booming and public anger is rising.".into(),
            choices: vec![
                EventChoice {
                    id: "nep_market_reforms".into(),
                    label: "Permit Small Private Cooperatives (State Capitalism Pivot)".into(),
                    description: "Allow private retail, farming plots, and foreign joint ventures while keeping party control.".into(),
                    impact_hint: "Economy -> State Capitalism, GDP Growth +1.8%, Approval +8, Capitalists +14.".into(),
                },
                EventChoice {
                    id: "emergency_food_imports".into(),
                    label: "Spend Hard Currency on Emergency Imports ($14B)".into(),
                    description: "Purchase grain and consumer goods from allied blocs to clear the queues.".into(),
                    impact_hint: "Treasury -$14B, Approval +7, Stability +6.".into(),
                },
                EventChoice {
                    id: "crackdown_speculators".into(),
                    label: "Launch Anti-Speculator Campaign & Ration Cards".into(),
                    description: "Blame black-market hoarders and tighten state rationing.".into(),
                    impact_hint: "Approval -8, Corruption -4, Security +6, Liberties -6.".into(),
                },
            ],
        },
        "cyberattack_grid" => ActiveEvent {
            id: id.into(),
            title: "Foreign State-Backed Cyberattack Hits National Power Grid".into(),
            category: LogCategory::Military,
            urgency: "INFRASTRUCTURE ALERT".into(),
            briefing: "A sophisticated zero-day worm has knocked out substations in three industrial regions, causing $8B in economic disruption. Signals intelligence suspects foreign state hackers.".into(),
            choices: vec![
                EventChoice {
                    id: "cyber_retaliation".into(),
                    label: "Authorize Covert Cyber Counter-Strike & Harden Grid ($9B)".into(),
                    description: "Rebuild substations and strike back at hostile telecom nodes.".into(),
                    impact_hint: "Treasury -$9B, Tech +5, Security +10, Eurasia/PSU relations -12.".into(),
                },
                EventChoice {
                    id: "defensive_patch".into(),
                    label: "Invest in Sovereign Cyber Defense Agency ($12B)".into(),
                    description: "Focus purely on air-gapped critical infrastructure and resilience.".into(),
                    impact_hint: "Treasury -$12B, Infrastructure +6, Tech +7, Stability +5.".into(),
                },
            ],
        },
        "whistleblower_corruption" => ActiveEvent {
            id: id.into(),
            title: "Leaked Dossier Exposes Defense Ministry Kickback Ring".into(),
            category: LogCategory::Political,
            urgency: "POLITICAL SCANDAL".into(),
            briefing: "Investigative journalists have published encrypted contracts showing senior officials siphoned billions from public procurement into offshore shell accounts.".into(),
            choices: vec![
                EventChoice {
                    id: "prosecute_transparently".into(),
                    label: "Appoint Independent Prosecutor & Arrest Perpetrators".into(),
                    description: "Accept short-term embarrassment to clean house decisively.".into(),
                    impact_hint: "Corruption -10, Rule of Law +8, Approval +5, Security/Military -6.".into(),
                },
                EventChoice {
                    id: "suppress_press_leak".into(),
                    label: "Raid the Newspaper & Silence the Whistleblower".into(),
                    description: "Invoke state secrecy statutes to bury the dossier.".into(),
                    impact_hint: "Press Freedom -15, Corruption +5, Intelligentsia -14, Security +8.".into(),
                },
            ],
        },
        "rare_earth_discovery" => ActiveEvent {
            id: id.into(),
            title: "Massive Lithium & Rare-Earth Deposit Discovered!".into(),
            category: LogCategory::Economic,
            urgency: "STRATEGIC OPPORTUNITY".into(),
            briefing: "Geological surveys in the northern highlands confirmed one of the world's largest strategic mineral reserves, attracting fierce bids from foreign corporations and state syndicates.".into(),
            choices: vec![
                EventChoice {
                    id: "state_mining_corp".into(),
                    label: "Create a 100% State-Owned National Mining Corporation".into(),
                    description: "Retain full sovereign control over profits and supply chains ($10B startup).".into(),
                    impact_hint: "Treasury -$10B now, GDP +$24B, State Ownership +8%, Nationalists +10, Labor +8.".into(),
                },
                EventChoice {
                    id: "auction_private_licenses".into(),
                    label: "Auction Mining Concessions to Global Private Consortia".into(),
                    description: "Collect an immediate licensing windfall and rapid private extraction.".into(),
                    impact_hint: "Treasury +$26B, GDP +$18B, Capitalists +14, UAR +10, Nationalists -8.".into(),
                },
            ],
        },
        "global_oil_shock" => ActiveEvent {
            id: id.into(),
            title: "Global Energy Shock Spikes Fuel & Food Prices".into(),
            category: LogCategory::Economic,
            urgency: "MACRO SHOCK".into(),
            briefing: "Maritime chokepoint disruptions have doubled global crude prices, threatening runaway inflation and industrial slowdowns across importing economies.".into(),
            choices: vec![
                EventChoice {
                    id: "subsidize_fuel".into(),
                    label: "Enact Emergency Energy & Food Price Subsidies ($14B)".into(),
                    description: "Shield households and factories from the price spike at treasury expense.".into(),
                    impact_hint: "Treasury -$14B, Inflation capped (+0.8%), Approval +6, Labor +8.".into(),
                },
                EventChoice {
                    id: "hike_rates_market".into(),
                    label: "Let Prices Float & Hike Central Bank Interest Rates".into(),
                    description: "Protect fiscal reserves and fight inflation via tighter monetary policy.".into(),
                    impact_hint: "Interest Rate +2.5%, Inflation +1.5%, Approval -7, Capitalists +6.".into(),
                },
            ],
        },
        "far_right_militia_rally" => ActiveEvent {
            id: id.into(),
            title: "Ultranationalist Paramilitaries Rally in Border Provinces".into(),
            category: LogCategory::Political,
            urgency: "EXTREMISM WARNING".into(),
            briefing: "Armed ultranationalist militias are conducting unauthorized patrols and targeting minority neighborhoods, claiming the central government is weak.".into(),
            choices: vec![
                EventChoice {
                    id: "ban_disarm_militias".into(),
                    label: "Outlaw Paramilitaries & Disarm Ringleaders".into(),
                    description: "Enforce the state's monopoly on violence through federal police.".into(),
                    impact_hint: "Ideological Extremism -10, Rule of Law +7, Nationalists -10, Intelligentsia +10.".into(),
                },
                EventChoice {
                    id: "coopt_nationalists".into(),
                    label: "Fold Militias into Territorial Reserve & Adopt Tough Rhetoric".into(),
                    description: "Channel radical energy into pro-regime border guards.".into(),
                    impact_hint: "Troops +25k, Nationalists +12, Ideological Extremism +8, Sectarian Tension +10, Liberties -8.".into(),
                },
            ],
        },
        "foreign_propaganda_campaign" => ActiveEvent {
            id: id.into(),
            title: "Foreign Disinformation Networks Inflame Domestic Polarization".into(),
            category: LogCategory::Diplomacy,
            urgency: "HYBRID WARFARE".into(),
            briefing: "Counter-intelligence uncovered thousands of foreign-funded bot farms amplifying sectarian hatred and coup rumors across domestic social platforms.".into(),
            choices: vec![
                EventChoice {
                    id: "sovereign_firewall".into(),
                    label: "Impose National Internet Firewall & Media Licensing".into(),
                    description: "Block foreign platforms and tighten state information control.".into(),
                    impact_hint: "Press Freedom -16, Extremism -6, Security +10, Intelligentsia -14.".into(),
                },
                EventChoice {
                    id: "public_transparency_counter".into(),
                    label: "Expose Foreign Agents Publicly & Fund Media Literacy ($4B)".into(),
                    description: "Declassify intelligence evidence without censoring domestic press.".into(),
                    impact_hint: "Treasury -$4B, Legitimacy +7, Press Freedom +4, Stability +4.".into(),
                },
            ],
        },
        "pandemic_outbreak" => ActiveEvent {
            id: id.into(),
            title: "Novel Respiratory Outbreak Strains Urban Hospitals".into(),
            category: LogCategory::Crisis,
            urgency: "PUBLIC HEALTH".into(),
            briefing: "A virulent pathogen has surged in major transit corridors. Epidemiologists warn that unchecked transmission could overwhelm hospitals within weeks.".into(),
            choices: vec![
                EventChoice {
                    id: "strict_lockdown_aid".into(),
                    label: "Enforce Quarantine & Emergency Healthcare Surge ($16B)".into(),
                    description: "Contain transmission rapidly while paying affected workers.".into(),
                    impact_hint: "Treasury -$16B, Healthcare +6, GDP Growth -1.5% short-term, Approval +5.".into(),
                },
                EventChoice {
                    id: "keep_economy_open".into(),
                    label: "Keep Factories Open & Rely on Voluntary Precautions".into(),
                    description: "Prioritize GDP output and private commerce over lockdowns.".into(),
                    impact_hint: "Capitalists +10, Approval -10, Healthcare -6, Stability -6.".into(),
                },
            ],
        },
        // GOD MODE / GOLDEN MANDATE PLAYER-FAVORING INCIDENTS
        "god_sovereign_tech_singularity" => ActiveEvent {
            id: id.into(),
            title: "Breakthrough: Sovereign AI & Quantum Singularity Achieved!".into(),
            category: LogCategory::Economic,
            urgency: "GOLDEN ERA WINDFALL".into(),
            briefing: format!(
                "National laboratories in {} have achieved the world's first fault-tolerant quantum-AI supercluster, catapulting your country decades ahead of every global competitor.",
                state.nation_name
            ),
            choices: vec![
                EventChoice {
                    id: "singularity_civil".into(),
                    label: "Deploy Across Civil Economy & Autonomous Industry".into(),
                    description: "Supercharge productivity, precision medicine, and smart infrastructure nationwide.".into(),
                    impact_hint: "GDP +$250B, GDP Growth +3.5%, Tech/Edu/Infra -> 100%, Approval -> 100%.".into(),
                },
                EventChoice {
                    id: "singularity_defense".into(),
                    label: "Integrate into Supreme Defense & Orbital Command".into(),
                    description: "Give your armed forces unbreakable battlefield omniscience and autonomous drone swarms.".into(),
                    impact_hint: "Military Tech & Readiness -> 100%, Troops +150K, All Active Wars +30 War Score.".into(),
                },
                EventChoice {
                    id: "singularity_export".into(),
                    label: "License Proprietary Quantum Patents Worldwide".into(),
                    description: "Collect historic global royalties and lock foreign economies into your tech ecosystem.".into(),
                    impact_hint: "Treasury +$400B, All Independent Powers Relations +25, Trade Balance +$20B.".into(),
                },
            ],
        },
        "god_rival_regime_collapse" => {
            let target_id = state
                .pending_event_target
                .unwrap_or(ForeignPowerId::BorderlandRepublic);
            let p = state.foreign(target_id);
            ActiveEvent {
                id: id.into(),
                title: format!("Geopolitical Windfall: {} Fractures & Petitions {}!", p.name, state.nation_name),
                category: LogCategory::Diplomacy,
                urgency: "GOLDEN ERA OPPORTUNITY".into(),
                briefing: format!(
                    "Dazzled by {}'s unmatched prosperity and stability, regional governors and military commanders inside {} have overthrown anti-{} hardliners and dispatched envoys offering full union or protectorate status without a shot fired!",
                    state.nation_name, p.name, state.nation_name
                ),
                choices: vec![
                    EventChoice {
                        id: "rival_peaceful_annex".into(),
                        label: format!("Conduct Referendum & Peacefully Annex {}", p.name),
                        description: format!("Absorb {} directly into {} with universal popular acclaim.", p.name, state.nation_name),
                        impact_hint: format!("Peacefully annexes {} (+Pop, +GDP, +Treasury, +Troops, ZERO unrest or backlash!).", p.name),
                    },
                    EventChoice {
                        id: "rival_install_protectorate".into(),
                        label: format!("Establish {} as Loyal Sovereign Protectorate", p.short_name),
                        description: "Install a devoted client administration paying permanent tribute and military support.".into(),
                        impact_hint: format!("Converts {} into Client State (Trade/Defense Pact + Quarterly Tribute) + Treasury +$120B.", p.name),
                    },
                    EventChoice {
                        id: "rival_resource_monopoly".into(),
                        label: "Secure Exclusive Resource & Trade Monopoly".into(),
                        description: "Leave them nominally independent while claiming preferential mineral, energy, and banking rights.".into(),
                        impact_hint: "Treasury +$300B, GDP +$180B, Target Relations +35, Target Military -25.".into(),
                    },
                ],
            }
        }
        "god_energy_fusion_miracle" => ActiveEvent {
            id: id.into(),
            title: "Historic Milestone: Commercial Nuclear Fusion Grid Online!".into(),
            category: LogCategory::Economic,
            urgency: "GOLDEN ERA WINDFALL".into(),
            briefing: format!(
                "Engineers in {} have achieved sustained net-positive fusion power at commercial scale, slashing energy costs to near zero across every factory and household.",
                state.nation_name
            ),
            choices: vec![
                EventChoice {
                    id: "fusion_domestic_utopia".into(),
                    label: "Provide Near-Free Clean Power to Citizens & Industry".into(),
                    description: "Eliminate household utility bills and trigger an industrial renaissance.".into(),
                    impact_hint: "GDP +$300B, Approval -> 100%, Stability -> 100%, Inflation -> 0.5%, Gini -8.".into(),
                },
                EventChoice {
                    id: "fusion_energy_hegemon".into(),
                    label: "Export Fusion Reactors & Become Global Energy Hegemon".into(),
                    description: "Supply clean power to neighboring grids in return for massive sovereign inflows.".into(),
                    impact_hint: "Treasury +$500B, All Sanctions Lifted, Independent Powers Relations +20, Factions +20.".into(),
                },
            ],
        },
        "god_global_brain_drain_inflow" => ActiveEvent {
            id: id.into(),
            title: format!("Global Talent Surge: Top Scientists & Capital Flock to {}!", state.nation_name),
            category: LogCategory::Political,
            urgency: "GOLDEN ERA WINDFALL".into(),
            briefing: "Word of your nation's unmatched security, booming markets, and world-leading institutions has triggered a historic migration of Nobel laureates, engineers, and sovereign investment funds.".into(),
            choices: vec![
                EventChoice {
                    id: "talent_innovation_hubs".into(),
                    label: "Fast-Track Citizenship & Build Mega-Research Cities".into(),
                    description: "Absorb millions of skilled professionals into cutting-edge science and industrial hubs.".into(),
                    impact_hint: "Population +4.5M, GDP +$220B, Tech & Education -> 100%, Intelligentsia +25.".into(),
                },
                EventChoice {
                    id: "talent_reserve_currency".into(),
                    label: "Establish Global Reserve Currency & Financial Capital".into(),
                    description: "Channel trillions in global capital inflows into your sovereign reserve system.".into(),
                    impact_hint: "Treasury +$350B, GDP +$150B, Capitalists +25, Credit Rating AAA locked.".into(),
                },
            ],
        },
        "god_historic_diplomatic_triumph" => ActiveEvent {
            id: id.into(),
            title: format!("World Summit Acclaim: Global Powers Hail {}'s Golden Era", state.leader_name),
            category: LogCategory::Diplomacy,
            urgency: "GOLDEN ERA WINDFALL".into(),
            briefing: format!(
                "At the Global Governance Summit, foreign delegates praised {} {} for orchestrating an unprecedented economic and strategic miracle.",
                state.leader_title, state.leader_name
            ),
            choices: vec![
                EventChoice {
                    id: "summit_grand_accord".into(),
                    label: "Forge Grand Global Trade & Non-Aggression Accord".into(),
                    description: "Turn international admiration into universal trade pacts and diplomatic dominance.".into(),
                    impact_hint: "All non-war independent powers: Relations +35, Trade Pact ON, Sanctions Cleared; PC -> 100.".into(),
                },
                EventChoice {
                    id: "summit_assert_hegemony".into(),
                    label: "Project Unrivaled Strategic & Military Supremacy".into(),
                    description: "Rally global volunteers and demand immediate concessions from wartime foes.".into(),
                    impact_hint: "All Active Wars +40 War Score, Active Troops +200K, Treasury +$180B, Military/Nationalists +25.".into(),
                },
            ],
        },
        "god_supergiant_resource_discovery" => ActiveEvent {
            id: id.into(),
            title: "Mega-Deposit Uncovered: World's Largest Rare-Earth, Helium & Gold Belt!".into(),
            category: LogCategory::Economic,
            urgency: "GOLDEN ERA WINDFALL".into(),
            briefing: "Deep-crust seismic imaging has revealed an astonishingly rich mineral belt spanning your sovereign territory, guaranteeing resource self-sufficiency for centuries.".into(),
            choices: vec![
                EventChoice {
                    id: "mega_deposit_treasury".into(),
                    label: "Channel Windfall into Sovereign Wealth Mega-Fund".into(),
                    description: "Lock in permanent sovereign dividends and massive treasury liquidity.".into(),
                    impact_hint: "Treasury +$600B, GDP +$200B, Sovereign Wealth Fund ON, Nationalists & Labor +20.".into(),
                },
                EventChoice {
                    id: "mega_deposit_utopia".into(),
                    label: "Fund World-Leading Healthcare, Transit & Education Utopia".into(),
                    description: "Invest mineral revenues directly into raising living standards to perfection.".into(),
                    impact_hint: "Healthcare/Infra/Education -> 100%, Universal Healthcare ON, Approval -> 100%, GDP +$240B.".into(),
                },
            ],
        },
        "god_enemy_command_defection" => ActiveEvent {
            id: id.into(),
            title: "Battlefield Miracle: Enemy Frontline Divisions Defect En Masse!".into(),
            category: LogCategory::Military,
            urgency: "GOLDEN ERA MILITARY TRIUMPH".into(),
            briefing: format!(
                "Demoralized by {}'s overwhelming technological superiority and humane reputation, entire enemy armored corps have turned their flags, surrendered command bunkers, and broadcast calls for immediate capitulation!",
                state.nation_name
            ),
            choices: vec![
                EventChoice {
                    id: "defection_blitz_capital".into(),
                    label: "Exploit Collapse & Spearhead Straight to Enemy Capitals".into(),
                    description: "Accept mass surrenders and race unopposed into enemy strategic centers.".into(),
                    impact_hint: "All Active Wars +55 War Score! Troops +120K, Treasury +$150B captured war chest, War Exhaustion -> 0%.".into(),
                },
                EventChoice {
                    id: "defection_absorb_legions".into(),
                    label: "Integrate Defecting Brigades & Seize Advanced Arsenals".into(),
                    description: "Fold defecting units directly into your armed forces while securing front-line gains.".into(),
                    impact_hint: "All Active Wars +35 War Score, Active Troops +220K, Military Tech & Readiness -> 100%.".into(),
                },
            ],
        },
        "god_cultural_renaissance" => ActiveEvent {
            id: id.into(),
            title: format!("Golden Age Renaissance: Unprecedented Civic Unity in {}", state.nation_name),
            category: LogCategory::Political,
            urgency: "GOLDEN ERA WINDFALL".into(),
            briefing: "From bustling universities to thriving factories, citizens across every faction are celebrating a historic golden age of prosperity, peace, and shared national pride.".into(),
            choices: vec![
                EventChoice {
                    id: "renaissance_unity_charter".into(),
                    label: "Enshrine National Golden Age Unity Charter".into(),
                    description: "Cement permanent harmony across labor, business, clergy, military, and intelligentsia.".into(),
                    impact_hint: "All Factions Approval -> 98%, Approval/Stability/Legitimacy -> 100%, Extremism & Insurgency -> 0%.".into(),
                },
                EventChoice {
                    id: "renaissance_mega_cities".into(),
                    label: "Launch Futuristic Mega-Metropolis & Baby Boom Initiative".into(),
                    description: "Construct gleaming next-generation cities and expand national demographic strength.".into(),
                    impact_hint: "Population +6.0M, GDP +$280B, Infrastructure -> 100%, Treasury +$200B.".into(),
                },
            ],
        },
        "god_orbital_space_program" => ActiveEvent {
            id: id.into(),
            title: "Orbital Hegemony: Sovereign Moon Base & Asteroid Mining Launch!".into(),
            category: LogCategory::Economic,
            urgency: "GOLDEN ERA SPACE TRIUMPH".into(),
            briefing: format!(
                "{}'s aerospace command has completed a permanent lunar outpost and captured a platinum-rich near-Earth asteroid, opening an off-world frontier under your flag.",
                state.nation_name
            ),
            choices: vec![
                EventChoice {
                    id: "orbital_asteroid_mine".into(),
                    label: "Harvest Rare-Metal Asteroids for Sovereign Treasury".into(),
                    description: "Deliver refined platinum, palladium, and helium-3 directly into national vaults.".into(),
                    impact_hint: "Treasury +$800B, GDP +$350B, Tech -> 100%.".into(),
                },
                EventChoice {
                    id: "orbital_defense_grid".into(),
                    label: "Deploy Orbital Reconnaissance & Kinetic Defense Grid".into(),
                    description: "Establish total space-to-ground dominance over every theater of war.".into(),
                    impact_hint: "Military Tech & Readiness -> 100%, Troops +150K, All Active Wars +45 War Score.".into(),
                },
            ],
        },
        "god_biotech_longevity_cure" => ActiveEvent {
            id: id.into(),
            title: "Medical Miracle: Sovereign Biotech Institute Cures Cancer & Cellular Aging!".into(),
            category: LogCategory::Political,
            urgency: "GOLDEN ERA BREAKTHROUGH".into(),
            briefing: format!(
                "Medical researchers in {} have unveiled a universal gene-repair therapy that eradicates chronic illness and extends healthy human lifespans by decades.",
                state.nation_name
            ),
            choices: vec![
                EventChoice {
                    id: "biotech_free_citizens".into(),
                    label: "Distribute Longevity Therapy Free to Every Citizen".into(),
                    description: "Guarantee universal access nationwide, sparking joy and a historic demographic surge.".into(),
                    impact_hint: "Healthcare -> 100%, Universal Healthcare ON, Population +5.0M, Approval & Stability -> 100%.".into(),
                },
                EventChoice {
                    id: "biotech_global_licenses".into(),
                    label: "License Miracle Therapeutics Globally for Goodwill & Revenue".into(),
                    description: "Supply clinics worldwide while earning unmatched diplomatic admiration.".into(),
                    impact_hint: "Treasury +$450B, GDP +$200B, All Independent Powers Relations +30.".into(),
                },
            ],
        },
        "god_tax_haven_capital_tsunami" => ActiveEvent {
            id: id.into(),
            title: format!("Global Capital Tsunami: Fortune 500 Giants Relocate HQs to {}!", state.nation_name),
            category: LogCategory::Economic,
            urgency: "GOLDEN ERA BOOM".into(),
            briefing: format!(
                "Attracted by {}'s ultra-low taxes, AAA stability, and unmatched infrastructure, hundreds of global tech, aerospace, and finance titans are moving their global headquarters inside your borders.",
                state.nation_name
            ),
            choices: vec![
                EventChoice {
                    id: "capital_tsunami_open".into(),
                    label: "Fast-Track All Corporate Campuses & Zero-Tax Zones".into(),
                    description: "Unleash a tidal wave of private investment and high-paying jobs.".into(),
                    impact_hint: "GDP +$420B, GDP Growth +4.0%, Treasury +$300B, Unemployment -> 1.0%, Capitalists +25.".into(),
                },
                EventChoice {
                    id: "capital_tsunami_compact".into(),
                    label: "Enact Innovation & Worker Profit-Sharing Compact".into(),
                    description: "Pair corporate relocation with universal employee equity and transit upgrades.".into(),
                    impact_hint: "GDP +$320B, Gini -8, Labor & Capitalists +25, Education & Infra -> 100%.".into(),
                },
            ],
        },
        "god_rival_spy_ring_turned" => ActiveEvent {
            id: id.into(),
            title: "Intelligence Coup: Foreign Spy Networks Defect to Your Service!".into(),
            category: LogCategory::Military,
            urgency: "GOLDEN ERA INTEL WINDFALL".into(),
            briefing: "Counter-intelligence has flipped entire foreign espionage bureaus simultaneously, handing you encrypted war plans, cipher keys, and political blackmail across rival capitals.".into(),
            choices: vec![
                EventChoice {
                    id: "spy_sabotage_rivals".into(),
                    label: "Sabotage Rival Arsenals & Activate Double-Agent Networks".into(),
                    description: "Quietly cripple foreign defense readiness and feed live targeting data to your generals.".into(),
                    impact_hint: "All independent rivals Military Power -12, Spy Networks ON everywhere, Active Wars +35 War Score.".into(),
                },
                EventChoice {
                    id: "spy_diplomatic_leverage".into(),
                    label: "Leverage Intelligence Files for Historic Global Concessions".into(),
                    description: "Trade silent discretion for favorable treaties and sovereign capital transfers.".into(),
                    impact_hint: "Treasury +$220B, Political Capital -> 100, Legitimacy -> 100%, Independent Powers Relations +25.".into(),
                },
            ],
        },
        "god_hypersonic_armada_unveiled" => ActiveEvent {
            id: id.into(),
            title: "Defense Breakthrough: Autonomous Hypersonic Fleet Enters Service!".into(),
            category: LogCategory::Military,
            urgency: "GOLDEN ERA MILITARY APEX".into(),
            briefing: format!(
                "Defense plants in {} have rolled out mach-12 hypersonic strike wings and autonomous naval carriers that render legacy enemy air defenses completely obsolete.",
                state.nation_name
            ),
            choices: vec![
                EventChoice {
                    id: "hypersonic_deploy_front".into(),
                    label: "Deploy Full Armada to Strategic Commands & Frontlines".into(),
                    description: "Establish uncontested air, sea, and land supremacy overnight.".into(),
                    impact_hint: "Active Troops +250K, Military Tech & Readiness -> 100%, All Active Wars +50 War Score, Military +25.".into(),
                },
                EventChoice {
                    id: "hypersonic_export_allies".into(),
                    label: "Sell Export-Variant Defense Platforms Worldwide".into(),
                    description: "Turn your defense-industrial base into the world's premier arms supplier.".into(),
                    impact_hint: "Treasury +$550B, GDP +$200B, Independent Powers Relations +18.".into(),
                },
            ],
        },
        "god_global_olympics_expo" => ActiveEvent {
            id: id.into(),
            title: format!("World Showcase: {} Hosts Historic Global Expo & Games!", state.nation_name),
            category: LogCategory::Political,
            urgency: "GOLDEN ERA PRESTIGE".into(),
            briefing: "Billions of viewers worldwide watched in awe as your dazzling futuristic capital hosted the largest cultural expo and athletic games in human history.".into(),
            choices: vec![
                EventChoice {
                    id: "expo_spectacle".into(),
                    label: "Celebrate Global Cultural Triumph & National Pride".into(),
                    description: "Cement your reputation as the world's beacon of excellence and unity.".into(),
                    impact_hint: "Approval & Legitimacy -> 100%, All Factions +20, Infra -> 100%, Independent Powers Relations +20.".into(),
                },
                EventChoice {
                    id: "expo_trade_metropolis".into(),
                    label: "Convert Expo District into Permanent Global Trade Metropolis".into(),
                    description: "Transform the gleaming showcase into a permanent tourism and commerce engine.".into(),
                    impact_hint: "GDP +$280B, Treasury +$320B, Trade Balance +$25B, Population +3.0M.".into(),
                },
            ],
        },
        "god_climate_geoengineering_triumph" => ActiveEvent {
            id: id.into(),
            title: "Ecological Miracle: Sovereign Desalination & Weather Grid Greens Deserts!".into(),
            category: LogCategory::Economic,
            urgency: "GOLDEN ERA WINDFALL".into(),
            briefing: "Powered by cheap clean energy, massive solar-fusion desalination corridors have transformed arid basins into lush farmlands and verdant garden cities.".into(),
            choices: vec![
                EventChoice {
                    id: "geo_breadbasket".into(),
                    label: "Turn New Fertile Heartlands into Agro-Utopia & Eco-Cities".into(),
                    description: "Crush food prices to historic lows and open vast habitable provinces.".into(),
                    impact_hint: "Population +7.0M, GDP +$300B, Inflation -> 0.4%, Stability -> 100%.".into(),
                },
                EventChoice {
                    id: "geo_export_climate_tech".into(),
                    label: "Export Climate Engineering Tech & Global Food Surpluses".into(),
                    description: "End global droughts under your banner and lift remaining trade barriers.".into(),
                    impact_hint: "Treasury +$480B, All Sanctions Lifted, Independent Powers Relations +25.".into(),
                },
            ],
        },
        "god_cyber_dominance_windfall" => ActiveEvent {
            id: id.into(),
            title: "Digital Supremacy: Sovereign Quantum-Encrypted Network Adopted Worldwide!".into(),
            category: LogCategory::Economic,
            urgency: "GOLDEN ERA TECH MONOPOLY".into(),
            briefing: format!(
                "Unhackable quantum banking and telecom protocols invented in {} have become the default global standard for international commerce and finance.",
                state.nation_name
            ),
            choices: vec![
                EventChoice {
                    id: "cyber_clearinghouse".into(),
                    label: "Operate the Global Quantum Financial Clearinghouse".into(),
                    description: "Collect effortless clearing fees from every international transaction on Earth.".into(),
                    impact_hint: "Treasury +$650B, GDP +$250B, Trade Balance +$30B, Credit Rating AAA.".into(),
                },
                EventChoice {
                    id: "cyber_shield_paralyze".into(),
                    label: "Activate Sovereign Cyber-Shield & Paralyze Hostile Networks".into(),
                    description: "Neutralize insurgent cells and shut down enemy military communications.".into(),
                    impact_hint: "All Active Wars +45 War Score, Insurgency & Extremism -> 0%, Security & Intelligentsia +20.".into(),
                },
            ],
        },
        "god_border_voluntary_federation" => ActiveEvent {
            id: id.into(),
            title: format!("Historic Petition: Neighboring Border Regions Vote to Join {}!", state.nation_name),
            category: LogCategory::Diplomacy,
            urgency: "GOLDEN ERA EXPANSION".into(),
            briefing: format!(
                "Inspired by {}'s booming tax-free prosperity and world-class services, bustling autonomous border districts have voted 94% in favor of uniting with your nation.",
                state.nation_name
            ),
            choices: vec![
                EventChoice {
                    id: "federation_annex_provinces".into(),
                    label: "Welcome New Autonomous Provinces into the Union".into(),
                    description: "Expand your borders peacefully, integrating factories, ports, and millions of citizens.".into(),
                    impact_hint: "Population +8.5M, GDP +$360B, Troops +140K, Treasury +$180B, Nationalists +25.".into(),
                },
                EventChoice {
                    id: "federation_econ_corridor".into(),
                    label: "Establish Joint Sovereign Special Economic Corridor".into(),
                    description: "Capture massive cross-border trade output while earning regional applause.".into(),
                    impact_hint: "GDP +$450B, Treasury +$340B, All Independent Powers Relations +20.".into(),
                },
            ],
        },
        "god_sovereign_golden_constitution" => ActiveEvent {
            id: id.into(),
            title: "Historic Accord: All Domestic Factions Propose Eternal Golden Era Charter!".into(),
            category: LogCategory::Political,
            urgency: "GOLDEN ERA LEGACY".into(),
            briefing: format!(
                "Recognizing {} {}'s unmatched stewardship, leaders from Business, Labor, Military, Clergy, Security, Nationalists, and Intelligentsia have jointly drafted a landmark Golden Era Charter.",
                state.leader_title, state.leader_name
            ),
            choices: vec![
                EventChoice {
                    id: "golden_charter_prosperity".into(),
                    label: "Ratify Eternal Prosperity, Zero-Corruption & Innovation Charter".into(),
                    description: "Lock in total domestic consensus and world-leading institutional excellence.".into(),
                    impact_hint: "All Factions -> 100%, Rule of Law -> 100%, Corruption -> 1%, PC -> 100, GDP +$220B.".into(),
                },
                EventChoice {
                    id: "golden_charter_expansion".into(),
                    label: "Proclaim Supreme Sovereign Expansion & Grand Armada Doctrine".into(),
                    description: "Unite every domestic faction behind total geopolitical dominance.".into(),
                    impact_hint: "Active Troops +300K, All Active Wars +40 War Score, Military & Nationalists +30, Treasury +$250B.".into(),
                },
            ],
        },
        _ => ActiveEvent {
            id: "oligarch_media_buyout".into(),
            title: "Billionaire Syndicate Moves to Buy Independent Broadcasters".into(),
            category: LogCategory::Political,
            urgency: "MEDIA CONCENTRATION".into(),
            briefing: "A politically connected conglomerate is acquiring 70% of private television and news networks, offering favorable coverage of your administration in exchange for regulatory waivers.".into(),
            choices: vec![
                EventChoice {
                    id: "block_monopoly".into(),
                    label: "Block the Merger Under Anti-Trust & Press Independence Law".into(),
                    description: "Protect pluralistic journalism and limit oligarchic influence.".into(),
                    impact_hint: "Press Freedom +8, Rule of Law +6, Intelligentsia +10, Capitalists -10.".into(),
                },
                EventChoice {
                    id: "approve_friendly_media".into(),
                    label: "Approve Buyout in Exchange for Favorable Coverage".into(),
                    description: "Gain a powerful propaganda ally while alienating watchdogs.".into(),
                    impact_hint: "Approval +7, Capitalists +10, Press Freedom -14, Corruption +6.".into(),
                },
            ],
        },
    }
}

pub fn resolve_event_choice(state: &mut NationState, choice_id: &str) {
    let Some(event) = state.active_event.take() else {
        return;
    };

    let outcome_detail: String;

    match (event.id.as_str(), choice_id) {
        // POST-WAR VICTORY & CONQUEST SETTLEMENT
        ("post_war_victory", "annex_territory") => {
            if let Some(target_id) = state.pending_event_target.take() {
                outcome_detail = match crate::simulation::annex_foreign_power(state, target_id, false) {
                    Ok(m) => m,
                    Err(e) => e,
                };
            } else {
                outcome_detail = "Territorial annexation decree executed.".into();
            }
        }
        ("post_war_victory", "install_puppet") => {
            if let Some(target_id) = state.pending_event_target.take() {
                outcome_detail = match crate::simulation::install_puppet_regime(state, target_id) {
                    Ok(m) => m,
                    Err(e) => e,
                };
            } else {
                outcome_detail = "Client government installed.".into();
            }
        }
        ("post_war_victory", "impose_reparations") => {
            if let Some(target_id) = state.pending_event_target.take() {
                outcome_detail = match crate::simulation::impose_war_reparations(state, target_id) {
                    Ok(m) => m,
                    Err(e) => e,
                };
            } else {
                outcome_detail = "War reparations treaty signed.".into();
            }
        }

        // GOD MODE / GOLDEN MANDATE INCIDENTS
        ("god_sovereign_tech_singularity", "singularity_civil") => {
            state.gdp_bn += 250.0;
            state.gdp_growth = (state.gdp_growth + 3.5).min(18.0);
            state.tech_level = 100.0;
            state.education_level = 100.0;
            state.infrastructure_level = 100.0;
            state.approval = 100.0;
            outcome_detail = "Quantum-AI automation unleashed a civil economic boom (+$250B GDP, Tech/Edu/Infra 100%).".into();
        }
        ("god_sovereign_tech_singularity", "singularity_defense") => {
            state.military_tech = 100.0;
            state.military_readiness = 100.0;
            state.active_troops_k += 150.0;
            for p in &mut state.foreign_powers {
                if p.at_war && !p.annexed {
                    p.war_score = (p.war_score + 30.0).min(100.0);
                }
            }
            outcome_detail = "Orbital quantum command overwhelmed hostile defenses (+150K troops, +30 War Score in all wars).".into();
        }
        ("god_sovereign_tech_singularity", "singularity_export") => {
            state.treasury_bn += 400.0;
            state.trade_balance_bn = (state.trade_balance_bn + 20.0).min(95.0);
            for p in &mut state.foreign_powers {
                if !p.annexed && !p.at_war {
                    p.relation = (p.relation + 25.0).min(100.0);
                }
            }
            outcome_detail = "Global patent royalties poured $400B into the treasury while warming world ties.".into();
        }
        ("god_rival_regime_collapse", "rival_peaceful_annex") => {
            if let Some(target_id) = state.pending_event_target.take() {
                outcome_detail = match crate::simulation::annex_foreign_power(state, target_id, true) {
                    Ok(m) => m,
                    Err(e) => e,
                };
            } else {
                outcome_detail = "Historic union referendum completed peacefully.".into();
            }
        }
        ("god_rival_regime_collapse", "rival_install_protectorate") => {
            state.treasury_bn += 120.0;
            if let Some(target_id) = state.pending_event_target.take() {
                outcome_detail = match crate::simulation::install_puppet_regime(state, target_id) {
                    Ok(m) => m,
                    Err(e) => e,
                };
            } else {
                outcome_detail = "Protectorate treaty signed (+$120B treasury).".into();
            }
        }
        ("god_rival_regime_collapse", "rival_resource_monopoly") => {
            state.treasury_bn += 300.0;
            state.gdp_bn += 180.0;
            if let Some(target_id) = state.pending_event_target.take() {
                let p = state.foreign_mut(target_id);
                if !p.annexed {
                    p.relation = (p.relation + 35.0).min(100.0);
                    p.military_power = (p.military_power - 25.0).max(10.0);
                }
            }
            outcome_detail = "Exclusive trade and resource concessions delivered +$300B treasury and +$180B GDP.".into();
        }
        ("god_energy_fusion_miracle", "fusion_domestic_utopia") => {
            state.gdp_bn += 300.0;
            state.approval = 100.0;
            state.stability = 100.0;
            state.inflation = 0.5;
            state.gini = (state.gini - 8.0).max(18.0);
            outcome_detail = "Clean unlimited fusion power ignited a historic domestic boom (+$300B GDP, Inflation 0.5%).".into();
        }
        ("god_energy_fusion_miracle", "fusion_energy_hegemon") => {
            state.treasury_bn += 500.0;
            for p in &mut state.foreign_powers {
                if !p.annexed && !p.at_war {
                    p.sanctioning_us = false;
                    p.relation = (p.relation + 20.0).min(100.0);
                }
            }
            for f in &mut state.factions {
                f.approval = (f.approval + 20.0).min(100.0);
            }
            outcome_detail = "Global fusion reactor exports netted $500B and swept away all foreign sanctions.".into();
        }
        ("god_global_brain_drain_inflow", "talent_innovation_hubs") => {
            state.population_m += 4.5;
            state.gdp_bn += 220.0;
            state.tech_level = 100.0;
            state.education_level = 100.0;
            state.adjust_faction(FactionId::Intelligentsia, 25.0, 5.0);
            outcome_detail = "Millions of top researchers and engineers joined your nation (+4.5M Pop, +$220B GDP).".into();
        }
        ("god_global_brain_drain_inflow", "talent_reserve_currency") => {
            state.treasury_bn += 350.0;
            state.gdp_bn += 150.0;
            state.adjust_faction(FactionId::Capitalists, 25.0, 5.0);
            outcome_detail = "Your currency became the global gold standard (+$350B Treasury, +$150B GDP).".into();
        }
        ("god_historic_diplomatic_triumph", "summit_grand_accord") => {
            state.legitimacy = 100.0;
            state.political_capital = 100.0;
            for p in &mut state.foreign_powers {
                if !p.annexed && !p.at_war {
                    p.sanctioning_us = false;
                    p.relation = (p.relation + 35.0).min(100.0);
                    p.trade_pact = true;
                }
            }
            outcome_detail = "World leaders signed your Grand Global Trade Accord; all sanctions lifted and trade pacts activated!".into();
        }
        ("god_historic_diplomatic_triumph", "summit_assert_hegemony") => {
            state.treasury_bn += 180.0;
            state.active_troops_k += 200.0;
            for p in &mut state.foreign_powers {
                if p.at_war && !p.annexed {
                    p.war_score = (p.war_score + 40.0).min(100.0);
                }
            }
            state.adjust_faction(FactionId::Military, 25.0, 5.0);
            state.adjust_faction(FactionId::Nationalists, 25.0, 5.0);
            outcome_detail = "Unmatched global prestige surged volunteer recruitment (+200K troops) and broke enemy morale (+40 War Score).".into();
        }
        ("god_supergiant_resource_discovery", "mega_deposit_treasury") => {
            state.treasury_bn += 600.0;
            state.gdp_bn += 200.0;
            state.flags.sovereign_wealth_fund = true;
            state.adjust_faction(FactionId::Nationalists, 20.0, 4.0);
            state.adjust_faction(FactionId::LaborUnions, 20.0, 4.0);
            outcome_detail = "Mega-deposit revenues fortified the Sovereign Wealth Fund (+$600B Treasury, +$200B GDP).".into();
        }
        ("god_supergiant_resource_discovery", "mega_deposit_utopia") => {
            state.gdp_bn += 240.0;
            state.healthcare_level = 100.0;
            state.infrastructure_level = 100.0;
            state.education_level = 100.0;
            state.flags.universal_healthcare = true;
            state.approval = 100.0;
            outcome_detail = "Resource wealth transformed national healthcare, transit, and universities into global showcases.".into();
        }
        ("god_enemy_command_defection", "defection_blitz_capital") => {
            state.active_troops_k += 120.0;
            state.treasury_bn += 150.0;
            state.war_exhaustion = 0.0;
            for p in &mut state.foreign_powers {
                if p.at_war && !p.annexed {
                    p.war_score = (p.war_score + 55.0).min(100.0);
                }
            }
            outcome_detail = "Enemy lines dissolved overnight! Your spearheads raced into enemy capitals (+55 War Score, +$150B war chest).".into();
        }
        ("god_enemy_command_defection", "defection_absorb_legions") => {
            state.active_troops_k += 220.0;
            state.military_tech = 100.0;
            state.military_readiness = 100.0;
            state.war_exhaustion = 0.0;
            for p in &mut state.foreign_powers {
                if p.at_war && !p.annexed {
                    p.war_score = (p.war_score + 35.0).min(100.0);
                }
            }
            state.adjust_faction(FactionId::Military, 25.0, 4.0);
            outcome_detail = "Defecting divisions swore allegiance to your banner (+220K troops, +35 War Score).".into();
        }
        ("god_cultural_renaissance", "renaissance_unity_charter") => {
            state.approval = 100.0;
            state.stability = 100.0;
            state.legitimacy = 100.0;
            state.religious_extremism = 0.0;
            state.ideological_extremism = 0.0;
            state.sectarian_tension = 0.0;
            state.insurgency_strength = 0.0;
            state.corruption = 2.0;
            for f in &mut state.factions {
                f.approval = f.approval.max(98.0);
            }
            outcome_detail = "The Golden Age Unity Charter united every domestic bloc in historic harmony (All Factions 98%+).".into();
        }
        ("god_cultural_renaissance", "renaissance_mega_cities") => {
            state.population_m += 6.0;
            state.gdp_bn += 280.0;
            state.treasury_bn += 200.0;
            state.infrastructure_level = 100.0;
            outcome_detail = "Futuristic mega-cities fueled a demographic and economic surge (+6.0M Pop, +$280B GDP, +$200B Treasury).".into();
        }
        ("god_orbital_space_program", "orbital_asteroid_mine") => {
            state.treasury_bn += 800.0;
            state.gdp_bn += 350.0;
            state.tech_level = 100.0;
            outcome_detail = "Off-world platinum and helium-3 shipments flooded the treasury (+$800B Treasury, +$350B GDP).".into();
        }
        ("god_orbital_space_program", "orbital_defense_grid") => {
            state.military_tech = 100.0;
            state.military_readiness = 100.0;
            state.active_troops_k += 150.0;
            for p in &mut state.foreign_powers {
                if p.at_war && !p.annexed {
                    p.war_score = (p.war_score + 45.0).min(100.0);
                }
            }
            outcome_detail = "Orbital defense platforms established total battlefield supremacy (+45 War Score in all wars, +150K Troops).".into();
        }
        ("god_biotech_longevity_cure", "biotech_free_citizens") => {
            state.healthcare_level = 100.0;
            state.flags.universal_healthcare = true;
            state.population_m += 5.0;
            state.approval = 100.0;
            state.stability = 100.0;
            outcome_detail = "Universal free longevity therapies transformed national health (+5.0M Pop, Healthcare & Approval 100%).".into();
        }
        ("god_biotech_longevity_cure", "biotech_global_licenses") => {
            state.treasury_bn += 450.0;
            state.gdp_bn += 200.0;
            for p in &mut state.foreign_powers {
                if !p.annexed && !p.at_war {
                    p.relation = (p.relation + 30.0).min(100.0);
                }
            }
            outcome_detail = "Global medical licensing won international gratitude (+$450B Treasury, +$200B GDP, Relations +30).".into();
        }
        ("god_tax_haven_capital_tsunami", "capital_tsunami_open") => {
            state.gdp_bn += 420.0;
            state.gdp_growth = (state.gdp_growth + 4.0).min(20.0);
            state.treasury_bn += 300.0;
            state.unemployment = 1.0;
            state.adjust_faction(FactionId::Capitalists, 25.0, 5.0);
            outcome_detail = "Global Fortune 500 relocations unleashed a historic economic surge (+$420B GDP, +$300B Treasury).".into();
        }
        ("god_tax_haven_capital_tsunami", "capital_tsunami_compact") => {
            state.gdp_bn += 320.0;
            state.gini = (state.gini - 8.0).max(16.0);
            state.education_level = 100.0;
            state.infrastructure_level = 100.0;
            state.adjust_faction(FactionId::Capitalists, 25.0, 4.0);
            state.adjust_faction(FactionId::LaborUnions, 25.0, 4.0);
            outcome_detail = "Shared-prosperity corporate relocations lifted both industry and workers (+$320B GDP, Gini -8).".into();
        }
        ("god_rival_spy_ring_turned", "spy_sabotage_rivals") => {
            for p in &mut state.foreign_powers {
                if !p.annexed && !p.puppet_state && p.id != ForeignPowerId::GlobalMonetaryCouncil {
                    p.military_power = (p.military_power - 12.0).max(10.0);
                    p.spy_network = true;
                }
                if p.at_war && !p.annexed {
                    p.war_score = (p.war_score + 35.0).min(100.0);
                }
            }
            outcome_detail = "Turned double-agents crippled rival defense readiness (-12 Enemy Military, +35 War Score in active wars).".into();
        }
        ("god_rival_spy_ring_turned", "spy_diplomatic_leverage") => {
            state.treasury_bn += 220.0;
            state.political_capital = 100.0;
            state.legitimacy = 100.0;
            for p in &mut state.foreign_powers {
                if !p.annexed && !p.at_war {
                    p.relation = (p.relation + 25.0).min(100.0);
                }
            }
            outcome_detail = "Quiet diplomatic leverage yielded sweeping treaties and $220B in favorable settlements.".into();
        }
        ("god_hypersonic_armada_unveiled", "hypersonic_deploy_front") => {
            state.active_troops_k += 250.0;
            state.military_tech = 100.0;
            state.military_readiness = 100.0;
            for p in &mut state.foreign_powers {
                if p.at_war && !p.annexed {
                    p.war_score = (p.war_score + 50.0).min(100.0);
                }
            }
            state.adjust_faction(FactionId::Military, 25.0, 5.0);
            state.adjust_faction(FactionId::Nationalists, 25.0, 5.0);
            outcome_detail = "Autonomous hypersonic strike wings shattered enemy fronts (+50 War Score, +250K Troops).".into();
        }
        ("god_hypersonic_armada_unveiled", "hypersonic_export_allies") => {
            state.treasury_bn += 550.0;
            state.gdp_bn += 200.0;
            for p in &mut state.foreign_powers {
                if !p.annexed && !p.at_war {
                    p.relation = (p.relation + 18.0).min(100.0);
                }
            }
            outcome_detail = "Export defense contracts poured $550B into sovereign coffers (+$200B GDP).".into();
        }
        ("god_global_olympics_expo", "expo_spectacle") => {
            state.approval = 100.0;
            state.legitimacy = 100.0;
            state.infrastructure_level = 100.0;
            for f in &mut state.factions {
                f.approval = (f.approval + 20.0).min(100.0);
            }
            for p in &mut state.foreign_powers {
                if !p.annexed && !p.at_war {
                    p.relation = (p.relation + 20.0).min(100.0);
                }
            }
            outcome_detail = "The dazzling Global Expo lifted national prestige and domestic unity to all-time highs.".into();
        }
        ("god_global_olympics_expo", "expo_trade_metropolis") => {
            state.gdp_bn += 280.0;
            state.treasury_bn += 320.0;
            state.trade_balance_bn = (state.trade_balance_bn + 25.0).min(100.0);
            state.population_m += 3.0;
            outcome_detail = "The Expo Trade Metropolis became the premier global hub (+$280B GDP, +$320B Treasury, +3.0M Pop).".into();
        }
        ("god_climate_geoengineering_triumph", "geo_breadbasket") => {
            state.population_m += 7.0;
            state.gdp_bn += 300.0;
            state.inflation = 0.4;
            state.stability = 100.0;
            outcome_detail = "Greened desert provinces unleashed an agricultural and population boom (+7.0M Pop, +$300B GDP, Inflation 0.4%).".into();
        }
        ("god_climate_geoengineering_triumph", "geo_export_climate_tech") => {
            state.treasury_bn += 480.0;
            for p in &mut state.foreign_powers {
                if !p.annexed && !p.at_war {
                    p.sanctioning_us = false;
                    p.relation = (p.relation + 25.0).min(100.0);
                }
            }
            outcome_detail = "Exporting climate-restoration grids brought $480B in revenue and lifted all sanctions.".into();
        }
        ("god_cyber_dominance_windfall", "cyber_clearinghouse") => {
            state.treasury_bn += 650.0;
            state.gdp_bn += 250.0;
            state.trade_balance_bn = (state.trade_balance_bn + 30.0).min(100.0);
            outcome_detail = "Your Sovereign Quantum Clearinghouse captured global settlement inflows (+$650B Treasury, +$250B GDP).".into();
        }
        ("god_cyber_dominance_windfall", "cyber_shield_paralyze") => {
            state.insurgency_strength = 0.0;
            state.religious_extremism = 0.0;
            state.ideological_extremism = 0.0;
            for p in &mut state.foreign_powers {
                if p.at_war && !p.annexed {
                    p.war_score = (p.war_score + 45.0).min(100.0);
                }
            }
            state.adjust_faction(FactionId::SecurityApparatus, 20.0, 4.0);
            state.adjust_faction(FactionId::Intelligentsia, 20.0, 4.0);
            outcome_detail = "Sovereign Cyber-Shield erased domestic extremist networks and blinded enemy war commands (+45 War Score).".into();
        }
        ("god_border_voluntary_federation", "federation_annex_provinces") => {
            state.population_m += 8.5;
            state.gdp_bn += 360.0;
            state.active_troops_k += 140.0;
            state.treasury_bn += 180.0;
            state.adjust_faction(FactionId::Nationalists, 25.0, 6.0);
            outcome_detail = "Autonomous border provinces peacefully joined the union (+8.5M Pop, +$360B GDP, +140K Troops, +$180B Treasury).".into();
        }
        ("god_border_voluntary_federation", "federation_econ_corridor") => {
            state.gdp_bn += 450.0;
            state.treasury_bn += 340.0;
            for p in &mut state.foreign_powers {
                if !p.annexed && !p.at_war {
                    p.relation = (p.relation + 20.0).min(100.0);
                }
            }
            outcome_detail = "The Joint Special Economic Corridor ignited cross-border prosperity (+$450B GDP, +$340B Treasury).".into();
        }
        ("god_sovereign_golden_constitution", "golden_charter_prosperity") => {
            for f in &mut state.factions {
                f.approval = 100.0;
            }
            state.rule_of_law = 100.0;
            state.corruption = 1.0;
            state.political_capital = 100.0;
            state.gdp_bn += 220.0;
            outcome_detail = "All domestic blocs ratified the Golden Era Charter (All Factions 100%, Corruption 1%, +$220B GDP).".into();
        }
        ("god_sovereign_golden_constitution", "golden_charter_expansion") => {
            state.active_troops_k += 300.0;
            state.treasury_bn += 250.0;
            for p in &mut state.foreign_powers {
                if p.at_war && !p.annexed {
                    p.war_score = (p.war_score + 40.0).min(100.0);
                }
            }
            state.adjust_faction(FactionId::Military, 30.0, 6.0);
            state.adjust_faction(FactionId::Nationalists, 30.0, 6.0);
            outcome_detail = "The Grand Armada Doctrine mobilized nationwide backing (+300K Troops, +40 War Score, +$250B Treasury).".into();
        }

        // ELECTIONS
        ("national_election", "fair_election") => {
            let win_prob = if state.flags.ledger_rebalance {
                1.0
            } else {
                (state.approval + state.legitimacy * 0.25 - 12.0).clamp(5.0, 95.0) / 100.0
            };
            state.democracy_index = (state.democracy_index + 6.0).min(100.0);
            state.legitimacy = (state.legitimacy + 8.0).min(100.0);
            if state.flags.ledger_rebalance || state.next_rand() <= win_prob {
                state.quarters_until_election = 16;
                state.political_capital = (state.political_capital + 20.0).min(100.0);
                outcome_detail = "Voters returned your coalition to power in a celebrated free and fair election! (+20 Political Capital)".into();
            } else {
                state.game_over = Some(GameOverReason::ElectoralDefeat(format!(
                    "In the {} general election, opposition parties defeated {} {} cleanly at the ballot box. Honoring the constitution, you presided over a peaceful transfer of power.",
                    state.year, state.leader_title, state.leader_name
                )));
                outcome_detail = "Opposition coalition won a decisive parliamentary majority.".into();
            }
        }
        ("national_election", "stimulus_campaign") => {
            state.treasury_bn -= 15.0;
            state.inflation += 1.2;
            let win_prob = if state.flags.ledger_rebalance {
                1.0
            } else {
                (state.approval + state.legitimacy * 0.25 + 6.0).clamp(10.0, 96.0) / 100.0
            };
            if state.flags.ledger_rebalance || state.next_rand() <= win_prob {
                state.quarters_until_election = 16;
                state.political_capital = (state.political_capital + 14.0).min(100.0);
                outcome_detail = "Last-minute populist spending propelled your coalition to a narrow re-election victory.".into();
            } else {
                state.game_over = Some(GameOverReason::ElectoralDefeat(format!(
                    "Despite $15B in pre-election stimulus, voters rejected your administration in {}. You stepped down from office.",
                    state.year
                )));
                outcome_detail = "Voters remained unconvinced and voted your government out of office.".into();
            }
        }
        ("national_election", "rig_ballots") => {
            state.quarters_until_election = 16;
            state.democracy_index = (state.democracy_index - 24.0).max(5.0);
            state.legitimacy = (state.legitimacy - 20.0).max(5.0);
            state.stability = (state.stability - 14.0).max(5.0);
            state.ideological_extremism = (state.ideological_extremism + 10.0).min(100.0);
            state.adjust_faction(FactionId::Intelligentsia, -24.0, -4.0);
            state.adjust_faction(FactionId::SecurityApparatus, 10.0, 8.0);
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 20.0;
            outcome_detail = "Electoral commissions declared a fabricated landslide victory amid outraged protests and international condemnation.".into();
        }
        ("national_election", "cancel_election_emergency") => {
            state.regime = RegimeType::AbsoluteAutocracy;
            state.flags.martial_law = true;
            state.flags.term_limits_abolished = true;
            state.democracy_index = (state.democracy_index - 38.0).max(5.0);
            state.civil_liberties = (state.civil_liberties - 32.0).max(5.0);
            state.legitimacy = (state.legitimacy - 22.0).max(5.0);
            state.adjust_faction(FactionId::Intelligentsia, -30.0, -10.0);
            state.adjust_faction(FactionId::SecurityApparatus, 12.0, 12.0);
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 30.0;
            outcome_detail = "Tanks surrounded the legislative hall as elections were suspended indefinitely under a State of Emergency.".into();
        }

        // MILITARY COUP ATTEMPT
        ("military_coup_attempt", "rally_loyalists") => {
            let sec_clout = state.faction(FactionId::SecurityApparatus).clout;
            let mil_app = state.faction(FactionId::Military).approval;
            let success_prob = ((sec_clout * 0.55 + state.approval * 0.35 + mil_app * 0.30) / 100.0).clamp(0.22, 0.88);
            if state.flags.ledger_rebalance || state.next_rand() <= success_prob {
                state.stability = (state.stability - 10.0).max(5.0);
                state.military_readiness = (state.military_readiness - 18.0).max(15.0);
                state.adjust_faction(FactionId::Military, 12.0, -22.0);
                state.adjust_faction(FactionId::SecurityApparatus, 14.0, 10.0);
                state.coup_risk = (state.coup_risk - 32.0).max(5.0);
                outcome_detail = "Loyalist paratroopers and security forces crushed the rebel armored column! Plotters face court-martial.".into();
            } else {
                state.game_over = Some(GameOverReason::MilitaryCoup(format!(
                    "Loyalist defenses collapsed before dawn. Rebel tanks breached the palace gates, arresting {} {} and installing a Supreme Military Council.",
                    state.leader_title, state.leader_name
                )));
                outcome_detail = "Rebel generals overwhelmed loyalist units and seized control of the state.".into();
            }
        }
        ("military_coup_attempt", "call_citizens_streets") => {
            let success_prob = ((state.approval * 0.85 + state.legitimacy * 0.35) / 100.0).clamp(0.15, 0.90);
            if state.flags.ledger_rebalance || state.next_rand() <= success_prob {
                state.legitimacy = (state.legitimacy + 16.0).min(100.0);
                state.democracy_index = (state.democracy_index + 10.0).min(100.0);
                state.adjust_faction(FactionId::Military, 8.0, -24.0);
                state.coup_risk = (state.coup_risk - 35.0).max(4.0);
                outcome_detail = "Hundreds of thousands of citizens surrounded the rebel tanks, forcing conscripts to lay down arms in a historic victory for civilian rule!".into();
            } else {
                state.game_over = Some(GameOverReason::MilitaryCoup(format!(
                    "Too few citizens heeded the emergency broadcast. Rebel troops dispersed protesters and deposed {} {} in a swift military coup.",
                    state.leader_title, state.leader_name
                )));
                outcome_detail = "The streets stayed quiet as the military junta consolidated total power.".into();
            }
        }
        ("military_coup_attempt", "power_sharing_junta") => {
            state.regime = RegimeType::MilitaryDictatorship;
            state.democracy_index = (state.democracy_index - 35.0).max(6.0);
            state.budget.military = (state.budget.military + 20.0).min(100.0);
            state.adjust_faction(FactionId::Military, 28.0, 22.0);
            state.adjust_faction(FactionId::Intelligentsia, -24.0, -10.0);
            state.coup_risk = 18.0;
            outcome_detail = "You struck a backroom pact with the High Command, surviving in office at the head of a Military Junta.".into();
        }

        // COMMUNIST GENERAL STRIKE
        ("communist_general_strike", "concede_socialist_reforms") => {
            state.taxes.corporate_tax = (state.taxes.corporate_tax + 6.0).min(65.0);
            state.taxes.state_ownership = (state.taxes.state_ownership + 15.0).min(100.0);
            state.budget.welfare = (state.budget.welfare + 12.0).min(100.0);
            state.gini = (state.gini - 5.0).max(20.0);
            state.stability = (state.stability + 8.0).min(100.0);
            state.adjust_faction(FactionId::LaborUnions, 20.0, 10.0);
            state.adjust_faction(FactionId::Capitalists, -18.0, -8.0);
            outcome_detail = "Unions returned to work after securing sweeping welfare and nationalization concessions.".into();
        }
        ("communist_general_strike", "embrace_revolution") => {
            state.regime = RegimeType::OnePartyCommunist;
            state.economy = EconomicSystem::PlannedCommunism;
            state.taxes.state_ownership = 88.0;
            state.gini = (state.gini - 12.0).max(20.0);
            state.flags.opposition_banned = true;
            state.flags.five_year_plan = true;
            state.adjust_faction(FactionId::LaborUnions, 30.0, 25.0);
            state.adjust_faction(FactionId::Capitalists, -45.0, -35.0);
            state.foreign_mut(ForeignPowerId::PeoplesUnion).relation += 32.0;
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 35.0;
            outcome_detail = "You stepped onto the workers' podium and proclaimed a People's Socialist Republic!".into();
        }
        ("communist_general_strike", "send_riot_police") => {
            state.civil_liberties = (state.civil_liberties - 12.0).max(5.0);
            state.ideological_extremism = (state.ideological_extremism + 10.0).min(100.0);
            state.adjust_faction(FactionId::Capitalists, 12.0, 6.0);
            state.adjust_faction(FactionId::LaborUnions, -22.0, -8.0);
            outcome_detail = "Riot battalions cleared the factories by force, deepening class bitterness.".into();
        }

        // EXTREMIST INSURGENCY
        ("extremist_insurgency_attack", "declare_martial_crackdown") => {
            state.flags.martial_law = true;
            state.insurgency_strength = (state.insurgency_strength - 16.0).max(0.0);
            state.civil_liberties = (state.civil_liberties - 16.0).max(5.0);
            state.religious_extremism = (state.religious_extremism + 4.0).min(100.0);
            state.adjust_faction(FactionId::SecurityApparatus, 10.0, 8.0);
            state.adjust_faction(FactionId::Military, 8.0, 6.0);
            outcome_detail = "Martial law enabled sweeping arrests of insurgent cells, though heavy-handed raids alienated civilians.".into();
        }
        ("extremist_insurgency_attack", "targeted_intel_ops") => {
            state.treasury_bn -= 8.0;
            state.insurgency_strength = (state.insurgency_strength - 10.0).max(0.0);
            state.sectarian_tension = (state.sectarian_tension - 12.0).max(0.0);
            state.religious_extremism = (state.religious_extremism - 8.0).max(0.0);
            state.approval = (state.approval + 5.0).min(100.0);
            outcome_detail = "Surgical strikes eliminated cell leaders while interfaith unity rallies cooled sectarian fury.".into();
        }
        ("extremist_insurgency_attack", "appease_clerical_hardliners") => {
            state.clerical_power = (state.clerical_power + 14.0).min(100.0);
            state.insurgency_strength = (state.insurgency_strength - 6.0).max(0.0);
            state.civil_liberties = (state.civil_liberties - 12.0).max(5.0);
            state.adjust_faction(FactionId::Clergy, 15.0, 12.0);
            state.adjust_faction(FactionId::Intelligentsia, -18.0, -5.0);
            outcome_detail = "Clerical hardliners condemned the bombings after receiving sweeping moral policing powers.".into();
        }

        // DEBT CRISIS
        ("debt_currency_crisis", "imf_austerity_deal") => {
            state.treasury_bn += 50.0;
            state.national_debt_bn += 50.0;
            state.inflation = (state.inflation - 3.0).max(1.0);
            state.imf_austerity_turns = 6;
            state.approval = (state.approval - 12.0).max(0.0);
            state.adjust_faction(FactionId::LaborUnions, -14.0, -4.0);
            state.foreign_mut(ForeignPowerId::GlobalMonetaryCouncil).relation += 16.0;
            outcome_detail = "GMC funds stabilized the currency, triggering bitter protests over mandated austerity.".into();
        }
        ("debt_currency_crisis", "capital_controls_print") => {
            state.treasury_bn += 25.0;
            state.flags.capital_controls = true;
            state.inflation += 6.0;
            state.adjust_faction(FactionId::Capitalists, -22.0, -8.0);
            outcome_detail = "Capital controls halted bank runs, but money-printing sent inflation soaring.".into();
        }
        ("debt_currency_crisis", "selective_default") => {
            state.national_debt_bn *= 0.70;
            state.gdp_bn *= 0.96;
            state.foreign_mut(ForeignPowerId::GlobalMonetaryCouncil).relation -= 40.0;
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 25.0;
            state.adjust_faction(FactionId::Nationalists, 10.0, 5.0);
            state.adjust_faction(FactionId::Capitalists, -16.0, -6.0);
            outcome_detail = "Foreign creditors absorbed a forced debt haircut; global markets froze access to new credit.".into();
        }

        // PRO DEMOCRACY UPRISING
        ("pro_democracy_uprising", "open_democratic_reforms") => {
            state.regime = RegimeType::LiberalDemocracy;
            state.flags.press_censorship = false;
            state.flags.opposition_banned = false;
            state.flags.term_limits_abolished = false;
            state.democracy_index = (state.democracy_index + 32.0).min(92.0);
            state.civil_liberties = (state.civil_liberties + 26.0).min(92.0);
            state.approval = (state.approval + 14.0).min(100.0);
            state.quarters_until_election = 8;
            state.adjust_faction(FactionId::Intelligentsia, 22.0, 14.0);
            state.adjust_faction(FactionId::SecurityApparatus, -15.0, -12.0);
            outcome_detail = "Jubilant crowds celebrated as censorship fell and open elections were announced.".into();
        }
        ("pro_democracy_uprising", "tiananmen_crackdown") => {
            state.stability = (state.stability + 6.0).min(100.0);
            state.democracy_index = (state.democracy_index - 20.0).max(4.0);
            state.civil_liberties = (state.civil_liberties - 22.0).max(4.0);
            state.ideological_extremism = (state.ideological_extremism + 12.0).min(100.0);
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).sanctioning_us = true;
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 35.0;
            state.adjust_faction(FactionId::Intelligentsia, -30.0, -12.0);
            state.adjust_faction(FactionId::SecurityApparatus, 12.0, 10.0);
            outcome_detail = "Security forces cleared the square overnight; Western powers slapped immediate sanctions on your regime.".into();
        }
        ("pro_democracy_uprising", "staged_concessions") => {
            state.treasury_bn -= 10.0;
            state.approval = (state.approval + 6.0).min(100.0);
            state.corruption = (state.corruption - 4.0).max(5.0);
            state.adjust_faction(FactionId::Intelligentsia, 4.0, 0.0);
            outcome_detail = "Cabinet reshuffles and urban housing grants partially defused the street demonstrations.".into();
        }

        // CAPITAL FLIGHT
        ("capital_flight_panic", "freeze_oligarch_assets") => {
            state.treasury_bn += 16.0;
            state.flags.capital_controls = true;
            state.taxes.state_ownership = (state.taxes.state_ownership + 10.0).min(100.0);
            state.adjust_faction(FactionId::Capitalists, -18.0, -10.0);
            state.adjust_faction(FactionId::LaborUnions, 12.0, 5.0);
            outcome_detail = "Seized corporate accounts replenished the treasury as capital controls locked down transfers.".into();
        }
        ("capital_flight_panic", "cut_corp_taxes") => {
            state.taxes.corporate_tax = (state.taxes.corporate_tax - 6.0).max(5.0);
            state.gdp_growth += 1.2;
            state.adjust_faction(FactionId::Capitalists, 18.0, 8.0);
            state.adjust_faction(FactionId::LaborUnions, -10.0, -2.0);
            outcome_detail = "Tax amnesty restored investor confidence and reversed capital outflows.".into();
        }

        // NUCLEAR LEAK
        ("nuclear_satellite_leak", "defy_world_test") => {
            state.nuclear_progress += 45.0;
            if state.nuclear_progress >= 100.0 {
                if let Some(next) = state.nuclear_status.next() {
                    state.nuclear_status = next;
                    state.nuclear_progress = 0.0;
                }
            }
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).sanctioning_us = true;
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 30.0;
            state.foreign_mut(ForeignPowerId::GlobalMonetaryCouncil).sanctioning_us = true;
            state.adjust_faction(FactionId::Nationalists, 16.0, 8.0);
            state.adjust_faction(FactionId::Military, 14.0, 6.0);
            outcome_detail = "Defying global sanctions, engineers accelerated warhead assembly to thunderous nationalist applause.".into();
        }
        ("nuclear_satellite_leak", "negotiate_nuclear_deal") => {
            state.nuclear_status = NuclearStatus::CivilianResearch;
            state.nuclear_progress = 0.0;
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).sanctioning_us = false;
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation += 24.0;
            state.foreign_mut(ForeignPowerId::GlobalMonetaryCouncil).sanctioning_us = false;
            state.foreign_mut(ForeignPowerId::GlobalMonetaryCouncil).relation += 20.0;
            state.adjust_faction(FactionId::Nationalists, -14.0, -4.0);
            outcome_detail = "Inspectors verified peaceful nuclear limits, unlocking sanctions relief and trade.".into();
        }

        // BORDER FLASHPOINT
        ("border_flashpoint", "launch_full_war") => {
            let p = state.foreign_mut(ForeignPowerId::BorderlandRepublic);
            p.at_war = true;
            p.relation = -95.0;
            p.war_score = 5.0;
            state.adjust_faction(FactionId::Nationalists, 16.0, 8.0);
            state.adjust_faction(FactionId::Military, 12.0, 10.0);
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation -= 15.0;
            outcome_detail = "Armored divisions crossed the frontier! Full-scale war with Borderland Republic has begun.".into();
        }
        ("border_flashpoint", "limited_airstrikes") => {
            state.treasury_bn -= 5.0;
            state.foreign_mut(ForeignPowerId::BorderlandRepublic).relation -= 18.0;
            state.adjust_faction(FactionId::Nationalists, 6.0, 2.0);
            state.adjust_faction(FactionId::Military, 6.0, 2.0);
            outcome_detail = "Precision strikes neutralized enemy artillery positions without escalating to total war.".into();
        }
        ("border_flashpoint", "diplomatic_deescalation") => {
            state.foreign_mut(ForeignPowerId::BorderlandRepublic).relation += 14.0;
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation += 10.0;
            state.stability = (state.stability + 4.0).min(100.0);
            state.adjust_faction(FactionId::Nationalists, -12.0, -3.0);
            state.adjust_faction(FactionId::Military, -8.0, -3.0);
            outcome_detail = "International monitors established a buffer zone, averting bloodshed.".into();
        }

        // RELIGIOUS EVENTS
        ("clerical_fatwa_resistance", "relax_to_secular") => {
            state.religion_policy = ReligionPolicy::SecularPluralism;
            state.religious_extremism = (state.religious_extremism - 10.0).max(0.0);
            state.stability = (state.stability + 8.0).min(100.0);
            state.adjust_faction(FactionId::Clergy, 18.0, 6.0);
            outcome_detail = "Reopening worship spaces under secular pluralism defused religious hostility.".into();
        }
        ("clerical_fatwa_resistance", "arrest_clerics") => {
            state.clerical_power = (state.clerical_power - 12.0).max(5.0);
            state.religious_extremism = (state.religious_extremism + 14.0).min(100.0);
            state.insurgency_strength = (state.insurgency_strength + 8.0).min(100.0);
            state.civil_liberties = (state.civil_liberties - 10.0).max(5.0);
            outcome_detail = "Mass arrests drove religious opposition underground, fueling militant radicalization.".into();
        }
        ("blasphemy_law_protests", "curb_morality_police") => {
            state.religion_policy = ReligionPolicy::OfficialReligion;
            state.civil_liberties = (state.civil_liberties + 14.0).min(100.0);
            state.adjust_faction(FactionId::Intelligentsia, 16.0, 6.0);
            state.adjust_faction(FactionId::Clergy, -16.0, -8.0);
            outcome_detail = "Disbanding street morality squads calmed urban unrest at the cost of clerical anger.".into();
        }
        ("blasphemy_law_protests", "back_clerical_council") => {
            state.approval = (state.approval - 8.0).max(0.0);
            state.ideological_extremism = (state.ideological_extremism + 8.0).min(100.0);
            state.adjust_faction(FactionId::Clergy, 14.0, 8.0);
            state.adjust_faction(FactionId::Intelligentsia, -18.0, -6.0);
            outcome_detail = "Pro-clerical militias restored order on campuses amid deep generational resentment.".into();
        }

        // SUPPLY SHORTAGES
        ("supply_shortage_breadlines", "nep_market_reforms") => {
            state.economy = EconomicSystem::StateCapitalism;
            state.taxes.state_ownership = (state.taxes.state_ownership - 24.0).max(35.0);
            state.gdp_growth += 1.8;
            state.approval = (state.approval + 8.0).min(100.0);
            state.adjust_faction(FactionId::Capitalists, 14.0, 10.0);
            outcome_detail = "Allowing private cooperatives rapidly refilled urban markets under a State Capitalist model.".into();
        }
        ("supply_shortage_breadlines", "emergency_food_imports") => {
            state.treasury_bn -= 14.0;
            state.approval = (state.approval + 7.0).min(100.0);
            state.stability = (state.stability + 6.0).min(100.0);
            outcome_detail = "Emergency shipments of grain and essentials ended the breadlines.".into();
        }
        ("supply_shortage_breadlines", "crackdown_speculators") => {
            state.approval = (state.approval - 8.0).max(0.0);
            state.corruption = (state.corruption - 4.0).max(5.0);
            state.civil_liberties = (state.civil_liberties - 6.0).max(5.0);
            state.adjust_faction(FactionId::SecurityApparatus, 6.0, 4.0);
            outcome_detail = "Strict ration cards stabilized distribution but left citizens frustrated.".into();
        }

        // AMBIENT EVENTS
        ("cyberattack_grid", "cyber_retaliation") => {
            state.treasury_bn -= 9.0;
            state.tech_level = (state.tech_level + 5.0).min(100.0);
            state.adjust_faction(FactionId::SecurityApparatus, 10.0, 4.0);
            state.foreign_mut(ForeignPowerId::EurasianFederation).relation -= 12.0;
            outcome_detail = "Power was restored while our cyber command disrupted hostile foreign servers.".into();
        }
        ("cyberattack_grid", "defensive_patch") => {
            state.treasury_bn -= 12.0;
            state.infrastructure_level = (state.infrastructure_level + 6.0).min(100.0);
            state.tech_level = (state.tech_level + 7.0).min(100.0);
            state.stability = (state.stability + 5.0).min(100.0);
            outcome_detail = "Air-gapped grid upgrades hardened critical infrastructure against future attacks.".into();
        }
        ("whistleblower_corruption", "prosecute_transparently") => {
            state.corruption = (state.corruption - 10.0).max(4.0);
            state.rule_of_law = (state.rule_of_law + 8.0).min(100.0);
            state.approval = (state.approval + 5.0).min(100.0);
            state.adjust_faction(FactionId::Military, -6.0, -3.0);
            outcome_detail = "Indicting corrupt officials won public praise and cleaned up procurement.".into();
        }
        ("whistleblower_corruption", "suppress_press_leak") => {
            state.press_freedom = (state.press_freedom - 15.0).max(4.0);
            state.corruption = (state.corruption + 5.0).min(100.0);
            state.adjust_faction(FactionId::Intelligentsia, -14.0, -4.0);
            state.adjust_faction(FactionId::SecurityApparatus, 8.0, 4.0);
            outcome_detail = "Security raids silenced the press outlet, fueling whispered rumors of regime corruption.".into();
        }
        ("rare_earth_discovery", "state_mining_corp") => {
            state.treasury_bn -= 10.0;
            state.gdp_bn += 24.0;
            state.taxes.state_ownership = (state.taxes.state_ownership + 8.0).min(100.0);
            state.adjust_faction(FactionId::Nationalists, 10.0, 4.0);
            state.adjust_faction(FactionId::LaborUnions, 8.0, 3.0);
            outcome_detail = "The new state mining champion is projected to generate decades of strategic revenue.".into();
        }
        ("rare_earth_discovery", "auction_private_licenses") => {
            state.treasury_bn += 26.0;
            state.gdp_bn += 18.0;
            state.adjust_faction(FactionId::Capitalists, 14.0, 6.0);
            state.adjust_faction(FactionId::Nationalists, -8.0, -2.0);
            state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation += 10.0;
            outcome_detail = "Global consortia paid $26B upfront for highland mineral concessions.".into();
        }
        ("global_oil_shock", "subsidize_fuel") => {
            state.treasury_bn -= 14.0;
            state.inflation += 0.8;
            state.approval = (state.approval + 6.0).min(100.0);
            state.adjust_faction(FactionId::LaborUnions, 8.0, 2.0);
            outcome_detail = "Emergency fuel subsidies cushioned families from the global energy shock.".into();
        }
        ("global_oil_shock", "hike_rates_market") => {
            state.taxes.interest_rate = (state.taxes.interest_rate + 2.5).min(25.0);
            state.inflation += 1.5;
            state.approval = (state.approval - 7.0).max(0.0);
            state.adjust_faction(FactionId::Capitalists, 6.0, 2.0);
            outcome_detail = "Higher interest rates defended the currency while squeezing borrowers.".into();
        }
        ("far_right_militia_rally", "ban_disarm_militias") => {
            state.ideological_extremism = (state.ideological_extremism - 10.0).max(0.0);
            state.rule_of_law = (state.rule_of_law + 7.0).min(100.0);
            state.adjust_faction(FactionId::Nationalists, -10.0, -5.0);
            state.adjust_faction(FactionId::Intelligentsia, 10.0, 3.0);
            outcome_detail = "Federal police disarmed paramilitary camps and restored constitutional order.".into();
        }
        ("far_right_militia_rally", "coopt_nationalists") => {
            state.active_troops_k += 25.0;
            state.ideological_extremism = (state.ideological_extremism + 8.0).min(100.0);
            state.sectarian_tension = (state.sectarian_tension + 10.0).min(100.0);
            state.civil_liberties = (state.civil_liberties - 8.0).max(4.0);
            state.adjust_faction(FactionId::Nationalists, 12.0, 8.0);
            outcome_detail = "Paramilitaries were folded into border militias, alarming minorities and civil libertarians.".into();
        }
        ("foreign_propaganda_campaign", "sovereign_firewall") => {
            state.press_freedom = (state.press_freedom - 16.0).max(4.0);
            state.ideological_extremism = (state.ideological_extremism - 6.0).max(0.0);
            state.adjust_faction(FactionId::SecurityApparatus, 10.0, 6.0);
            state.adjust_faction(FactionId::Intelligentsia, -14.0, -4.0);
            outcome_detail = "A national internet firewall curbed foreign bot networks alongside domestic dissent.".into();
        }
        ("foreign_propaganda_campaign", "public_transparency_counter") => {
            state.treasury_bn -= 4.0;
            state.legitimacy = (state.legitimacy + 7.0).min(100.0);
            state.press_freedom = (state.press_freedom + 4.0).min(100.0);
            state.stability = (state.stability + 4.0).min(100.0);
            outcome_detail = "Declassified counter-intel briefings exposed foreign meddling and united public opinion.".into();
        }
        ("pandemic_outbreak", "strict_lockdown_aid") => {
            state.treasury_bn -= 16.0;
            state.healthcare_level = (state.healthcare_level + 6.0).min(100.0);
            state.gdp_growth -= 1.5;
            state.approval = (state.approval + 5.0).min(100.0);
            outcome_detail = "Swift quarantine and hospital funding contained the outbreak at high fiscal cost.".into();
        }
        ("pandemic_outbreak", "keep_economy_open") => {
            state.adjust_faction(FactionId::Capitalists, 10.0, 4.0);
            state.approval = (state.approval - 10.0).max(0.0);
            state.healthcare_level = (state.healthcare_level - 6.0).max(5.0);
            state.stability = (state.stability - 6.0).max(5.0);
            outcome_detail = "Commerce continued uninterrupted, but overwhelmed clinics sparked public outrage.".into();
        }
        ("oligarch_media_buyout", "block_monopoly") => {
            state.press_freedom = (state.press_freedom + 8.0).min(100.0);
            state.rule_of_law = (state.rule_of_law + 6.0).min(100.0);
            state.adjust_faction(FactionId::Intelligentsia, 10.0, 4.0);
            state.adjust_faction(FactionId::Capitalists, -10.0, -4.0);
            outcome_detail = "Anti-trust regulators blocked the media monopoly, earning praise from journalists.".into();
        }
        ("oligarch_media_buyout", "approve_friendly_media") => {
            state.approval = (state.approval + 7.0).min(100.0);
            state.press_freedom = (state.press_freedom - 14.0).max(4.0);
            state.corruption = (state.corruption + 6.0).min(100.0);
            state.adjust_faction(FactionId::Capitalists, 10.0, 6.0);
            outcome_detail = "Consolidating broadcasters delivered glowing evening news coverage of your administration.".into();
        }
        _ => {
            outcome_detail = "Directive executed.".into();
        }
    }

    state.last_fiscal = crate::simulation::calculate_fiscal(state);
    state.coup_risk = crate::simulation::calculate_coup_risk(state);
    state.credit_rating = crate::simulation::compute_credit_rating(state);
    state.log(event.category, format!("Resolved: {}", event.title), outcome_detail);

    // If another war has reached total victory (e.g. from simultaneous victories or battlefield defection), immediately queue its conference
    if state.game_over.is_none() && state.active_event.is_none() {
        if let Some(next_victory) = state
            .foreign_powers
            .iter()
            .find(|p| p.at_war && !p.annexed && p.war_score >= 95.0)
            .map(|p| p.id)
        {
            state.pending_event_target = Some(next_victory);
            state.active_event = Some(build_post_war_victory_event(state, next_victory));
        }
    }
}
