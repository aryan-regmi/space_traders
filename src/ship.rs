use crate::{
    common::{BoundedInt, Description, LowerBoundInt, Name, NonNegative, Symbol},
    faction::FactionSymbol,
    waypoint::WaypointType,
};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Ship {
    symbol: Symbol,
    registration: Registration,
    nav: Nav,
    crew: Crew,
    frame: Frame,
    reactor: Reactor,
    engine: Engine,
    modules: Vec<Module>,
    mounts: Vec<Mount>,
    cargo: Cargo,
    fuel: Fuel,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Nav {
    system_symbol: Symbol,
    waypoint_symbol: Symbol,
    route: Route,
    status: Status,
    flight_mode: FlightMode,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Status {
    InTransit,
    InOrbit,
    Docked,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum FlightMode {
    Drift,
    Stealth,
    Cruise,
    Burn,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Route {
    destination: Location,
    departure: Location,
    departure_time: chrono::DateTime<chrono::Utc>,
    arrival: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Location {
    symbol: Symbol,
    #[serde(rename = "type")]
    waypoint_type: WaypointType,
    system_symbol: Symbol,
    x: i32,
    y: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Crew {
    current: i32,
    required: i32,
    capacity: i32,
    rotation: Rotation,
    morale: BoundedInt<0, 100>,
    wages: NonNegative,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Rotation {
    Strict,
    Relaxed,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Fuel {
    current: NonNegative,
    capacity: NonNegative,
    consumed: Consumed,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Consumed {
    amount: NonNegative,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Frame {
    symbol: FrameSymbol,
    name: Name,
    description: Description,
    condition: BoundedInt<0, 100>,
    module_slots: NonNegative,
    mounting_points: NonNegative,
    fuel_capacity: NonNegative,
    requirements: Requirements,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
enum FrameSymbol {
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

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Reactor {
    symbol: ReactorSymbol,
    name: Name,
    description: Description,
    condition: BoundedInt<0, 100>,
    power_output: LowerBoundInt<1>,
    requirements: Requirements,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ReactorSymbol {
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Engine {
    symbol: EngineSymbol,
    name: Name,
    description: Description,
    condition: BoundedInt<0, 100>,
    speed: LowerBoundInt<1>,
    requirements: Requirements,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum EngineSymbol {
    EngineImpulseDriveI,
    EngineIonDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_II")]
    EngineIonDriveII,
    EngineHyperDriveI,
}
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Module {
    symbol: ModuleSymbol,
    capacity: Option<NonNegative>,
    range: Option<NonNegative>,
    name: Name,
    description: Description,
    requirements: Requirements,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ModuleSymbol {
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

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Requirements {
    power: Option<i32>,
    crew: Option<i32>,
    slots: Option<i32>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Mount {
    symbol: MountSymbol,
    name: Name,
    description: Description,
    strength: NonNegative,
    deposits: Option<Vec<Deposit>>,
    requirements: Requirements,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Deposit {
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

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum MountSymbol {
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

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Registration {
    name: Name,
    faction_symbol: FactionSymbol,
    role: Role,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Role {
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

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Cargo {
    capacity: NonNegative,
    units: NonNegative,
    inventory: Vec<InventoryItem>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct InventoryItem {
    symbol: Symbol,
    name: Name,
    description: Description,
    units: LowerBoundInt<1>,
}
