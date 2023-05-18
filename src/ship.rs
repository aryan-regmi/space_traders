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
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE,
};
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

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SurveyDeposit {
    symbol: Deposit,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SurveySize {
    Small,
    Moderate,
    Large,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Survey {
    signature: Symbol,
    symbol: Symbol,
    deposits: Vec<SurveyDeposit>,
    expiration: DateTime<Utc>,
    size: SurveySize,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    ship_symbol: Symbol,
    total_seconds: NonNegative,
    remaining_seconds: NonNegative,
    expiration: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExtractionYield {
    symbol: Symbol,
    units: i32,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Extraction {
    ship_symbol: Symbol,
    #[serde(rename = "yield")]
    yield_: ExtractionYield,
}

impl SpaceTradersClient {
    /// NOTE: A ship needs to be docked at the waypoint to see the ships that are for sale.
    pub async fn view_shipyard(
        &self,
        system_symbol: &Symbol,
        waypoint_symbol: &Symbol,
    ) -> STResult<Shipyard> {
        let url = format!(
            "https://api.spacetraders.io/v2/systems/{}/waypoints/{}/shipyard",
            system_symbol, waypoint_symbol
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
                } else {
                    // The ship is already purchased at this point, so the transaction is returned
                    // as a part of the error
                    return Err(SpaceTradersError::EmptyCache(Some(
                        serde_json::json!(data.transactions).to_string(),
                    )));
                }

                Ok(data.transactions)
            }
            ResponseData::PaginatedData { .. } => unreachable!(),
            ResponseData::Error { error } => Err(SpaceTradersError::ResponseError(error)),
        }
    }

    pub async fn dock_ship(&self, ship_symbol: &Symbol) -> STResult<Nav> {
        //  If the ship is already docked, dont make API call
        let ship = self.get_ship(ship_symbol)?;
        if ship.nav.status == ShipStatus::Docked {
            return Ok(ship.nav.clone());
        }

        let url = format!(
            "https://api.spacetraders.io/v2/my/ships/{}/dock",
            ship_symbol
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
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

        // Send request
        let res = self.client.post(url).headers(headers).send().await?;

        // dbg!(res.json::<serde_json::Value>().await?);
        // todo!();

        #[derive(Debug, Deserialize, Serialize)]
        struct DockShipResponse {
            nav: Nav,
        }

        match res.json::<ResponseData<DockShipResponse>>().await? {
            ResponseData::Data { data } => Ok(data.nav),
            ResponseData::PaginatedData { .. } => unreachable!(),
            ResponseData::Error { error } => Err(SpaceTradersError::ResponseError(error)),
        }
    }

    pub async fn orbit_ship(&self, ship_symbol: &Symbol) -> STResult<Nav> {
        //  If the ship is already docked, dont make API call
        let ship = self.get_ship(ship_symbol)?;
        if ship.nav.status == ShipStatus::InOrbit {
            return Ok(ship.nav.clone());
        }

        let url = format!(
            "https://api.spacetraders.io/v2/my/ships/{}/orbit",
            ship_symbol
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
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

        // Send request
        let res = self.client.post(url).headers(headers).send().await?;

        #[derive(Debug, Deserialize, Serialize)]
        struct DockShipResponse {
            nav: Nav,
        }

        match res.json::<ResponseData<DockShipResponse>>().await? {
            ResponseData::Data { data } => Ok(data.nav),
            ResponseData::PaginatedData { .. } => unreachable!(),
            ResponseData::Error { error } => Err(SpaceTradersError::ResponseError(error)),
        }
    }

    pub async fn extract_resources(
        &mut self,
        ship_symbol: &Symbol,
        survey: Option<Survey>,
    ) -> STResult<(Cooldown, Extraction)> {
        // Check if ship_symbol exists first
        self.get_ship(ship_symbol)?;

        let url = format!(
            "https://api.spacetraders.io/v2/my/ships/{}/extract",
            ship_symbol
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
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

        // Send request
        let res = match survey {
            // Send survey as body if there is one
            Some(survey) => {
                self.client
                    .post(url)
                    .headers(headers)
                    .json(&survey)
                    .send()
                    .await?
            }
            None => self.client.post(url).headers(headers).send().await?,
        };

        #[derive(Debug, Deserialize, Serialize)]
        struct ExtractResourceResponse {
            cooldown: Cooldown,
            extraction: Extraction,
            cargo: Cargo,
        }

        match res.json::<ResponseData<ExtractResourceResponse>>().await? {
            ResponseData::Data { data } => {
                // Update the ship's cargo with the new cargo
                let ship = self.get_ship_mut(ship_symbol)?;
                ship.cargo = data.cargo;

                Ok((data.cooldown, data.extraction))
            }
            ResponseData::PaginatedData { .. } => unreachable!(),
            ResponseData::Error { error } => Err(SpaceTradersError::ResponseError(error)),
        }
    }

    fn get_ship_mut(&mut self, ship_symbol: &Symbol) -> STResult<&mut Ship> {
        match &mut self.cache {
            Some(cache) => {
                for ship in &mut cache.ships {
                    if ship.symbol == *ship_symbol {
                        return Ok(ship);
                    }
                }

                Err(SpaceTradersError::InvalidShipSymbol(format!(
                    "ShipNotFound: {}",
                    ship_symbol
                )))
            }
            None => Err(SpaceTradersError::EmptyCache(None)),
        }
    }

    fn get_ship(&self, ship_symbol: &Symbol) -> STResult<&Ship> {
        match &self.cache {
            Some(cache) => {
                for ship in &cache.ships {
                    if ship.symbol == *ship_symbol {
                        return Ok(ship);
                    }
                }

                Err(SpaceTradersError::InvalidShipSymbol(format!(
                    "ShipNotFound: {}",
                    ship_symbol
                )))
            }
            None => Err(SpaceTradersError::EmptyCache(None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_view_shipyard() -> STResult<()> {
        let client = SpaceTradersClient::load_saved()?;

        let system_symbol = Symbol::new("X1-ZA40").unwrap();
        let waypoint_symbol = Symbol::new("X1-ZA40-68707C").unwrap();

        let shipyard = client
            .view_shipyard(&system_symbol, &waypoint_symbol)
            .await?;

        dbg!(&shipyard);

        assert_eq!(shipyard.symbol, "X1-ZA40-68707C");
        assert_eq!(shipyard.ship_types.len(), 4);
        assert_eq!(
            shipyard.ship_types[0].type_,
            ShipType::ShipRefiningFreighter
        );
        assert_eq!(shipyard.ship_types[1].type_, ShipType::ShipProbe);
        assert_eq!(shipyard.ship_types[2].type_, ShipType::ShipOreHound);
        assert_eq!(shipyard.ship_types[3].type_, ShipType::ShipMiningDrone);
        assert!(shipyard.transactions.is_none());
        assert!(shipyard.ships.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn can_dock_and_orbit_ship() -> STResult<()> {
        let client = SpaceTradersClient::load_saved()?;

        let ship = &client.cache.as_ref().unwrap().ships[0];

        let nav = client.dock_ship(&ship.symbol).await?;
        assert_eq!(nav.status, ShipStatus::Docked);
        assert_eq!(nav.system_symbol, ship.nav.system_symbol);
        assert_eq!(nav.waypoint_symbol, ship.nav.waypoint_symbol);

        let nav = client.orbit_ship(&ship.symbol).await?;
        assert_eq!(nav.status, ShipStatus::InOrbit);
        assert_eq!(nav.system_symbol, ship.nav.system_symbol);
        assert_eq!(nav.waypoint_symbol, ship.nav.waypoint_symbol);

        Ok(())
    }
}
