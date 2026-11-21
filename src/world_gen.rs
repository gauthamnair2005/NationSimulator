use crate::models::{ForeignPower, ForeignPowerId};

/// 6 Original Fictional Continents as (lat, lon) polygon loops for the 3D Globe
pub fn fictional_continents() -> &'static [(&'static str, &'static [(f32, f32)])] {
    &[
        (
            "Valdoria",
            &[
                (64.0, -138.0),
                (68.0, -96.0),
                (60.0, -54.0),
                (44.0, -52.0),
                (30.0, -76.0),
                (26.0, -112.0),
                (38.0, -136.0),
                (54.0, -148.0),
            ],
        ),
        (
            "Aethelis",
            &[
                (62.0, -24.0),
                (65.0, 22.0),
                (54.0, 48.0),
                (36.0, 44.0),
                (26.0, 14.0),
                (28.0, -16.0),
                (46.0, -32.0),
            ],
        ),
        (
            "Zahara",
            &[
                (22.0, -22.0),
                (24.0, 42.0),
                (8.0, 58.0),
                (-16.0, 50.0),
                (-34.0, 24.0),
                (-28.0, -4.0),
                (-6.0, -18.0),
                (10.0, -26.0),
            ],
        ),
        (
            "Indravara",
            &[
                (56.0, 62.0),
                (60.0, 118.0),
                (46.0, 148.0),
                (22.0, 144.0),
                (6.0, 112.0),
                (8.0, 74.0),
                (28.0, 58.0),
            ],
        ),
        (
            "Solterra",
            &[
                (12.0, -88.0),
                (10.0, -48.0),
                (-12.0, -36.0),
                (-38.0, -46.0),
                (-56.0, -68.0),
                (-46.0, -84.0),
                (-18.0, -94.0),
            ],
        ),
        (
            "Thalassia",
            &[
                (-12.0, 114.0),
                (-8.0, 156.0),
                (-24.0, 174.0),
                (-46.0, 162.0),
                (-44.0, 126.0),
                (-28.0, 108.0),
            ],
        ),
    ]
}

pub fn core_power_geo(id: ForeignPowerId) -> (f32, f32, &'static str) {
    match id {
        ForeignPowerId::AtlanticRepublic => (44.0, -88.0, "Valdoria"),
        ForeignPowerId::PeoplesUnion => (42.0, 106.0, "Indravara"),
        ForeignPowerId::EurasianFederation => (56.0, 66.0, "Indravara"),
        ForeignPowerId::CrescentCoalition => (14.0, 36.0, "Zahara"),
        ForeignPowerId::BorderlandRepublic => (28.0, 16.0, "Aethelis"),
        ForeignPowerId::GlobalMonetaryCouncil => (46.0, 6.0, "Aethelis"),
        ForeignPowerId::BharatUnion => (22.0, 86.0, "Indravara"),
        ForeignPowerId::NordicConfederation => (58.0, 14.0, "Aethelis"),
        ForeignPowerId::PacificSyndicate => (-22.0, 148.0, "Thalassia"),
        ForeignPowerId::AndeanFederation => (-24.0, -68.0, "Solterra"),
        ForeignPowerId::SaharaAlliance => (-8.0, 18.0, "Zahara"),
        ForeignPowerId::AlpineRepublic => (42.0, -8.0, "Aethelis"),
        ForeignPowerId::VesperaEmpire => (36.0, -24.0, "Aethelis"),
        ForeignPowerId::KharosDesertLeague => (6.0, 46.0, "Zahara"),
        ForeignPowerId::CaledonArchipelago => (-32.0, 132.0, "Thalassia"),
        ForeignPowerId::DrakoriaState => (54.0, -118.0, "Valdoria"),
        ForeignPowerId::LuminaRepublic => (-12.0, -52.0, "Solterra"),
        ForeignPowerId::OakhavenUnion => (38.0, -68.0, "Valdoria"),
        _ => (0.0, 0.0, "Frontier"),
    }
}

