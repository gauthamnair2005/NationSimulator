# SOVEREIGN // Nation Simulator Dashboard

A pure-dashboard Rust GUI nation simulator (built with `eframe`/`egui`) modeling interconnected real-world **political, economic, military, religious, and extremism** dynamics — with zero cartoon graphics and a clean executive intelligence dashboard UI.

## Features

- **5 Curated Archetype Scenarios + Custom Nation Builder**:
  - *Republic of Aurelia* (Liberal Democracy, Mixed Economy)
  - *People's Republic of Krasnovia* (One-Party Communist State, Planned Economy)
  - *Federation of Veridia* (Oligarchic Free-Market Capitalism)
  - *Military State of Solaria* (Military Junta / Dictatorship)
  - *Sacred Republic of Al-Noor* (Theocratic Republic)
- **Ideologies & Systemic Transitions**:
  - Transition between Democracy, Social Democracy, One-Party Communism, Military Dictatorship, Absolute Autocracy, Theocratic Republic, and Oligarchy.
  - Shift economic systems between Free-Market Capitalism, Mixed Economy, State Capitalism, Planned Communism, and Total War Economy.
- **Macroeconomic & Fiscal Engine**:
  - Real GDP growth, inflation, unemployment, national debt, sovereign credit rating (`AAA` to `D`), Gini inequality, tax & tariff sliders, central bank interest rates, public budget allocations, capital controls, and GMC/IMF emergency bailouts.
- **Domestic Factions, Coups & Elections**:
  - 7 dynamic power blocs (Capitalists, Labor Unions, Armed Forces, Clergy, Intelligentsia, Security Apparatus, Nationalists) with Approval and Clout meters.
  - Live Military Coup Risk calculation, loyalty purges, martial law, press censorship, and interactive election / coup / uprising crises.
- **Religion & Extremism**:
  - State Atheism, Secular Pluralism, Official State Religion, and Supreme Theocratic Law.
  - Religious & ideological extremism indices, sectarian tension, armed insurgency tracking, deradicalization programs, and counter-terror operations.
- **Military & Geopolitics**:
  - Active troop mobilization, conscription, combat readiness, military technology, a 5-stage Nuclear Deterrent program, and 6 foreign powers with trade pacts, defense treaties, sanctions, espionage, proxy wars, and full-scale conventional wars.
- **Persistence**: Save & load your nation state (`sovereign_save.json`).

## Running

```bash
cargo run --release
```

## Testing

```bash
cargo test
```
