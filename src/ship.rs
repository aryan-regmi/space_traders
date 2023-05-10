use crate::waypoint::WaypointType;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Ship {
    symbol: String,
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
    system_symbol: String,
    waypoint_symbol: String,
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

    // FIXME: Datetime string
    departure_time: String,

    // FIXME: Datetime string
    arrival: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Location {
    symbol: String,

    #[serde(rename = "type")]
    waypoint_type: WaypointType,

    system_symbol: String,
    x: i32,
    y: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Crew {
    current: i32,
    capacity: i32,
    required: i32,
    rotation: Rotation,

    // FIXME: [0,100]
    morale: i32,

    // FIXME: Non-negative
    wages: i32,
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
    // FIXME: Non-negative
    current: i32,
    // FIXME: Non-negative
    capacity: i32,
    consumed: Consumed,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Consumed {
    // FIXME: Non-negative
    amount: i32,
    // FIXME: Datetime
    timestamp: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Frame {
    symbol: FrameSymbol,
    name: String,
    description: String,

    // FIXME: [0,100]
    condition: i32,

    // FIXME: Non-negative
    module_slots: i32,
    // FIXME: Non-negative
    mounting_points: i32,
    // FIXME: Non-negative
    fuel_capacity: i32,

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
    name: String,
    description: String,

    // FIXME: [0,100]
    condition: i32,

    // FIXME: >= 1
    power_output: i32,
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
    name: String,
    description: String,

    // FIXME: [0,100]
    condition: i32,

    // FIXME: >= 1
    speed: i32,

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
    // FIXME: Non-negative
    capacity: Option<i32>,
    // FIXME: Non-negative
    range: Option<i32>,
    name: String,
    description: String,
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
    name: String,
    description: String,
    // FIXME: Non-negative
    strength: i32,
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
    name: String,
    faction_symbol: String,
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
    // FIXME: Non-negative
    capacity: i32,
    // FIXME: Non-negative
    units: i32,
    inventory: Vec<InventoryItem>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct InventoryItem {
    symbol: String,
    name: String,
    description: String,
    // FIXME: >= 1
    units: i32,
}
