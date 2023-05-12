//! Provides the main functionality to interact with the `SpaceTraders API`.

use std::collections::HashMap;

use crate::{
    agent::Agent,
    conditional_types::Symbol,
    contract::Contract,
    faction::{Faction, FactionSymbol},
    ship::Ship,
    waypoint::Waypoint,
    ResponseData, STResult, SpaceTradersError,
};

/// Values cached from initial registration
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CachedInfo {
    agent: Agent,
    contracts: Vec<Contract>,
    faction: Faction,
    ship: Ship,
}

/// The client used to interact with the `SpaceTraders API`.
#[derive(Debug)]
pub struct SpaceTradersClient {
    client: reqwest::Client,
    token: Option<String>,
    token_set: bool,
    cache: Option<CachedInfo>,
}

impl Default for SpaceTradersClient {
    /// Initalize a client to register with.
    ///
    /// **NOTE: [register_callsign](Self::register_callsign) must be called to generate a token and get registration data.**
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            token: None,
            token_set: false,
            cache: None,
        }
    }
}

impl SpaceTradersClient {
    // TODO: Eventually add support for multiple save files
    //
    /// Load a `SpaceTradersClient` from a save file.
    ///
    /// Currently, there can only be one save file (named `spacetraders.save`), though support for
    /// multiple save files will be added in the future.
    ///
    ///
    /// # Example
    /// ```
    /// # use space_traders::prelude::*;
    /// // Creates SpaceTradersClient from the data saved in the "spacetraders.save" file.
    /// let client = SpaceTradersClient::load_saved();
    /// ```
    pub fn load_saved() -> STResult<Self> {
        use std::fs::File;
        use std::io::BufRead;
        use std::io::BufReader;

        const SAVEFILE: &str = "./spacetraders.save";

        let mut save_data = BufReader::new(File::open(SAVEFILE)?).lines();

        // Read first line and get token
        let token = save_data
            .next()
            .ok_or_else(|| SpaceTradersError::InvalidSave(SAVEFILE.into()))??;

        // Read second line and get cache
        let cache: CachedInfo = serde_json::from_str(
            &save_data
                .next()
                .ok_or_else(|| SpaceTradersError::InvalidSave(SAVEFILE.into()))??,
        )?;

        Ok(Self {
            client: reqwest::Client::new(),
            token: Some(token),
            token_set: true,
            cache: Some(cache),
        })
    }

    // TODO: Eventually add support for multiple save files
    //
    /// Saves the `SpaceTradersClient` to a file named `spacetraders.save`.
    ///
    /// This data can be retrieved using [load_saved](SpaceTradersClient::load_saved).
    pub fn save_client(&self) -> STResult<()> {
        use std::fs::File;
        use std::io::Write;

        const SAVEFILE: &str = "./spacetraders.save";

        if !self.token_set || self.token.is_none() {
            return Err(SpaceTradersError::TokenNotSet);
        }

        // Save cached data
        if self.cache.is_some() {
            let token = self.token.as_ref().unwrap();
            let mut cached_data = File::create(SAVEFILE)?;
            let file_contents = format!(
                "{}\n{}",
                token,
                serde_json::to_string(self.cache.as_ref().unwrap())?
            );
            cached_data.write_all(file_contents.as_bytes())?;
        } else {
            return Err(SpaceTradersError::EmptyCache);
        }

        Ok(())
    }

