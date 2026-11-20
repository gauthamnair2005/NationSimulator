use crate::models::*;

pub fn maybe_trigger_event(state: &mut NationState) {
    if state.active_event.is_some() || state.game_over.is_some() {
        return;
    }

    // 1. Election Cycle in Democratic / Semi-Democratic regimes
    if state.regime.is_democratic() && state.quarters_until_election == 0 && !state.flags.term_limits_abolished {
        state.active_event = Some(build_election_event(state));
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

    if state.foreign_powers.iter().any(|p| p.id == ForeignPowerId::BorderlandRepublic && p.relation < -25.0 && !p.at_war)
        && cooldown_ready(state, "border_flashpoint")
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

fn cooldown_ready(state: &NationState, id: &str) -> bool {
    state.event_cooldowns.get(id).copied().unwrap_or(0) == 0
}

fn build_election_event(state: &NationState) -> ActiveEvent {
    let win_prob = (state.approval + state.legitimacy * 0.25 - 12.0).clamp(5.0, 95.0);
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
        // ELECTIONS
        ("national_election", "fair_election") => {
            let win_prob = (state.approval + state.legitimacy * 0.25 - 12.0).clamp(5.0, 95.0) / 100.0;
            state.democracy_index = (state.democracy_index + 6.0).min(100.0);
            state.legitimacy = (state.legitimacy + 8.0).min(100.0);
            if state.next_rand() <= win_prob {
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
            let win_prob = (state.approval + state.legitimacy * 0.25 + 6.0).clamp(10.0, 96.0) / 100.0;
            if state.next_rand() <= win_prob {
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
            if state.next_rand() <= success_prob {
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
            if state.next_rand() <= success_prob {
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
}
