use crate::{
    conditional_types::ints::{BoundedInt, LowerBoundInt, NonNegative},
    conditional_types::strings::{Description, Name, Symbol},
    faction::FactionSymbol,
    prelude::Agent,
    space_traders_client::SpaceTradersClient,
    waypoint::{Waypoint, WaypointTraitSymbols, WaypointType},
    ResponseData, STResult, SpaceTradersError,
};
use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Ship {
    pub(crate) symbol: Symbol,
    pub(crate) registration: Registration,
    pub(crate) nav: Nav,
    pub(crate) crew: Crew,
    pub(crate) frame: Frame,
    pub(crate) reactor: Reactor,
    pub(crate) engine: Engine,
    pub(crate) modules: Vec<Module>,
    pub(crate) mounts: Vec<Mount>,
    pub(crate) cargo: Cargo,
    pub(crate) fuel: Fuel,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Nav {
    pub(crate) system_symbol: Symbol,
    pub(crate) waypoint_symbol: Symbol,
    pub(crate) route: Route,
    pub(crate) status: ShipStatus,
    pub(crate) flight_mode: FlightMode,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ShipStatus {
    InTransit,
    InOrbit,
    Docked,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum FlightMode {
    Drift,
    Stealth,
    Cruise,
    Burn,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Route {
    pub(crate) destination: Location,
    pub(crate) departure: Location,
    departure_time: chrono::DateTime<chrono::Utc>,
    arrival: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Location {
    pub(crate) symbol: Symbol,
    #[serde(rename = "type")]
    pub(crate) waypoint_type: WaypointType,
    pub(crate) system_symbol: Symbol,
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Crew {
    pub(crate) current: i32,
    pub(crate) required: i32,
    pub(crate) capacity: i32,
    pub(crate) rotation: Rotation,
    pub(crate) morale: BoundedInt<0, 100>,
    pub(crate) wages: NonNegative,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum Rotation {
    Strict,
    Relaxed,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Fuel {
    pub(crate) current: NonNegative,
    pub(crate) capacity: NonNegative,
    pub(crate) consumed: Consumed,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Consumed {
    pub(crate) amount: NonNegative,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Frame {
    pub(crate) symbol: FrameSymbol,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) condition: BoundedInt<0, 100>,
    pub(crate) module_slots: NonNegative,
    pub(crate) mounting_points: NonNegative,
    pub(crate) fuel_capacity: NonNegative,
    pub(crate) requirements: Requirements,
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Debug, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum FrameSymbol {
    FrameProbe,
    FrameDrone,
    FrameInterceptor,
    FrameRacer,
    FrameFighter,
    FrameFrigate,
    FrameShuttle,
    FrameExplorer,
    FrameMiner,
    FrameLightFreighter,
    FrameHeavyFreighter,
    FrameTransport,
    FrameDestroyer,
    FrameCruiser,
    FrameCarrier,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Reactor {
    pub(crate) symbol: ReactorSymbol,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) condition: BoundedInt<0, 100>,
    pub(crate) power_output: LowerBoundInt<1>,
    pub(crate) requirements: Requirements,
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Debug, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ReactorSymbol {
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Engine {
    pub(crate) symbol: EngineSymbol,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) condition: BoundedInt<0, 100>,
    pub(crate) speed: LowerBoundInt<1>,
    pub(crate) requirements: Requirements,
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Debug, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum EngineSymbol {
    EngineImpulseDriveI,
    EngineIonDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_II")]
    EngineIonDriveII,
    EngineHyperDriveI,
}
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Module {
    pub(crate) symbol: ModuleSymbol,
    pub(crate) capacity: Option<NonNegative>,
    pub(crate) range: Option<NonNegative>,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) requirements: Requirements,
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Debug, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ModuleSymbol {
    ModuleMineralProcessorI,
    ModuleCargoHoldI,
    ModuleCrewQuartersI,
    ModuleEnvoyQuartersI,
    ModulePassengerCabinI,
    ModuleMicroRefineryI,
    ModuleOreRefineryI,
    ModuleFuelRefineryI,
    ModuleScienceLabI,
    ModuleJumpDriveI,
    #[serde(rename = "MODULE_JUMP_DRIVE_II")]
    ModuleJumpDriveII,
    #[serde(rename = "MODULE_JUMP_DRIVE_III")]
    ModuleJumpDriveIII,
    ModuleWarpDriveI,
    #[serde(rename = "MODULE_WARP_DRIVE_II")]
    ModuleWarpDriveII,
    #[serde(rename = "MODULE_WARP_DRIVE_III")]
    ModuleWarpDriveIII,
    ModuleShieldGeneratorI,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_II")]
    ModuleShieldGeneratorII,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Requirements {
    pub(crate) power: Option<i32>,
    pub(crate) crew: Option<i32>,
    pub(crate) slots: Option<i32>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Mount {
    pub(crate) symbol: MountSymbol,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) strength: NonNegative,
    pub(crate) deposits: Option<Vec<Deposit>>,
    pub(crate) requirements: Requirements,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum Deposit {
    QuartzSand,
    SiliconCrystals,
    PreciousStones,
    IceWater,
    AmmoniaIce,
    IronOre,
    CopperOre,
    SilverOre,
    AluminumOre,
    GoldOre,
    PlatinumOre,
    Diamonds,
    UraniteOre,
    MeritiumOre,
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Debug, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum MountSymbol {
    MountGasSiphonI,
    #[serde(rename = "MOUNT_GAS_SIPHON_II")]
    MountGasSiphonII,
    #[serde(rename = "MOUNT_GAS_SIPHON_III")]
    MountGasSiphonIII,
    MountSurveyorI,
    #[serde(rename = "MOUNT_SURVEYOR_II")]
    MountSurveyorII,
    #[serde(rename = "MOUNT_SURVEYOR_III")]
    MountSurveyorIII,
    MountSensorArrayI,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_II")]
    MountSensorArrayII,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_III")]
    MountSensorArrayIII,
    MountMiningLaserI,
    #[serde(rename = "MOUNT_MINING_LASER_II")]
    MountMiningLaserII,
    #[serde(rename = "MOUNT_MINING_LASER_III")]
    MountMiningLaserIII,
    MountLaserCannonI,
    MountMissileLauncherI,
    MountTurretI,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Registration {
    name: Name,
    pub(crate) faction_symbol: FactionSymbol,
    pub(crate) role: Role,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum Role {
    Fabricator,
    Harvester,
    Hauler,
    Interceptor,
    Excavator,
    Transport,
    Repair,
    Surveyor,
    Command,
    Carrier,
    Patrol,
    Satellite,
    Explorer,
    Refinery,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Cargo {
    pub(crate) capacity: NonNegative,
    pub(crate) units: NonNegative,
    pub(crate) inventory: Vec<InventoryItem>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InventoryItem {
    pub(crate) symbol: Symbol,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) units: LowerBoundInt<1>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipType {
    ShipProbe,
    ShipMiningDrone,
    ShipInterceptor,
    ShipLightHauler,
    ShipCommandFrigate,
    ShipExplorer,
    ShipHeavyFreighter,
    ShipLightShuttle,
    ShipOreHound,
    ShipRefiningFreighter,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct InnerShipType {
    #[serde(rename = "type")]
    type_: ShipType,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardTransaction {
    waypoint_symbol: Symbol,
    ship_symbol: Symbol,
    price: LowerBoundInt<1>,
    agent_symbol: Symbol,
    timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ShipyardShip {
    #[serde(rename = "type")]
    type_: ShipType,
    name: Name,
    description: Description,
    purchase_price: i32,
    frame: Frame,
    reactor: Reactor,
    engine: Engine,
    modules: Vec<Module>,
    mounts: Vec<Mount>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Shipyard {
    symbol: Symbol,
    ship_types: Vec<InnerShipType>,
    transactions: Option<Vec<ShipyardTransaction>>,
    ships: Option<Vec<ShipyardShip>>,
}

impl SpaceTradersClient {
    // TODO: Cache shipyards
    //
    /// Finds a shipyard in the system.
    pub async fn find_shipyards(&self, system_symbol: Symbol) -> STResult<Vec<Waypoint>> {
        let url = format!(
            "https://api.spacetraders.io/v2/systems/{}/waypoints",
            system_symbol
        );

        let header = (
            AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "Bearer {}",
                self.token.as_ref().ok_or(SpaceTradersError::TokenNotSet)?
            ))?,
        );

        // Send request
        let res = self
            .client
            .get(url)
            .header(header.0, header.1)
            .send()
            .await?;

        let mut shipyard_waypoints: Vec<Waypoint> = vec![];
        match res.json::<ResponseData<Waypoint>>().await? {
            ResponseData::Data { .. } => unreachable!(),
            ResponseData::PaginatedData { data, .. } => {
                // FIXME: Properly handle paginated data!!

                for waypoint in data {
                    for tr in &waypoint.traits {
                        if tr.symbol == WaypointTraitSymbols::Shipyard {
                            shipyard_waypoints.push(waypoint.clone())
                        }
                    }
                }

                Ok(shipyard_waypoints)
            }
            ResponseData::Error { error } => Err(SpaceTradersError::ResponseError(error)),
        }
    }

    /// NOTE: A ship needs to be at the waypoint inorder to see the ships that are for sale.
    pub async fn view_shipyard(&self, shipyard_waypoint: &Waypoint) -> STResult<Shipyard> {
        let url = format!(
            "https://api.spacetraders.io/v2/systems/{}/waypoints/{}/shipyard",
            shipyard_waypoint.system_symbol, shipyard_waypoint.symbol
        );

        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "Bearer {}",
                self.token.as_ref().ok_or(SpaceTradersError::TokenNotSet)?
            ))?,
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        // Send request
        let res = self.client.get(url).headers(headers).send().await?;

        match res.json::<ResponseData<Shipyard>>().await? {
            ResponseData::Data { data } => Ok(data),
            ResponseData::PaginatedData { .. } => unreachable!(),
            ResponseData::Error { error } => Err(SpaceTradersError::ResponseError(error)),
        }
    }

    pub async fn buy_ship(
        &mut self,
        ship_type: ShipType,
        waypoint_symbol: Symbol,
    ) -> STResult<ShipyardTransaction> {
        const URL: &str = "https://api.spacetraders.io/v2/my/ships";

        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "Bearer {}",
                self.token.as_ref().ok_or(SpaceTradersError::TokenNotSet)?
            ))?,
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        #[derive(Debug, Serialize, Deserialize)]
        struct BuyShipData {
            ship_type: ShipType,
            waypoint_symbol: Symbol,
        }

        // Send request
        let res = self
            .client
            .post(URL)
            .headers(headers)
            .json(&BuyShipData {
                ship_type,
                waypoint_symbol,
            })
            .send()
            .await?;

        #[derive(Debug, Serialize, Deserialize)]
        struct BuyShipResponse {
            agent: Agent,
            ship: Ship,
            transactions: ShipyardTransaction,
        }

        match res.json::<ResponseData<BuyShipResponse>>().await? {
            ResponseData::Data { data } => {
                if let Some(cache) = &mut self.cache {
                    cache.agent = data.agent;
                    cache.ships.push(data.ship)
                }

                Ok(data.transactions)
            }
            ResponseData::PaginatedData { .. } => unreachable!(),
            ResponseData::Error { error } => Err(SpaceTradersError::ResponseError(error)),
        }
    }

    pub async fn dock_ship(&self, ship_symbol: &Symbol) -> STResult<Nav> {
        // Check if ship_symbol exists first
        if let Some(cache) = &self.cache {
            if !cache.ships.iter().any(|s| s.symbol == *ship_symbol) {
                return Err(SpaceTradersError::InvalidShipSymbol(
                    ship_symbol.to_string(),
                ));
            }
        } else {
            return Err(SpaceTradersError::EmptyCache);
        }

        let url = format!(
            "https://api.spacetraders.io/v2/my/ships/{}/dock",
            ship_symbol
        );

        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "bearer {}",
                self.token.as_ref().ok_or(SpaceTradersError::TokenNotSet)?
            ))?,
        );
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Send request
        let res = self.client.post(url).headers(headers).send().await?;

        match res.json::<ResponseData<Nav>>().await? {
            ResponseData::Data { data } => Ok(data),
            ResponseData::PaginatedData { .. } => unreachable!(),
            ResponseData::Error { error } => Err(SpaceTradersError::ResponseError(error)),
        }
    }

    #[allow(unused)]
    pub fn orbit_ship(&self, ship_symbol: Symbol) {
        todo!()
    }

    #[allow(unused)]
    pub fn extract_resource(&self, ship_symbol: Symbol) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_find_shipyards_in_system() -> STResult<()> {
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

    #[tokio::test]
    async fn can_view_shipyard() -> STResult<()> {
        let client = SpaceTradersClient::load_saved()?;

        let shipyard_waypoints = &client.find_shipyards(client.starting_system()?).await?;

        let shipyard = client.view_shipyard(&shipyard_waypoints[0]).await?;

        assert_eq!(shipyard.symbol, shipyard_waypoints[0].symbol);
        assert_eq!(shipyard.ship_types.len(), 4);
        assert_eq!(shipyard.ship_types[0].type_, ShipType::ShipProbe);
        assert_eq!(
            shipyard.ship_types[1].type_,
            ShipType::ShipRefiningFreighter
        );
        assert_eq!(shipyard.ship_types[2].type_, ShipType::ShipOreHound);
        assert_eq!(shipyard.ship_types[3].type_, ShipType::ShipMiningDrone);

        Ok(())
    }

    #[tokio::test]
    async fn can_dock_ship() -> STResult<()> {
        let client = SpaceTradersClient::load_saved()?;

        let ship = &client.cache.as_ref().unwrap().ships[0];

        client.dock_ship(&ship.symbol).await?;

        Ok(())
    }
}