    /// Registers the given (unique) callsign with the Space Traders API.
    ///
    /// The response from the API is used to populate the fields of the [SpaceTradersClient].
    ///
    /// # Example
    /// ```
    /// # use space_traders::prelude::*;
    /// # tokio_test::block_on(async {
    /// // Creates SpaceTradersClient with the `client` field initalized.
    /// let mut client = SpaceTradersClient::default();
    ///
    /// // Populates the `cache` field with values from the response.
    /// // Note: callsign must be unique or an error is returned!
    /// client.register_callsign("UNIQUE_SYMBOL", None).await.unwrap_err();
    /// # })
    /// ```
    pub async fn register_callsign(
        &mut self,
        callsign: &str,
        faction: Option<FactionSymbol>,
    ) -> STResult<()> {
        use reqwest::header::{HeaderName, CONTENT_TYPE};

        let clen = callsign.len();
        if !(3..=14).contains(&clen) {
            return Err(SpaceTradersError::InvalidCallsignLength);
        }

        const URL: &str = "https://api.spacetraders.io/v2/register";
        const HEADER: (HeaderName, &str) = (CONTENT_TYPE, "application/json");

        let mut data = HashMap::with_capacity(1);
        data.insert("symbol", callsign);
        if let Some(faction) = faction {
            data.insert("faction", faction.as_str());
        } else {
            data.insert("faction", "COSMIC");
        }

        // Send request
        let res = self
            .client
            .post(URL)
            .header(HEADER.0, HEADER.1)
            .json(&data)
            .send()
            .await?;

        #[derive(serde::Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct RegistrationResponse {
            token: String,
            agent: Agent,
            contract: Contract,
            faction: Faction,
            ship: Ship,
        }

