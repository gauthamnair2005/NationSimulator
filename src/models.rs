use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegimeType {
    LiberalDemocracy,
    SocialDemocracy,
    OnePartyCommunist,
    MilitaryDictatorship,
    AbsoluteAutocracy,
    TheocraticRepublic,
    OligarchicRepublic,
}

impl RegimeType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::LiberalDemocracy => "Liberal Democracy",
            Self::SocialDemocracy => "Social Democracy",
            Self::OnePartyCommunist => "One-Party Communist State",
            Self::MilitaryDictatorship => "Military Dictatorship",
            Self::AbsoluteAutocracy => "Absolute Autocracy",
            Self::TheocraticRepublic => "Theocratic Republic",
            Self::OligarchicRepublic => "Oligarchic Republic",
        }
    }

    pub fn short_badge(&self) -> &'static str {
        match self {
            Self::LiberalDemocracy => "DEMOCRACY",
            Self::SocialDemocracy => "SOC-DEM",
            Self::OnePartyCommunist => "COMMUNIST",
            Self::MilitaryDictatorship => "MIL-JUNTA",
            Self::AbsoluteAutocracy => "AUTOCRACY",
            Self::TheocraticRepublic => "THEOCRACY",
            Self::OligarchicRepublic => "OLIGARCHY",
        }
    }

    pub fn is_democratic(&self) -> bool {
        matches!(self, Self::LiberalDemocracy | Self::SocialDemocracy)
    }

    pub fn is_authoritarian(&self) -> bool {
        matches!(
            self,
            Self::OnePartyCommunist
                | Self::MilitaryDictatorship
                | Self::AbsoluteAutocracy
                | Self::TheocraticRepublic
        )
    }

    pub fn all() -> &'static [RegimeType] {
        &[
            Self::LiberalDemocracy,
            Self::SocialDemocracy,
            Self::OnePartyCommunist,
            Self::MilitaryDictatorship,
            Self::AbsoluteAutocracy,
            Self::TheocraticRepublic,
            Self::OligarchicRepublic,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EconomicSystem {
    FreeMarketCapitalism,
    MixedEconomy,
    StateCapitalism,
    PlannedCommunism,
    WarEconomy,
}

impl EconomicSystem {
    pub fn label(&self) -> &'static str {
        match self {
            Self::FreeMarketCapitalism => "Free-Market Capitalism",
            Self::MixedEconomy => "Mixed Market Economy",
            Self::StateCapitalism => "State Capitalism",
            Self::PlannedCommunism => "Planned / Command Economy",
            Self::WarEconomy => "Total War Economy",
        }
    }

    pub fn short_badge(&self) -> &'static str {
        match self {
            Self::FreeMarketCapitalism => "CAPITALISM",
            Self::MixedEconomy => "MIXED MKT",
            Self::StateCapitalism => "STATE CAP",
            Self::PlannedCommunism => "PLANNED",
            Self::WarEconomy => "WAR ECON",
        }
    }

    pub fn all() -> &'static [EconomicSystem] {
        &[
            Self::FreeMarketCapitalism,
            Self::MixedEconomy,
            Self::StateCapitalism,
            Self::PlannedCommunism,
            Self::WarEconomy,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReligionPolicy {
    StateAtheism,
    SecularPluralism,
    OfficialReligion,
    TheocraticLaw,
}

impl ReligionPolicy {
    pub fn label(&self) -> &'static str {
        match self {
            Self::StateAtheism => "State Atheism",
            Self::SecularPluralism => "Secular Pluralism",
            Self::OfficialReligion => "Official State Religion",
            Self::TheocraticLaw => "Supreme Theocratic Law",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::StateAtheism => "Religious institutions restricted; science & state ideology prioritized. Angers Clergy & Traditionalists.",
            Self::SecularPluralism => "Separation of church and state with freedom of worship. Boosts liberties & moderates sectarianism.",
            Self::OfficialReligion => "State endorses majority faith with clerical advisory roles. Boosts stability among traditionalists.",
            Self::TheocraticLaw => "Clerical council supersedes civil courts. Empowers Clergy heavily; alienates secular youth and Western powers.",
        }
    }

    pub fn all() -> &'static [ReligionPolicy] {
        &[
            Self::StateAtheism,
            Self::SecularPluralism,
            Self::OfficialReligion,
            Self::TheocraticLaw,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefensePosture {
    Pacifist,
    Defensive,
    PowerProjection,
    FullMobilization,
}

impl DefensePosture {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Pacifist => "Pacifist / Minimalist",
            Self::Defensive => "Territorial Defense",
            Self::PowerProjection => "Power Projection",
            Self::FullMobilization => "Full War Mobilization",
        }
    }

    pub fn upkeep_multiplier(&self) -> f64 {
        match self {
            Self::Pacifist => 0.65,
            Self::Defensive => 1.0,
            Self::PowerProjection => 1.35,
            Self::FullMobilization => 1.85,
        }
    }

    pub fn all() -> &'static [DefensePosture] {
        &[
            Self::Pacifist,
            Self::Defensive,
            Self::PowerProjection,
            Self::FullMobilization,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NuclearStatus {
    None,
    CivilianResearch,
    SecretEnrichment,
    TacticalArsenal,
    StrategicTriad,
}

impl NuclearStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::None => "Non-Nuclear State",
            Self::CivilianResearch => "Civilian Nuclear Energy",
            Self::SecretEnrichment => "Clandestine Enrichment",
            Self::TacticalArsenal => "Tactical Nuclear Deterrent",
            Self::StrategicTriad => "Strategic ICBM Triad",
        }
    }

    pub fn stage(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::CivilianResearch => 1,
            Self::SecretEnrichment => 2,
            Self::TacticalArsenal => 3,
            Self::StrategicTriad => 4,
        }
    }

    pub fn next(&self) -> Option<NuclearStatus> {
        match self {
            Self::None => Some(Self::CivilianResearch),
            Self::CivilianResearch => Some(Self::SecretEnrichment),
            Self::SecretEnrichment => Some(Self::TacticalArsenal),
            Self::TacticalArsenal => Some(Self::StrategicTriad),
            Self::StrategicTriad => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionId {
    Capitalists,
    LaborUnions,
    Military,
    Clergy,
    Intelligentsia,
    SecurityApparatus,
    Nationalists,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub id: FactionId,
    pub name: String,
    pub approval: f64, // 0..100
    pub clout: f64,    // 0..100
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ForeignPowerId {
    AtlanticRepublic,
    PeoplesUnion,
    EurasianFederation,
    CrescentCoalition,
    BorderlandRepublic,
    GlobalMonetaryCouncil,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignPower {
    pub id: ForeignPowerId,
    pub name: String,
    pub short_name: String,
    pub ideology: String,
    pub military_power: f64, // 0..100
    pub economic_power: f64, // 0..100
    pub relation: f64,       // -100..100
    pub trade_pact: bool,
    pub defense_alliance: bool,
    pub sanctioning_us: bool,
    pub we_sanction_them: bool,
    pub at_war: bool,
    pub war_score: f64, // -100..100
    pub proxy_funding: bool,
    pub spy_network: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxPolicy {
    pub income_tax: f64,      // 5..70%
    pub corporate_tax: f64,   // 5..65%
    pub tariffs: f64,         // 0..60%
    pub wealth_tax: f64,      // 0..30%
    pub state_ownership: f64, // 0..100% of industry nationalized
    pub interest_rate: f64,   // 0.5..25.0%
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetAllocation {
    pub healthcare: f64,       // 0..100 intensity
    pub education: f64,        // 0..100
    pub infrastructure: f64,   // 0..100
    pub welfare: f64,          // 0..100
    pub military: f64,         // 0..100
    pub security_intel: f64,   // 0..100
    pub research: f64,         // 0..100
    pub religion_culture: f64, // 0..100
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FiscalSummary {
    pub income_rev: f64,
    pub corp_rev: f64,
    pub tariff_rev: f64,
    pub wealth_rev: f64,
    pub soe_rev: f64,
    pub total_revenue: f64,
    pub program_spending: f64,
    pub debt_interest: f64,
    pub total_spending: f64,
    pub net_balance: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyFlags {
    pub martial_law: bool,
    pub press_censorship: bool,
    pub secret_police: bool,
    pub opposition_banned: bool,
    pub term_limits_abolished: bool,
    pub conscription: bool,
    pub mass_surveillance: bool,
    pub deradicalization_program: bool,
    pub religious_courts: bool,
    pub capital_controls: bool,
    pub five_year_plan: bool,
    pub universal_healthcare: bool,
    pub sovereign_wealth_fund: bool,
    pub independent_judiciary: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogCategory {
    Political,
    Economic,
    Military,
    Religious,
    Crisis,
    Diplomacy,
}

impl LogCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Political => "POLITICAL",
            Self::Economic => "ECONOMIC",
            Self::Military => "MILITARY",
            Self::Religious => "RELIGIOUS",
            Self::Crisis => "CRISIS",
            Self::Diplomacy => "DIPLOMACY",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub turn: u32,
    pub date_label: String,
    pub category: LogCategory,
    pub headline: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryPoint {
    pub turn: u32,
    pub year: u32,
    pub quarter: u8,
    pub gdp_bn: f64,
    pub gdp_growth: f64,
    pub inflation: f64,
    pub unemployment: f64,
    pub debt_to_gdp: f64,
    pub approval: f64,
    pub stability: f64,
    pub coup_risk: f64,
    pub extremism: f64,
    pub treasury_bn: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventChoice {
    pub id: String,
    pub label: String,
    pub description: String,
    pub impact_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveEvent {
    pub id: String,
    pub title: String,
    pub category: LogCategory,
    pub urgency: String,
    pub briefing: String,
    pub choices: Vec<EventChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameOverReason {
    MilitaryCoup(String),
    PopularRevolution(String),
    ExtremistTakeover(String),
    ForeignConquest(String),
    SovereignCollapse(String),
    ElectoralDefeat(String),
}

impl GameOverReason {
    pub fn title(&self) -> &'static str {
        match self {
            Self::MilitaryCoup(_) => "OVERTHROWN BY MILITARY COUP",
            Self::PopularRevolution(_) => "TOPPLLED BY POPULAR REVOLUTION",
            Self::ExtremistTakeover(_) => "STATE FALLS TO EXTREMIST INSURGENCY",
            Self::ForeignConquest(_) => "CAPITULATION TO FOREIGN FORCES",
            Self::SovereignCollapse(_) => "TOTAL ECONOMIC & STATE COLLAPSE",
            Self::ElectoralDefeat(_) => "VOTED OUT OF OFFICE",
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::MilitaryCoup(m)
            | Self::PopularRevolution(m)
            | Self::ExtremistTakeover(m)
            | Self::ForeignConquest(m)
            | Self::SovereignCollapse(m)
            | Self::ElectoralDefeat(m) => m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioPreset {
    AureliaDemocracy,
    KrasnoviaCommunist,
    VeridiaCapitalist,
    SolariaDictatorship,
    AlNoorTheocracy,
}

impl ScenarioPreset {
    pub fn all() -> &'static [ScenarioPreset] {
        &[
            Self::AureliaDemocracy,
            Self::KrasnoviaCommunist,
            Self::VeridiaCapitalist,
            Self::SolariaDictatorship,
            Self::AlNoorTheocracy,
        ]
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::AureliaDemocracy => "Republic of Aurelia",
            Self::KrasnoviaCommunist => "People's Republic of Krasnovia",
            Self::VeridiaCapitalist => "Federation of Veridia",
            Self::SolariaDictatorship => "Military State of Solaria",
            Self::AlNoorTheocracy => "Sacred Republic of Al-Noor",
        }
    }

    pub fn subtitle(&self) -> &'static str {
        match self {
            Self::AureliaDemocracy => "Fragile Young Democracy • Mixed Economy • Rising Polarization",
            Self::KrasnoviaCommunist => "Communist One-Party State • Planned Economy • Western Sanctions",
            Self::VeridiaCapitalist => "Free-Market Capitalist Federation • High Inequality • Labor Unrest",
            Self::SolariaDictatorship => "Military Junta • High Coup Risk • Heavy Security Apparatus",
            Self::AlNoorTheocracy => "Theocratic Republic • Clerical Rule • Sectarian & Extremist Pressure",
        }
    }

    pub fn summary(&self) -> &'static str {
        match self {
            Self::AureliaDemocracy => "Navigate election cycles, moderate debt, and ideological polarization while resisting foreign interference and domestic extremists.",
            Self::KrasnoviaCommunist => "Command a collectivized industrial state with strong armed forces and loyal labor unions, but manage stagnation, shortages, and geopolitical isolation.",
            Self::VeridiaCapitalist => "Lead a booming laissez-faire powerhouse dominated by corporate oligarchs while preventing inequality from sparking a socialist revolution.",
            Self::SolariaDictatorship => "Rule by decree atop a restless general staff. Keep the military paid, crush dissent, or attempt a perilous transition to democracy.",
            Self::AlNoorTheocracy => "Balance clerical authority, resource revenues, religious law, and modernizing urban youth while containing militant fundamentalist splinters.",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationState {
    pub nation_name: String,
    pub leader_name: String,
    pub leader_title: String,
    pub year: u32,
    pub quarter: u8, // 1..=4
    pub turn: u32,
    pub political_capital: f64, // 0..100

    pub regime: RegimeType,
    pub economy: EconomicSystem,
    pub religion_policy: ReligionPolicy,
    pub defense_posture: DefensePosture,
    pub nuclear_status: NuclearStatus,
    pub nuclear_progress: f64, // 0..100 toward next stage

    // Macroeconomic Indicators
    pub population_m: f64,
    pub gdp_bn: f64,
    pub gdp_growth: f64, // annualized %
    pub inflation: f64,  // %
    pub unemployment: f64,
    pub treasury_bn: f64,
    pub national_debt_bn: f64,
    pub gini: f64, // 20..70
    pub trade_balance_bn: f64,
    pub credit_rating: String,
    pub imf_austerity_turns: u32,

    // Governance & Social Indicators (0..100)
    pub approval: f64,
    pub stability: f64,
    pub legitimacy: f64,
    pub democracy_index: f64,
    pub civil_liberties: f64,
    pub press_freedom: f64,
    pub corruption: f64,
    pub rule_of_law: f64,
    pub education_level: f64,
    pub healthcare_level: f64,
    pub infrastructure_level: f64,
    pub tech_level: f64,

    // Religion & Extremism (0..100)
    pub religiosity: f64,
    pub clerical_power: f64,
    pub religious_extremism: f64,
    pub ideological_extremism: f64,
    pub sectarian_tension: f64,
    pub insurgency_strength: f64,

    // Military & Security
    pub active_troops_k: f64,
    pub military_readiness: f64, // 0..100
    pub military_tech: f64,      // 0..100
    pub war_exhaustion: f64,     // 0..100
    pub coup_risk: f64,          // 0..100

    pub quarters_until_election: u32,

    pub taxes: TaxPolicy,
    pub budget: BudgetAllocation,
    pub flags: PolicyFlags,
    pub factions: Vec<Faction>,
    pub foreign_powers: Vec<ForeignPower>,

    pub enacted_decisions: Vec<String>,
    pub decision_cooldowns: HashMap<String, u32>,
    pub event_cooldowns: HashMap<String, u32>,

    pub history: Vec<HistoryPoint>,
    pub logs: Vec<LogEntry>,
    pub active_event: Option<ActiveEvent>,
    pub game_over: Option<GameOverReason>,
    pub last_fiscal: FiscalSummary,
    pub rng_state: u64,
}

impl NationState {
    pub fn date_label(&self) -> String {
        format!("Q{} {}", self.quarter, self.year)
    }

    pub fn debt_to_gdp(&self) -> f64 {
        if self.gdp_bn <= 0.1 {
            999.0
        } else {
            (self.national_debt_bn / self.gdp_bn) * 100.0
        }
    }

    pub fn total_extremism(&self) -> f64 {
        (self.religious_extremism * 0.55 + self.ideological_extremism * 0.45).clamp(0.0, 100.0)
    }

    pub fn active_wars_count(&self) -> usize {
        self.foreign_powers.iter().filter(|f| f.at_war).count()
    }

    pub fn faction(&self, id: FactionId) -> &Faction {
        self.factions
            .iter()
            .find(|f| f.id == id)
            .expect("faction must exist")
    }

    pub fn faction_mut(&mut self, id: FactionId) -> &mut Faction {
        self.factions
            .iter_mut()
            .find(|f| f.id == id)
            .expect("faction must exist")
    }

    pub fn adjust_faction(&mut self, id: FactionId, approval_delta: f64, clout_delta: f64) {
        let f = self.faction_mut(id);
        f.approval = (f.approval + approval_delta).clamp(0.0, 100.0);
        f.clout = (f.clout + clout_delta).clamp(5.0, 100.0);
    }

    pub fn foreign_mut(&mut self, id: ForeignPowerId) -> &mut ForeignPower {
        self.foreign_powers
            .iter_mut()
            .find(|p| p.id == id)
            .expect("foreign power must exist")
    }

    pub fn log(&mut self, category: LogCategory, headline: impl Into<String>, detail: impl Into<String>) {
        let entry = LogEntry {
            turn: self.turn,
            date_label: self.date_label(),
            category,
            headline: headline.into(),
            detail: detail.into(),
        };
        self.logs.insert(0, entry);
        if self.logs.len() > 250 {
            self.logs.truncate(250);
        }
    }

    pub fn next_rand(&mut self) -> f64 {
        // Deterministic LCG/xorshift64* for save-reproducible probabilities
        let mut x = self.rng_state;
        if x == 0 {
            x = 0x9E3779B97F4A7C15;
        }
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.rng_state = x;
        let v = x.wrapping_mul(0x2545F4914F6CDD1D);
        ((v >> 11) as f64) / ((1u64 << 53) as f64)
    }

    pub fn record_history(&mut self) {
        let pt = HistoryPoint {
            turn: self.turn,
            year: self.year,
            quarter: self.quarter,
            gdp_bn: self.gdp_bn,
            gdp_growth: self.gdp_growth,
            inflation: self.inflation,
            unemployment: self.unemployment,
            debt_to_gdp: self.debt_to_gdp(),
            approval: self.approval,
            stability: self.stability,
            coup_risk: self.coup_risk,
            extremism: self.total_extremism(),
            treasury_bn: self.treasury_bn,
        };
        self.history.push(pt);
        if self.history.len() > 160 {
            self.history.remove(0);
        }
    }

    pub fn from_preset(preset: ScenarioPreset) -> Self {
        let mut state = Self::base_template();
        match preset {
            ScenarioPreset::AureliaDemocracy => {
                state.nation_name = "Republic of Aurelia".into();
                state.leader_name = "Elena Vance".into();
                state.leader_title = "President".into();
                state.regime = RegimeType::LiberalDemocracy;
                state.economy = EconomicSystem::MixedEconomy;
                state.religion_policy = ReligionPolicy::SecularPluralism;
                state.gdp_bn = 420.0;
                state.population_m = 34.5;
                state.treasury_bn = 32.0;
                state.national_debt_bn = 245.0;
                state.approval = 54.0;
                state.stability = 62.0;
                state.legitimacy = 72.0;
                state.democracy_index = 78.0;
                state.civil_liberties = 76.0;
                state.press_freedom = 74.0;
                state.corruption = 32.0;
                state.rule_of_law = 70.0;
                state.gini = 38.0;
                state.religiosity = 46.0;
                state.clerical_power = 28.0;
                state.religious_extremism = 14.0;
                state.ideological_extremism = 18.0;
                state.active_troops_k = 140.0;
                state.flags.independent_judiciary = true;
            }
            ScenarioPreset::KrasnoviaCommunist => {
                state.nation_name = "People's Republic of Krasnovia".into();
                state.leader_name = "Viktor Morozov".into();
                state.leader_title = "General Secretary".into();
                state.regime = RegimeType::OnePartyCommunist;
                state.economy = EconomicSystem::PlannedCommunism;
                state.religion_policy = ReligionPolicy::StateAtheism;
                state.defense_posture = DefensePosture::PowerProjection;
                state.nuclear_status = NuclearStatus::CivilianResearch;
                state.gdp_bn = 380.0;
                state.population_m = 52.0;
                state.treasury_bn = 24.0;
                state.national_debt_bn = 160.0;
                state.approval = 51.0;
                state.stability = 64.0;
                state.legitimacy = 58.0;
                state.democracy_index = 18.0;
                state.civil_liberties = 24.0;
                state.press_freedom = 16.0;
                state.corruption = 44.0;
                state.rule_of_law = 38.0;
                state.gini = 26.0;
                state.inflation = 5.5;
                state.unemployment = 3.2;
                state.religiosity = 28.0;
                state.clerical_power = 12.0;
                state.religious_extremism = 12.0;
                state.ideological_extremism = 22.0;
                state.active_troops_k = 360.0;
                state.military_readiness = 74.0;
                state.taxes = TaxPolicy {
                    income_tax: 32.0,
                    corporate_tax: 48.0,
                    tariffs: 34.0,
                    wealth_tax: 22.0,
                    state_ownership: 84.0,
                    interest_rate: 4.5,
                };
                state.budget.welfare = 72.0;
                state.budget.military = 68.0;
                state.budget.security_intel = 65.0;
                state.budget.religion_culture = 15.0;
                state.flags.press_censorship = true;
                state.flags.secret_police = true;
                state.flags.opposition_banned = true;
                state.flags.five_year_plan = true;
                state.flags.conscription = true;
                state.adjust_faction(FactionId::LaborUnions, 20.0, 22.0);
                state.adjust_faction(FactionId::Capitalists, -30.0, -25.0);
                state.adjust_faction(FactionId::Clergy, -28.0, -18.0);
                state.adjust_faction(FactionId::SecurityApparatus, 18.0, 22.0);
                state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation = -55.0;
                state.foreign_mut(ForeignPowerId::AtlanticRepublic).sanctioning_us = true;
                state.foreign_mut(ForeignPowerId::PeoplesUnion).relation = 65.0;
                state.foreign_mut(ForeignPowerId::PeoplesUnion).trade_pact = true;
                state.foreign_mut(ForeignPowerId::PeoplesUnion).defense_alliance = true;
            }
            ScenarioPreset::VeridiaCapitalist => {
                state.nation_name = "Federation of Veridia".into();
                state.leader_name = "Marcus Sterling".into();
                state.leader_title = "Chancellor".into();
                state.regime = RegimeType::OligarchicRepublic;
                state.economy = EconomicSystem::FreeMarketCapitalism;
                state.religion_policy = ReligionPolicy::SecularPluralism;
                state.gdp_bn = 690.0;
                state.population_m = 41.0;
                state.treasury_bn = 68.0;
                state.national_debt_bn = 290.0;
                state.gdp_growth = 4.2;
                state.approval = 48.0;
                state.stability = 58.0;
                state.legitimacy = 60.0;
                state.democracy_index = 62.0;
                state.civil_liberties = 72.0;
                state.press_freedom = 68.0;
                state.corruption = 42.0;
                state.rule_of_law = 66.0;
                state.gini = 52.0;
                state.unemployment = 7.4;
                state.ideological_extremism = 26.0;
                state.taxes = TaxPolicy {
                    income_tax: 16.0,
                    corporate_tax: 12.0,
                    tariffs: 6.0,
                    wealth_tax: 0.0,
                    state_ownership: 8.0,
                    interest_rate: 3.5,
                };
                state.budget.welfare = 24.0;
                state.budget.healthcare = 34.0;
                state.budget.research = 68.0;
                state.budget.infrastructure = 62.0;
                state.flags.independent_judiciary = true;
                state.adjust_faction(FactionId::Capitalists, 24.0, 30.0);
                state.adjust_faction(FactionId::LaborUnions, -24.0, 8.0);
                state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation = 58.0;
                state.foreign_mut(ForeignPowerId::AtlanticRepublic).trade_pact = true;
                state.foreign_mut(ForeignPowerId::PeoplesUnion).relation = -28.0;
            }
            ScenarioPreset::SolariaDictatorship => {
                state.nation_name = "Military State of Solaria".into();
                state.leader_name = "Gen. Rafael Cortez".into();
                state.leader_title = "Supreme Commander".into();
                state.regime = RegimeType::MilitaryDictatorship;
                state.economy = EconomicSystem::StateCapitalism;
                state.religion_policy = ReligionPolicy::OfficialReligion;
                state.defense_posture = DefensePosture::PowerProjection;
                state.gdp_bn = 290.0;
                state.population_m = 29.0;
                state.treasury_bn = 18.0;
                state.national_debt_bn = 195.0;
                state.approval = 38.0;
                state.stability = 49.0;
                state.legitimacy = 42.0;
                state.democracy_index = 14.0;
                state.civil_liberties = 20.0;
                state.press_freedom = 18.0;
                state.corruption = 58.0;
                state.rule_of_law = 28.0;
                state.gini = 47.0;
                state.inflation = 8.4;
                state.active_troops_k = 310.0;
                state.military_readiness = 78.0;
                state.ideological_extremism = 28.0;
                state.flags.martial_law = true;
                state.flags.press_censorship = true;
                state.flags.term_limits_abolished = true;
                state.flags.conscription = true;
                state.budget.military = 78.0;
                state.budget.security_intel = 70.0;
                state.budget.education = 30.0;
                state.adjust_faction(FactionId::Military, 10.0, 32.0);
                state.adjust_faction(FactionId::Intelligentsia, -30.0, -10.0);
                state.adjust_faction(FactionId::Nationalists, 16.0, 15.0);
                state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation = -35.0;
                state.foreign_mut(ForeignPowerId::BorderlandRepublic).relation = -45.0;
            }
            ScenarioPreset::AlNoorTheocracy => {
                state.nation_name = "Sacred Republic of Al-Noor".into();
                state.leader_name = "Grand Cleric Yusuf Al-Hakim".into();
                state.leader_title = "Supreme Guardian".into();
                state.regime = RegimeType::TheocraticRepublic;
                state.economy = EconomicSystem::StateCapitalism;
                state.religion_policy = ReligionPolicy::TheocraticLaw;
                state.gdp_bn = 340.0;
                state.population_m = 37.0;
                state.treasury_bn = 52.0;
                state.national_debt_bn = 110.0;
                state.approval = 52.0;
                state.stability = 56.0;
                state.legitimacy = 64.0;
                state.democracy_index = 24.0;
                state.civil_liberties = 22.0;
                state.press_freedom = 20.0;
                state.corruption = 40.0;
                state.rule_of_law = 44.0;
                state.religiosity = 84.0;
                state.clerical_power = 82.0;
                state.religious_extremism = 34.0;
                state.sectarian_tension = 36.0;
                state.ideological_extremism = 16.0;
                state.active_troops_k = 220.0;
                state.budget.religion_culture = 78.0;
                state.budget.security_intel = 64.0;
                state.budget.military = 58.0;
                state.flags.religious_courts = true;
                state.flags.press_censorship = true;
                state.adjust_faction(FactionId::Clergy, 25.0, 35.0);
                state.adjust_faction(FactionId::Intelligentsia, -28.0, -6.0);
                state.foreign_mut(ForeignPowerId::CrescentCoalition).relation = 68.0;
                state.foreign_mut(ForeignPowerId::CrescentCoalition).trade_pact = true;
                state.foreign_mut(ForeignPowerId::AtlanticRepublic).relation = -42.0;
            }
        }

        state.last_fiscal = crate::simulation::calculate_fiscal(&state);
        state.coup_risk = crate::simulation::calculate_coup_risk(&state);
        state.credit_rating = crate::simulation::compute_credit_rating(&state);
        state.record_history();
        state.log(
            LogCategory::Political,
            format!("Inauguration in {}", state.nation_name),
            format!(
                "{} {} assumed executive command under a {} ({}).",
                state.leader_title,
                state.leader_name,
                state.regime.label(),
                state.economy.label()
            ),
        );
        state
    }

    pub fn custom(
        nation_name: String,
        leader_name: String,
        leader_title: String,
        regime: RegimeType,
        economy: EconomicSystem,
        religion_policy: ReligionPolicy,
    ) -> Self {
        let mut state = Self::base_template();
        state.nation_name = if nation_name.trim().is_empty() {
            "New Sovereign Republic".into()
        } else {
            nation_name.trim().into()
        };
        state.leader_name = if leader_name.trim().is_empty() {
            "Alexei Koval".into()
        } else {
            leader_name.trim().into()
        };
        state.leader_title = if leader_title.trim().is_empty() {
            "President".into()
        } else {
            leader_title.trim().into()
        };
        state.regime = regime;
        state.economy = economy;
        state.religion_policy = religion_policy;

        // Tune starting indicators based on chosen regime & economy
        match regime {
            RegimeType::LiberalDemocracy | RegimeType::SocialDemocracy => {
                state.democracy_index = 78.0;
                state.civil_liberties = 75.0;
                state.press_freedom = 74.0;
                state.flags.independent_judiciary = true;
            }
            RegimeType::OnePartyCommunist => {
                state.democracy_index = 20.0;
                state.civil_liberties = 26.0;
                state.press_freedom = 18.0;
                state.flags.opposition_banned = true;
                state.flags.press_censorship = true;
                state.flags.secret_police = true;
                state.adjust_faction(FactionId::LaborUnions, 18.0, 20.0);
                state.adjust_faction(FactionId::Capitalists, -25.0, -20.0);
            }
            RegimeType::MilitaryDictatorship => {
                state.democracy_index = 15.0;
                state.civil_liberties = 22.0;
                state.press_freedom = 20.0;
                state.flags.martial_law = true;
                state.flags.term_limits_abolished = true;
                state.adjust_faction(FactionId::Military, 14.0, 30.0);
            }
            RegimeType::AbsoluteAutocracy => {
                state.democracy_index = 12.0;
                state.civil_liberties = 20.0;
                state.press_freedom = 15.0;
                state.flags.term_limits_abolished = true;
                state.flags.secret_police = true;
                state.adjust_faction(FactionId::SecurityApparatus, 18.0, 24.0);
            }
            RegimeType::TheocraticRepublic => {
                state.democracy_index = 26.0;
                state.civil_liberties = 25.0;
                state.religiosity = 80.0;
                state.clerical_power = 78.0;
                state.flags.religious_courts = true;
                state.adjust_faction(FactionId::Clergy, 22.0, 30.0);
            }
            RegimeType::OligarchicRepublic => {
                state.democracy_index = 54.0;
                state.corruption = 48.0;
                state.gini = 50.0;
                state.adjust_faction(FactionId::Capitalists, 20.0, 28.0);
            }
        }

        match economy {
            EconomicSystem::FreeMarketCapitalism => {
                state.taxes.corporate_tax = 14.0;
                state.taxes.income_tax = 18.0;
                state.taxes.state_ownership = 10.0;
                state.gini = state.gini.max(46.0);
            }
            EconomicSystem::PlannedCommunism => {
                state.taxes.state_ownership = 82.0;
                state.taxes.corporate_tax = 46.0;
                state.gini = 27.0;
                state.flags.five_year_plan = true;
            }
            EconomicSystem::WarEconomy => {
                state.defense_posture = DefensePosture::FullMobilization;
                state.budget.military = 82.0;
                state.taxes.state_ownership = 60.0;
            }
            _ => {}
        }

        match religion_policy {
            ReligionPolicy::StateAtheism => {
                state.religiosity = 28.0;
                state.clerical_power = 12.0;
            }
            ReligionPolicy::TheocraticLaw => {
                state.religiosity = 82.0;
                state.clerical_power = 78.0;
                state.flags.religious_courts = true;
            }
            _ => {}
        }

        state.last_fiscal = crate::simulation::calculate_fiscal(&state);
        state.coup_risk = crate::simulation::calculate_coup_risk(&state);
        state.credit_rating = crate::simulation::compute_credit_rating(&state);
        state.record_history();
        state.log(
            LogCategory::Political,
            format!("Founding of {}", state.nation_name),
            format!(
                "{} {} established a {} with a {} framework.",
                state.leader_title,
                state.leader_name,
                state.regime.label(),
                state.economy.label()
            ),
        );
        state
    }

    fn base_template() -> Self {
        let factions = vec![
            Faction {
                id: FactionId::Capitalists,
                name: "Industrialists & Capitalists".into(),
                approval: 52.0,
                clout: 50.0,
                description: "Corporations, financiers, and private sector elites favoring free markets and low taxes.".into(),
            },
            Faction {
                id: FactionId::LaborUnions,
                name: "Labor Unions & Workers".into(),
                approval: 52.0,
                clout: 50.0,
                description: "Organized workers advocating welfare, low inequality, public employment, and collective rights.".into(),
            },
            Faction {
                id: FactionId::Military,
                name: "Armed Forces Command".into(),
                approval: 58.0,
                clout: 52.0,
                description: "Generals and officer corps guarding sovereignty; dangerous coup threat if alienated while powerful.".into(),
            },
            Faction {
                id: FactionId::Clergy,
                name: "Clergy & Faith Institutions".into(),
                approval: 50.0,
                clout: 42.0,
                description: "Religious leadership shaping moral law, cultural cohesion, and traditional legitimacy.".into(),
            },
            Faction {
                id: FactionId::Intelligentsia,
                name: "Intelligentsia & Urban Youth".into(),
                approval: 54.0,
                clout: 44.0,
                description: "Universities, journalists, and professionals demanding democracy, press freedom, and R&D.".into(),
            },
            Faction {
                id: FactionId::SecurityApparatus,
                name: "Intelligence & Security State".into(),
                approval: 56.0,
                clout: 44.0,
                description: "Internal security services capable of crushing plots—or orchestrating a palace coup.".into(),
            },
            Faction {
                id: FactionId::Nationalists,
                name: "Nationalists & Traditionalists".into(),
                approval: 54.0,
                clout: 46.0,
                description: "Sovereigntists favoring border strength, tariffs, national prestige, and social order.".into(),
            },
        ];

        let foreign_powers = vec![
            ForeignPower {
                id: ForeignPowerId::AtlanticRepublic,
                name: "United Atlantic Republic".into(),
                short_name: "UAR (West)".into(),
                ideology: "Liberal Democracy / Free Market".into(),
                military_power: 95.0,
                economic_power: 96.0,
                relation: 18.0,
                trade_pact: false,
                defense_alliance: false,
                sanctioning_us: false,
                we_sanction_them: false,
                at_war: false,
                war_score: 0.0,
                proxy_funding: false,
                spy_network: false,
            },
            ForeignPower {
                id: ForeignPowerId::PeoplesUnion,
                name: "People's Socialist Union".into(),
                short_name: "PSU (East)".into(),
                ideology: "One-Party Communist / Planned".into(),
                military_power: 90.0,
                economic_power: 88.0,
                relation: 5.0,
                trade_pact: false,
                defense_alliance: false,
                sanctioning_us: false,
                we_sanction_them: false,
                at_war: false,
                war_score: 0.0,
                proxy_funding: false,
                spy_network: false,
            },
            ForeignPower {
                id: ForeignPowerId::EurasianFederation,
                name: "Eurasian Federation".into(),
                short_name: "Eurasia".into(),
                ideology: "Autocracy / State Capitalism".into(),
                military_power: 84.0,
                economic_power: 68.0,
                relation: 0.0,
                trade_pact: false,
                defense_alliance: false,
                sanctioning_us: false,
                we_sanction_them: false,
                at_war: false,
                war_score: 0.0,
                proxy_funding: false,
                spy_network: false,
            },
            ForeignPower {
                id: ForeignPowerId::CrescentCoalition,
                name: "Crescent Theocratic Coalition".into(),
                short_name: "Crescent Bloc".into(),
                ideology: "Theocratic Republic / Oil Bloc".into(),
                military_power: 64.0,
                economic_power: 72.0,
                relation: 4.0,
                trade_pact: false,
                defense_alliance: false,
                sanctioning_us: false,
                we_sanction_them: false,
                at_war: false,
                war_score: 0.0,
                proxy_funding: false,
                spy_network: false,
            },
            ForeignPower {
                id: ForeignPowerId::BorderlandRepublic,
                name: "Borderland Republic".into(),
                short_name: "Borderland".into(),
                ideology: "Disputed Neighbor / Mixed".into(),
                military_power: 46.0,
                economic_power: 42.0,
                relation: -12.0,
                trade_pact: false,
                defense_alliance: false,
                sanctioning_us: false,
                we_sanction_them: false,
                at_war: false,
                war_score: 0.0,
                proxy_funding: false,
                spy_network: false,
            },
            ForeignPower {
                id: ForeignPowerId::GlobalMonetaryCouncil,
                name: "Global Monetary Council".into(),
                short_name: "GMC / IMF".into(),
                ideology: "Multilateral Financial Institution".into(),
                military_power: 15.0,
                economic_power: 98.0,
                relation: 22.0,
                trade_pact: false,
                defense_alliance: false,
                sanctioning_us: false,
                we_sanction_them: false,
                at_war: false,
                war_score: 0.0,
                proxy_funding: false,
                spy_network: false,
            },
        ];

        Self {
            nation_name: "Republic of Aurelia".into(),
            leader_name: "Elena Vance".into(),
            leader_title: "President".into(),
            year: 2026,
            quarter: 1,
            turn: 1,
            political_capital: 45.0,
            regime: RegimeType::LiberalDemocracy,
            economy: EconomicSystem::MixedEconomy,
            religion_policy: ReligionPolicy::SecularPluralism,
            defense_posture: DefensePosture::Defensive,
            nuclear_status: NuclearStatus::None,
            nuclear_progress: 0.0,
            population_m: 36.0,
            gdp_bn: 400.0,
            gdp_growth: 2.6,
            inflation: 3.1,
            unemployment: 5.8,
            treasury_bn: 30.0,
            national_debt_bn: 210.0,
            gini: 37.0,
            trade_balance_bn: 2.5,
            credit_rating: "A".into(),
            imf_austerity_turns: 0,
            approval: 54.0,
            stability: 60.0,
            legitimacy: 66.0,
            democracy_index: 65.0,
            civil_liberties: 64.0,
            press_freedom: 62.0,
            corruption: 34.0,
            rule_of_law: 62.0,
            education_level: 62.0,
            healthcare_level: 60.0,
            infrastructure_level: 58.0,
            tech_level: 55.0,
            religiosity: 48.0,
            clerical_power: 32.0,
            religious_extremism: 15.0,
            ideological_extremism: 16.0,
            sectarian_tension: 18.0,
            insurgency_strength: 8.0,
            active_troops_k: 165.0,
            military_readiness: 64.0,
            military_tech: 56.0,
            war_exhaustion: 0.0,
            coup_risk: 10.0,
            quarters_until_election: 12,
            taxes: TaxPolicy {
                income_tax: 26.0,
                corporate_tax: 24.0,
                tariffs: 12.0,
                wealth_tax: 4.0,
                state_ownership: 28.0,
                interest_rate: 4.0,
            },
            budget: BudgetAllocation {
                healthcare: 52.0,
                education: 52.0,
                infrastructure: 50.0,
                welfare: 46.0,
                military: 48.0,
                security_intel: 42.0,
                research: 45.0,
                religion_culture: 32.0,
            },
            flags: PolicyFlags::default(),
            factions,
            foreign_powers,
            enacted_decisions: Vec::new(),
            decision_cooldowns: HashMap::new(),
            event_cooldowns: HashMap::new(),
            history: Vec::new(),
            logs: Vec::new(),
            active_event: None,
            game_over: None,
            last_fiscal: FiscalSummary::default(),
            rng_state: 0xC0FFEE_2026_1120u64,
        }
    }
}
