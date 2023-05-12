use crate::conditional_types::{Description, Name, NonEmptyString, Symbol};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Waypoint {
    symbol: Symbol,
    #[serde(rename = "type")]
    waypoint_type: WaypointType,
    system_symbol: Symbol,
    x: i32,
    y: i32,
    orbitals: Vec<InnerSymbol>,
    faction: Option<InnerSymbol>,
    traits: Vec<Trait>,
    chart: Option<Chart>,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum WaypointType {
    Planet,
    GasGiant,
    Moon,
    OrbitalStation,
    JumpGate,
    AsteroidField,
    Nebula,
    DebrisField,
    GravityWell,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct InnerSymbol {
    symbol: Symbol,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Trait {
    symbol: TraitSymbols,
    name: Name,
    description: Description,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum TraitSymbols {
    Uncharted,
    Marketplace,
    Shipyard,
    Outpost,
    ScatteredSettlements,
    SprawlingCities,
    MegaStructures,
    Overcrowded,
    HighTech,
    Corrupt,
    Bureaucratic,
    TradingHub,
    Industrial,
    BlackMarket,
    ResearchFacility,
    MilitaryBase,
    SurveillanceOutpost,
    ExplorationOutpost,
    MineralDeposits,
    CommonMetalDeposits,
    PreciousMetalDeposits,
    RareMetalDeposits,
    MethanePools,
    IceCrystals,
    ExplosiveGases,
    StrongMagnetosphere,
    VibrantAuroras,
    SaltFlats,
    Canyons,
    PerpetualDaylight,
    PerpetualOvercast,
    DrySeabeds,
    MagmaSeas,
    Supervolcanoes,
    AshClouds,
    VastRuins,
    MutatedFlora,
    Terraformed,
    ExtremeTemperatures,
    ExtremePressure,
    DiverseLife,
    ScarceLife,
    Fossils,
    WeakGravity,
    StrongGravity,
    CrushingGravity,
    ToxicAtmosphere,
    CorrosiveAtmosphere,
    BreathableAtmosphere,
    Jovian,
    Rocky,
    Volcanic,
    Frozen,
    Swamp,
    Barren,
    Temperate,
    Jungle,
    Ocean,
    Stripped,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Chart {
    waypoint_symbol: String,
    submitted_by: String,
    submitted_on: chrono::DateTime<chrono::Utc>,
}