        match res.json::<ResponseData<RegistrationResponse>>().await? {
            ResponseData::Data(registered_info) => {
                self.token = Some(registered_info.token);
                self.token_set = true;

                self.cache = Some(CachedInfo {
                    agent: registered_info.agent,
                    contracts: vec![registered_info.contract],
                    faction: registered_info.faction,
                    ship: registered_info.ship,
                });

                Ok(())
            }
            ResponseData::Error { data, .. } => Err(SpaceTradersError::RegisterAgentExistsError(
                data.symbol[0].to_owned(),
            )),
        }
    }

    /// Get a reference to the [Agent] associated with the current client.
    pub fn agent(&self) -> STResult<&Agent> {
        if let Some(cache) = &self.cache {
            return Ok(&cache.agent);
        }

        Err(SpaceTradersError::EmptyCache)
    }

    /// Get a mutable reference to the [Agent] associated with the current client.
    pub fn agent_mut(&mut self) -> STResult<&mut Agent> {
        if let Some(cache) = &mut self.cache {
            return Ok(&mut cache.agent);
        }

        Err(SpaceTradersError::EmptyCache)
    }

    pub fn view_waypoint(&self, system_symbol: Symbol, waypoint_symbol: Symbol) -> Waypoint {
        let url = format!(
            "https://api.spacetraders.io/v2/systems/{}/waypoints/{}",
            *system_symbol, *waypoint_symbol
        );
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        contract::ContractType,
        faction::TraitSymbol,
        ship::{
            Deposit, EngineSymbol, FlightMode, FrameSymbol, ModuleSymbol, MountSymbol,
            ReactorSymbol, Role, Rotation, ShipStatus,
        },
        waypoint::WaypointType,
    };

    use super::*;

    fn gen_callsign() -> String {
        use std::thread;
        use std::time::Duration;
        use std::time::Instant;

        thread::sleep(Duration::from_millis(5));

        let t1 = Instant::now();
        let t2 = Instant::now();

        format!(
            "{}TEST{}",
            t1.elapsed().subsec_nanos(),
            t2.elapsed().subsec_nanos()
        )
    }

    fn check_default_values(cache: CachedInfo, callsign: &str) {
        let agent = cache.agent;
        assert_eq!(*agent.symbol, callsign);
        assert_eq!(*agent.headquarters, "X1-DF55-20250Z");
        assert_eq!(agent.credits, 100_000);

        let contracts = cache.contracts;
        assert_eq!(contracts.len(), 1);
        assert_eq!(contracts[0].faction_symbol, FactionSymbol::Cosmic);
        assert_eq!(contracts[0].contract_type, ContractType::Procurement);
        assert_eq!(contracts[0].terms.deliver[0].units_fulfilled, 0);
        assert!(!contracts[0].accepted);
        assert!(!contracts[0].fulfilled);

        let faction = cache.faction;
        assert_eq!(faction.symbol, FactionSymbol::Cosmic);
        assert_eq!(*faction.name, "Cosmic Engineers");
        assert_eq!(*faction.description, "The Cosmic Engineers are a group of highly advanced scientists and engineers who seek to terraform and colonize new worlds, pushing the boundaries of technology and exploration.");
        assert_eq!(*faction.headquarters, "X1-DF55-20250Z");
        assert_eq!(faction.traits.len(), 4);

        let traits = faction.traits;
        assert_eq!(traits[0].symbol, TraitSymbol::Innovative);
        assert_eq!(*traits[0].name, "Innovative");
        assert_eq!(*traits[0].description, "Willing to try new and untested ideas. Sometimes able to come up with creative and original solutions to problems, and may be able to think outside the box. Sometimes at the forefront of technological or social change, and may be willing to take risks in order to advance the boundaries of human knowledge and understanding.");
        assert_eq!(traits[1].symbol, TraitSymbol::Bold);
        assert_eq!(*traits[1].name, "Bold");
        assert_eq!(*traits[1].description, "Unafraid to take risks and challenge the status quo. Sometimes willing to do things that others would not dare, and may be able to overcome obstacles and challenges that would be insurmountable for others. Sometimes able to inspire and motivate others to take bold action as well.");
        assert_eq!(traits[2].symbol, TraitSymbol::Visionary);
        assert_eq!(*traits[2].name, "Visionary");
        assert_eq!(*traits[2].description, "Possessing a clear and compelling vision for the future. Sometimes able to see beyond the present and anticipate the needs and challenges of tomorrow. Sometimes able to inspire and guide others towards a better and brighter future, and may be willing to take bold and decisive action to make their vision a reality.");
        assert_eq!(traits[3].symbol, TraitSymbol::Curious);
        assert_eq!(*traits[3].name, "Curious");
        assert_eq!(*traits[3].description, "Possessing a strong desire to learn and explore. Sometimes interested in a wide range of topics and may be willing to take risks in order to satisfy their curiosity. Sometimes able to think outside the box and come up with creative solutions to challenges.");

        let ship = cache.ship;
        let nav = &ship.nav;
        assert_eq!(*nav.system_symbol, "X1-DF55");
        assert_eq!(*nav.waypoint_symbol, "X1-DF55-20250Z");
        assert_eq!(*nav.route.departure.symbol, "X1-DF55-20250Z");
        assert_eq!(*nav.route.departure.system_symbol, "X1-DF55");
        assert_eq!(nav.route.departure.x, -5);
        assert_eq!(nav.route.departure.y, 9);
        assert_eq!(*nav.route.destination.symbol, "X1-DF55-20250Z");
        assert_eq!(nav.route.destination.waypoint_type, WaypointType::Planet);
        assert_eq!(*ship.nav.route.destination.system_symbol, "X1-DF55");
        assert_eq!(ship.nav.route.destination.x, -5);
        assert_eq!(ship.nav.route.destination.y, 9);
        assert_eq!(ship.nav.status, ShipStatus::Docked);
        assert_eq!(ship.nav.flight_mode, FlightMode::Cruise);

        let crew = ship.crew;
        assert_eq!(crew.current, 0);
        assert_eq!(crew.capacity, 80);
        assert_eq!(crew.required, 59);
        assert_eq!(crew.rotation, Rotation::Strict);
        assert_eq!(*crew.morale, 100);
        assert_eq!(*crew.wages, 0);

        let fuel = ship.fuel;
        assert_eq!(*fuel.current, 1200);
        assert_eq!(*fuel.capacity, 1200);
        assert_eq!(*fuel.consumed.amount, 0);

        let frame = ship.frame;
        assert_eq!(frame.symbol, FrameSymbol::FrameFrigate);
        assert_eq!(*frame.name, "Frame Frigate");
        assert_eq!(*frame.description, "A medium-sized, multi-purpose spacecraft, often used for combat, transport, or support operations.");
        assert_eq!(*frame.module_slots, 8);
        assert_eq!(*frame.mounting_points, 5);
        assert_eq!(*frame.fuel_capacity, 1200);
        assert_eq!(*frame.condition, 100);
        assert_eq!(frame.requirements.power, Some(8));
        assert_eq!(frame.requirements.crew, Some(25));

        let reactor = ship.reactor;
        assert_eq!(reactor.symbol, ReactorSymbol::ReactorFissionI);
        assert_eq!(*reactor.name, "Fission Reactor I");
        assert_eq!(*reactor.description, "A basic fission power reactor, used to generate electricity from nuclear fission reactions.");
        assert_eq!(*reactor.condition, 100);
        assert_eq!(*reactor.power_output, 31);
        assert_eq!(reactor.requirements.crew, Some(8));

        let engine = ship.engine;
        assert_eq!(engine.symbol, EngineSymbol::EngineIonDriveII);
        assert_eq!(*engine.name, "Ion Drive II");
        assert_eq!(*engine.description, "An advanced propulsion system that uses ionized particles to generate high-speed, low-thrust acceleration, with improved efficiency and performance.");
        assert_eq!(*engine.condition, 100);
        assert_eq!(*engine.speed, 30);
        assert_eq!(engine.requirements.power, Some(6));
        assert_eq!(engine.requirements.crew, Some(8));

        let modules = ship.modules;
        assert_eq!(modules.len(), 7);
        assert_eq!(modules[0].symbol, ModuleSymbol::ModuleCargoHoldI);
        assert_eq!(*modules[0].name, "Cargo Hold");
        assert_eq!(
            *modules[0].description,
            "A module that increases a ship's cargo capacity."
        );
        assert_eq!(**modules[0].capacity.as_ref().unwrap(), 30);
        assert_eq!(modules[0].requirements.crew, Some(0));
        assert_eq!(modules[0].requirements.power, Some(1));
        assert_eq!(modules[0].requirements.slots, Some(1));
        assert_eq!(modules[1].symbol, ModuleSymbol::ModuleCargoHoldI);
        assert_eq!(*modules[1].name, "Cargo Hold");
        assert_eq!(
            *modules[1].description,
            "A module that increases a ship's cargo capacity."
        );
        assert_eq!(**modules[1].capacity.as_ref().unwrap(), 30);
        assert_eq!(modules[1].requirements.crew, Some(0));
        assert_eq!(modules[1].requirements.power, Some(1));
        assert_eq!(modules[1].requirements.slots, Some(1));
        assert_eq!(modules[2].symbol, ModuleSymbol::ModuleCrewQuartersI);
        assert_eq!(*modules[2].name, "Crew Quarters");
        assert_eq!(
            *modules[2].description,
            "A module that provides living space and amenities for the crew."
        );
        assert_eq!(**modules[2].capacity.as_ref().unwrap(), 40);
        assert_eq!(modules[2].requirements.crew, Some(2));
        assert_eq!(modules[2].requirements.power, Some(1));
        assert_eq!(modules[2].requirements.slots, Some(1));
        assert_eq!(modules[3].symbol, ModuleSymbol::ModuleCrewQuartersI);
        assert_eq!(*modules[3].name, "Crew Quarters");
        assert_eq!(
            *modules[3].description,
            "A module that provides living space and amenities for the crew."
        );
        assert_eq!(**modules[3].capacity.as_ref().unwrap(), 40);
        assert_eq!(modules[3].requirements.crew, Some(2));
        assert_eq!(modules[3].requirements.power, Some(1));
        assert_eq!(modules[3].requirements.slots, Some(1));
        assert_eq!(modules[4].symbol, ModuleSymbol::ModuleMineralProcessorI);
        assert_eq!(*modules[4].name, "Mineral Processor");
        assert_eq!(
            *modules[4].description,
            "Crushes and processes extracted minerals and ores into their component parts, filters out impurities, and containerizes them into raw storage units."
        );
        assert_eq!(modules[4].requirements.crew, Some(0));
        assert_eq!(modules[4].requirements.power, Some(1));
        assert_eq!(modules[4].requirements.slots, Some(2));
        assert_eq!(modules[5].symbol, ModuleSymbol::ModuleJumpDriveI);
        assert_eq!(*modules[5].name, "Jump Drive I");
        assert_eq!(
            *modules[5].description,
            "A basic antimatter jump drive that allows for instantaneous short-range interdimensional travel."
        );
        assert_eq!(**modules[5].range.as_ref().unwrap(), 500);
        assert_eq!(modules[5].requirements.crew, Some(10));
        assert_eq!(modules[5].requirements.power, Some(4));
        assert_eq!(modules[5].requirements.slots, Some(1));
        assert_eq!(modules[6].symbol, ModuleSymbol::ModuleWarpDriveI);
        assert_eq!(*modules[6].name, "Warp Drive I");
        assert_eq!(
            *modules[6].description,
            "A basic warp drive that allows for short-range interstellar travel."
        );
        assert_eq!(**modules[6].range.as_ref().unwrap(), 2000);
        assert_eq!(modules[6].requirements.crew, Some(2));
        assert_eq!(modules[6].requirements.power, Some(3));
        assert_eq!(modules[6].requirements.slots, Some(1));

        let mounts = ship.mounts;
        assert_eq!(mounts.len(), 3);
        assert_eq!(mounts[0].symbol, MountSymbol::MountSensorArrayI);
        assert_eq!(*mounts[0].name, "Sensor Array I");
        assert_eq!(*mounts[0].description, "A basic sensor array that improves a ship's ability to detect and track other objects in space.");
        assert_eq!(*mounts[0].strength, 1);
        assert_eq!(mounts[0].requirements.crew, Some(0));
        assert_eq!(mounts[0].requirements.power, Some(1));
        assert_eq!(mounts[1].symbol, MountSymbol::MountMiningLaserI);
        assert_eq!(*mounts[1].name, "Mining Laser I");
        assert_eq!(*mounts[1].description, "A basic mining laser that can be used to extract valuable minerals from asteroids and other space objects.");
        assert_eq!(*mounts[1].strength, 10);
        assert_eq!(mounts[1].requirements.crew, Some(0));
        assert_eq!(mounts[1].requirements.power, Some(1));
        assert_eq!(mounts[2].symbol, MountSymbol::MountSurveyorI);
        assert_eq!(*mounts[2].name, "Surveyor I");
        assert_eq!(
            *mounts[2].description,
            "A basic survey probe that can be used to gather information about a mineral deposit."
        );
        assert_eq!(*mounts[2].strength, 1);
        assert_eq!(mounts[2].requirements.crew, Some(2));
        assert_eq!(mounts[2].requirements.power, Some(1));
        let deposits = mounts[2].deposits.as_ref().unwrap();
        assert_eq!(deposits.len(), 11);
        assert_eq!(
            *deposits,
            vec![
                Deposit::QuartzSand,
                Deposit::SiliconCrystals,
                Deposit::PreciousStones,
                Deposit::IceWater,
                Deposit::AmmoniaIce,
                Deposit::IronOre,
                Deposit::CopperOre,
                Deposit::SilverOre,
                Deposit::AluminumOre,
                Deposit::GoldOre,
                Deposit::PlatinumOre,
            ]
        );

        let registration = ship.registration;
        assert_eq!(registration.faction_symbol, FactionSymbol::Cosmic);
        assert_eq!(registration.role, Role::Command);

        let cargo = ship.cargo;
        assert_eq!(*cargo.capacity, 60);
        assert_eq!(*cargo.units, 15);
        assert_eq!(cargo.inventory.len(), 1);
        assert_eq!(*cargo.inventory[0].symbol, "ANTIMATTER");
        assert_eq!(*cargo.inventory[0].name, "Antimatter");
        assert_eq!(*cargo.inventory[0].description, "A highly valuable and dangerous substance used for advanced propulsion and weapons systems.");
        assert_eq!(*cargo.inventory[0].units, 15);
    }

    #[tokio::test]
    async fn can_register_agent() {
        let callsign = gen_callsign();

        let mut client = SpaceTradersClient::default();
        client.register_callsign(&callsign, None).await.unwrap();

        assert!(client.token_set);
        let cache = client.cache.unwrap();
        check_default_values(cache, &callsign);
    }

    #[tokio::test]
    async fn can_save_and_load_client() {
        let callsign = gen_callsign();

        let mut client = SpaceTradersClient::default();
        client.register_callsign(&callsign, None).await.unwrap();

        client.save_client().unwrap();

        let saved_client = SpaceTradersClient::load_saved().unwrap();

        assert_eq!(saved_client.token, client.token);

        // TODO: Check all cached values!
        let cache = client.cache.unwrap();
        let saved_cache = saved_client.cache.unwrap();
        assert_eq!(*saved_cache.ship.symbol, *cache.ship.symbol);
        check_default_values(saved_cache, &callsign);
    }
}
