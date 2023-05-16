#![allow(unused)]

use crate::{
    conditional_types::{Description, Name, Symbol},
    faction::FactionSymbol,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Waypoint {
    pub(crate) symbol: Symbol,
    #[serde(rename = "type")]
    pub(crate) waypoint_type: WaypointType,
    pub(crate) system_symbol: Symbol,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) orbitals: Vec<OrbitalSymbol>,
    pub(crate) faction: Option<InnerFactionSymbol>,
    pub(crate) traits: Vec<Trait>,
    pub(crate) chart: Option<Chart>,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Eq, Clone, Copy)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InnerFactionSymbol {
    pub(crate) symbol: FactionSymbol,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OrbitalSymbol {
    pub(crate) symbol: Symbol,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Trait {
    pub(crate) symbol: WaypointTraitSymbols,
    pub(crate) name: Name,
    pub(crate) description: Description,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum WaypointTraitSymbols {
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Chart {
    pub waypoint_symbol: Option<Symbol>,
    pub submitted_by: Option<Symbol>,
    pub submitted_on: chrono::DateTime<chrono::Utc>,
}
