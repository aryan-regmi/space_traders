use crate::{
    conditional_types::ints::{BoundedInt, LowerBoundInt, NonNegative},
    conditional_types::strings::{Description, Name, Symbol},
    faction::FactionSymbol,
    waypoint::WaypointType,
};

#[derive(serde::Deserialize, Debug, serde::Serialize)]
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

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Nav {
    pub(crate) system_symbol: Symbol,
    pub(crate) waypoint_symbol: Symbol,
    pub(crate) route: Route,
    pub(crate) status: ShipStatus,
    pub(crate) flight_mode: FlightMode,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ShipStatus {
    InTransit,
    InOrbit,
    Docked,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum FlightMode {
    Drift,
    Stealth,
    Cruise,
    Burn,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Route {
    pub(crate) destination: Location,
    pub(crate) departure: Location,
    departure_time: chrono::DateTime<chrono::Utc>,
    arrival: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Location {
    pub(crate) symbol: Symbol,
    #[serde(rename = "type")]
    pub(crate) waypoint_type: WaypointType,
    pub(crate) system_symbol: Symbol,
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Crew {
    pub(crate) current: i32,
    pub(crate) required: i32,
    pub(crate) capacity: i32,
    pub(crate) rotation: Rotation,
    pub(crate) morale: BoundedInt<0, 100>,
    pub(crate) wages: NonNegative,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum Rotation {
    Strict,
    Relaxed,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Fuel {
    pub(crate) current: NonNegative,
    pub(crate) capacity: NonNegative,
    pub(crate) consumed: Consumed,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Consumed {
    pub(crate) amount: NonNegative,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
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

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
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

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Reactor {
    pub(crate) symbol: ReactorSymbol,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) condition: BoundedInt<0, 100>,
    pub(crate) power_output: LowerBoundInt<1>,
    pub(crate) requirements: Requirements,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ReactorSymbol {
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Engine {
    pub(crate) symbol: EngineSymbol,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) condition: BoundedInt<0, 100>,
    pub(crate) speed: LowerBoundInt<1>,
    pub(crate) requirements: Requirements,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum EngineSymbol {
    EngineImpulseDriveI,
    EngineIonDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_II")]
    EngineIonDriveII,
    EngineHyperDriveI,
}
#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Module {
    pub(crate) symbol: ModuleSymbol,
    pub(crate) capacity: Option<NonNegative>,
    pub(crate) range: Option<NonNegative>,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) requirements: Requirements,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
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

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Requirements {
    pub(crate) power: Option<i32>,
    pub(crate) crew: Option<i32>,
    pub(crate) slots: Option<i32>,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Mount {
    pub(crate) symbol: MountSymbol,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) strength: NonNegative,
    pub(crate) deposits: Option<Vec<Deposit>>,
    pub(crate) requirements: Requirements,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
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

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
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

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Registration {
    name: Name,
    pub(crate) faction_symbol: FactionSymbol,
    pub(crate) role: Role,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
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

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Cargo {
    pub(crate) capacity: NonNegative,
    pub(crate) units: NonNegative,
    pub(crate) inventory: Vec<InventoryItem>,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InventoryItem {
    pub(crate) symbol: Symbol,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) units: LowerBoundInt<1>,
}
