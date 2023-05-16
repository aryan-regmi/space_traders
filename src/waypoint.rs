#![allow(unused)]

use crate::{
    conditional_types::{Description, Name, Symbol},
    faction::FactionSymbol,
    space_traders_client::SpaceTradersClient,
    ResponseData, STResult, SpaceTradersError,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InnerFactionSymbol {
    pub(crate) symbol: FactionSymbol,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OrbitalSymbol {
    pub(crate) symbol: Symbol,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Trait {
    pub(crate) symbol: WaypointTraitSymbols,
    pub(crate) name: Name,
    pub(crate) description: Description,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Chart {
    pub waypoint_symbol: Option<Symbol>,
    pub submitted_by: Option<Symbol>,
    pub submitted_on: chrono::DateTime<chrono::Utc>,
}

impl SpaceTradersClient {
    /// Get info on a specific waypoint.
    pub async fn view_waypoint(
        &self,
        system_symbol: Symbol,
        waypoint_symbol: Symbol,
    ) -> STResult<Waypoint> {
        use reqwest::header::AUTHORIZATION;

        if !self.token_set || self.token.is_none() {
            return Err(SpaceTradersError::TokenNotSet);
        }

        let url = format!(
            "https://api.spacetraders.io/v2/systems/{}/waypoints/{}",
            system_symbol, waypoint_symbol
        );

        let header = (AUTHORIZATION, self.token.as_ref().unwrap());

        // Send request
        let res = self
            .client
            .get(url)
            .header(header.0, header.1)
            .send()
            .await?;

        match res.json::<ResponseData<Waypoint>>().await? {
            ResponseData::Data { data } => Ok(data),
            ResponseData::PaginatedData { .. } => unreachable!(),
            ResponseData::Error { error } => {
                Err(SpaceTradersError::SpaceTradersResponseError(error))
            }
        }
    }

    // TODO: Cache shipyards?
    //
    /// Finds a shipyard in the system.
    pub async fn find_shipyards(&self, system_symbol: Symbol) -> STResult<Vec<Waypoint>> {
        use reqwest::header::{HeaderValue, AUTHORIZATION};

        let url = format!(
            "https://api.spacetraders.io/v2/systems/{}/waypoints",
            system_symbol
        );

        let header = (
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token.as_ref().unwrap())).unwrap(),
        );

        // Send request
        let res = self
            .client
            .get(url)
            .header(header.0, header.1)
            .send()
            .await?;

        let mut shipyards: Vec<Waypoint> = vec![];
        match res.json::<ResponseData<Waypoint>>().await? {
            ResponseData::Data { .. } => unreachable!(),
            ResponseData::PaginatedData { data, .. } => {
                // FIXME: Properly handle paginated data!!

                for waypoint in data {
                    for tr in &waypoint.traits {
                        if tr.symbol == WaypointTraitSymbols::Shipyard {
                            shipyards.push(waypoint.clone())
                        }
                    }
                }

                Ok(shipyards)
            }
            ResponseData::Error { error } => {
                Err(SpaceTradersError::SpaceTradersResponseError(error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn check_waypoint_default_valies(waypoint: Waypoint) {
        assert_eq!(waypoint.system_symbol, "X1-ZA40");
        assert_eq!(waypoint.symbol, "X1-ZA40-15970B");
        assert_eq!(waypoint.waypoint_type, WaypointType::Planet);
        assert_eq!(waypoint.x, 10);
        assert_eq!(waypoint.y, 0);

        let orbitals = waypoint.orbitals;
        assert_eq!(orbitals.len(), 3);
        assert_eq!(orbitals[0].symbol, "X1-ZA40-69371X");
        assert_eq!(orbitals[1].symbol, "X1-ZA40-97262C");
        assert_eq!(orbitals[2].symbol, "X1-ZA40-11513D");

        let traits = waypoint.traits;
        assert_eq!(traits.len(), 5);
        assert_eq!(traits[0].symbol, WaypointTraitSymbols::Overcrowded);
        assert_eq!(traits[0].name, "Overcrowded");
        assert_eq!(traits[0].description, "A waypoint teeming with inhabitants, leading to cramped living conditions and a high demand for resources.");
        assert_eq!(traits[1].symbol, WaypointTraitSymbols::HighTech);
        assert_eq!(traits[1].name, "High-Tech");
        assert_eq!(traits[1].description, "A center of innovation and cutting-edge technology, driving progress and attracting skilled individuals from around the galaxy.");
        assert_eq!(traits[2].symbol, WaypointTraitSymbols::Bureaucratic);
        assert_eq!(traits[2].name, "Bureaucratic");
        assert_eq!(traits[2].description, "A waypoint governed by complex regulations, red tape, and layers of administration, often leading to inefficiencies and frustration.");
        assert_eq!(traits[3].symbol, WaypointTraitSymbols::Temperate);
        assert_eq!(traits[3].name, "Temperate");
        assert_eq!(traits[3].description, "A world with a mild climate and balanced ecosystem, providing a comfortable environment for a variety of life forms and supporting diverse industries.");
        assert_eq!(traits[4].symbol, WaypointTraitSymbols::Marketplace);
        assert_eq!(traits[4].name, "Marketplace");
        assert_eq!(traits[4].description, "A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods.");

        let chart = waypoint.chart.unwrap();
        assert_eq!(chart.submitted_by.unwrap(), "COSMIC");
        let submitted_on: chrono::DateTime<chrono::Utc> =
            chrono::DateTime::from_str("2023-05-13T17:48:46.579Z").unwrap();
        assert_eq!(chart.submitted_on, submitted_on);

        assert_eq!(waypoint.faction.unwrap().symbol, FactionSymbol::Cosmic);
    }

    #[tokio::test]
    async fn can_query_waypoint() {
        let client = SpaceTradersClient::load_saved().unwrap();

        let waypoint = client
            .view_waypoint(
                "X1-ZA40".try_into().unwrap(),
                "X1-ZA40-15970B".try_into().unwrap(),
            )
            .await
            .unwrap();

        check_waypoint_default_valies(waypoint);
    }

    #[tokio::test]
    async fn can_find_shipyards() -> STResult<()> {
        let client = SpaceTradersClient::load_saved()?;

        let shipyards = &client.find_shipyards(client.starting_system()?).await?;

        assert_eq!(shipyards.len(), 1);
        assert_eq!(shipyards[0].symbol, "X1-ZA40-68707C");
        assert_eq!(shipyards[0].waypoint_type, WaypointType::OrbitalStation);
        assert_eq!(shipyards[0].system_symbol, "X1-ZA40");
        assert_eq!(shipyards[0].x, -44);
        assert_eq!(shipyards[0].y, -22);
        assert!(shipyards[0].orbitals.is_empty());

        Ok(())
    }
}