/// 160 completely unique, 100% original fictional countries spread across the 6 continents.
/// Zero real-world country names or flag resemblances.
pub fn generate_160_fictional_countries() -> Vec<ForeignPower> {
    const ROOTS: [&str; 160] = [
        "Aerilon", "Alveria", "Arcturia", "Ashvale", "Astralor", "Aurendel", "Avaloria", "Azuria",
        "Baeloria", "Belvoria", "Brambleis", "Branthia", "Briarhold", "Bronzeway", "Borythia", "Caeloria",
        "Caldris", "Cinderis", "Cobaltia", "Corvath", "Crystalia", "Cygnara", "Daeloria", "Dawnspire",
        "Delvara", "Dravenor", "Duskfall", "Ebonreach", "Eldoria", "Eldrath", "Emberis", "Eryndor",
        "Faeloria", "Falkenis", "Fenwicka", "Fjordia", "Flamecrest", "Frostmere", "Galehaven", "Glimmeris",
        "Goldcrest", "Grendalia", "Halcyona", "Hearthis", "Heliosia", "Highgarde", "Holloway", "Hyperia",
        "Ignisia", "Ildaria", "Iridia", "Ironspire", "Ivorynia", "Jadehaven", "Jasperia", "Jorvath",
        "Kaldris", "Kaelenis", "Kestrelia", "Korvadia", "Kragmoor", "Kyrelia", "Lapisia", "Larkspire",
        "Lirienn", "Lorathia", "Lunaris", "Lycoria", "Maeloria", "Marvella", "Mistvale", "Moonspire",
        "Myrland", "Nevoria", "Nimbora", "Norendal", "Nyxara", "Obsidia", "Onyxia", "Opalis",
        "Orvath", "Osteloria", "Pelorion", "Phaedra", "Prismara", "Pyralis", "Quartzis", "Quenara",
        "Quiloria", "Ravensia", "Rivenhold", "Rosariae", "Runehaven", "Sableis", "Saffronia", "Salvoris",
        "Saphiria", "Scarletia", "Seraphia", "Shadowfen", "Shirehold", "Silveria", "Skyreach", "Solariae",
        "Solstice", "Starfall", "Steelmark", "Stormvale", "Sunspire", "Sylvaria", "Taeloria", "Talvoria",
        "Tempestia", "Thornhold", "Tidecrest", "Timberis", "Topazia", "Tridenta", "Ulthara", "Ulvaris",
        "Umbralis", "Vaelenia", "Valcrest", "Valenora", "Valkyria", "Vanguardia", "Veloria", "Verdantia",
        "Vesperis", "Virelia", "Voltaris", "Vortexia", "Wavecrest", "Westmere", "Whisperia", "Wildreach",
        "Windspire", "Winteris", "Wyveria", "Xandoria", "Xeloria", "Xylaria", "Yarrowia", "Yonderis",
        "Yveria", "Zaffiria", "Zeloria", "Zenithia", "Zephyria", "Zirconia", "Zoryana", "Aetheris",
        "Berylia", "Calypsia", "Damaris", "Elaraia", "Ferndalia", "Garnetia", "Hesperia", "Isoldia",
    ];

    const PREFIXES: [&str; 12] = [
        "Republic of",
        "Commonwealth of",
        "Technate of",
        "Principality of",
        "Concord of",
        "Federation of",
        "Dominion of",
        "Freeholds of",
        "League of",
        "Directorate of",
        "Archonate of",
        "Syndicate of",
    ];

    const IDEOLOGIES: [&str; 10] = [
        "Civic Republic / Mixed Economy",
        "Maritime Trade League / Free Market",
        "High-Tech Technocracy / Innovation",
        "Constitutional Monarchy / Mercantile",
        "Socialist Collective / Planned",
        "Industrial Directorate / State Cap",
        "Frontier Federation / Resource Hub",
        "Eco-Cooperative / Green Industry",
        "Clerical Assembly / Traditionalist",
        "Neutral Banking Compact / Finance",
    ];

    // 6 Continental bounding boxes: (region_name, lat_min, lat_max, lon_min, lon_max)
    const REGIONS: [(&str, f32, f32, f32, f32); 6] = [
        ("Valdoria", 30.0, 63.0, -138.0, -56.0),
        ("Aethelis", 28.0, 61.0, -26.0, 42.0),
        ("Zahara", -28.0, 20.0, -18.0, 50.0),
        ("Indravara", 10.0, 54.0, 64.0, 142.0),
        ("Solterra", -50.0, 8.0, -86.0, -40.0),
        ("Thalassia", -44.0, -10.0, 114.0, 168.0),
    ];

    let mut out = Vec::with_capacity(ROOTS.len());
    for (idx, &root) in ROOTS.iter().enumerate() {
        let id_num = (idx as u32) + 1;
        // Deterministic hash for natural geographical scatter, stats & flag seeds
        let h = ((id_num.wrapping_mul(2654435761)) ^ (id_num.wrapping_mul(2246822519))) as usize;
        let reg_idx = idx % REGIONS.len();
        let (region_name, lat_min, lat_max, lon_min, lon_max) = REGIONS[reg_idx];

        let u1 = ((h & 0xFFFF) as f32) / 65535.0;
        let u2 = (((h >> 16) & 0xFFFF) as f32) / 65535.0;
        let lat = lat_min + u1 * (lat_max - lat_min);
        let lon = lon_min + u2 * (lon_max - lon_min);

        let prefix = PREFIXES[(h >> 3) % PREFIXES.len()];
        let ideology = IDEOLOGIES[(h >> 7) % IDEOLOGIES.len()];

        let mil = 18.0 + (((h >> 11) % 62) as f64); // 18..79
        let econ = 16.0 + (((h >> 17) % 68) as f64); // 16..83
        let rel = -18.0 + (((h >> 21) % 48) as f64); // -18..+29

        out.push(ForeignPower {
            id: ForeignPowerId::WorldState(id_num),
            name: format!("{} {}", prefix, root),
            short_name: root.to_string(),
            ideology: ideology.to_string(),
            military_power: mil,
            economic_power: econ,
            relation: rel,
            trade_pact: false,
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
            parent_origin: None,
            flag_seed: ((h >> 5) % 251) as u8,
            lat,
            lon,
            region: region_name.to_string(),
        });
    }
    out
}
