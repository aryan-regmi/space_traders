#![allow(unused)]

use agent::Agent;
use std::{collections::HashMap, path::Path};

mod agent;
mod common;
mod contract;
mod faction;
mod ship;
mod waypoint;

#[derive(thiserror::Error, Debug)]
pub enum SpaceTradersError {
    #[error("{0}")]
    RegisterAgentExistsError(String),

    #[error("The callsign must not be longer than 14 characters")]
    CallsignTooLong,

    #[error("Token must be set first: use `register_callsign` or `initialize_with_token` to set the token.")]
    TokenNotSet,

    #[error("ReqwestError: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("FileError: {0}")]
    FileError(#[from] std::io::Error),
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ErrorInnerData {
    symbol: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum ResponseData<T> {
    Data(T),

    #[serde(rename_all = "camelCase")]
    Error {
        _message: String,
        _code: i32,
        data: ErrorInnerData,
    },
}

type STResult<T> = Result<T, SpaceTradersError>;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpaceTradersClient {
    #[serde(skip)]
    client: reqwest::Client,

    #[serde(rename = "token")]
    access_token: Option<String>,

    #[serde(skip)]
    token_set: bool,

    // Values cached from initial registration
    agent: Option<agent::Agent>,
    contract: Option<contract::Contract>,
    faction: Option<faction::Faction>,
    ship: Option<ship::Ship>,
}

impl Default for SpaceTradersClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SpaceTradersClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            access_token: None,
            agent: None,
            contract: None,
            faction: None,
            ship: None,
            token_set: false,
        }
    }

    // TODO: Make connection and retrieve the agent, contract, faction, and ship info
    pub fn initialize_with_token(access_token: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            access_token: Some(access_token.into()),
            agent: None,
            contract: None,
            faction: None,
            ship: None,
            token_set: true,
        }
    }

    // TODO: Add faction parameter to choose the proper faction
    pub async fn register_callsign(&mut self, callsign: &str) -> STResult<()> {
        use reqwest::header::{HeaderName, CONTENT_TYPE};

        if callsign.len() > 14 {
            return Err(SpaceTradersError::CallsignTooLong);
        }

        const URL: &str = "https://api.spacetraders.io/v2/register";
        const HEADER: (HeaderName, &str) = (CONTENT_TYPE, "application/json");

        let mut data = HashMap::with_capacity(1);
        data.insert("symbol", callsign);
        data.insert("faction", "COSMIC");

        // Send request
        let res = self
            .client
            .post(URL)
            .header(HEADER.0, HEADER.1)
            .json(&data)
            .send()
            .await?;

        match res.json::<ResponseData<SpaceTradersClient>>().await? {
            ResponseData::Data(d) => {
                self.access_token = d.access_token;
                self.agent = d.agent;
                self.contract = d.contract;
                self.faction = d.faction;
                self.ship = d.ship;
                self.token_set = true;

                Ok(())
            }
            ResponseData::Error { data, .. } => Err(SpaceTradersError::RegisterAgentExistsError(
                data.symbol[0].to_owned(),
            )),
        }
    }

    pub fn store_token<T: AsRef<Path>>(&self, filepath: T) -> STResult<()> {
        use std::fs::File;
        use std::io::Write;

        if !self.token_set || self.access_token.is_none() {
            return Err(SpaceTradersError::TokenNotSet);
        }

        let mut secrets = File::create(filepath)?;

        let token = self.access_token.as_ref().unwrap();

        let file_contents = format!("token = {}", token);

        secrets.write_all(file_contents.as_bytes())?;

        Ok(())
    }

    // NOTE: Maybe don't call API at all? the SpaceTradersClient could just keep track of
    // adjustments to the agent
    pub async fn query_agent(&mut self) -> STResult<Agent> {
        if self.token_set {
            if let Some(agent) = &self.agent {
                Ok(agent.clone())
            } else {
                use reqwest::header::{HeaderName, HeaderValue, AUTHORIZATION};

                const URL: &str = "https://api.spacetraders.io/v2/my/agent";
                let header: (HeaderName, HeaderValue) = (
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!(
                        "Bearer {}",
                        self.access_token.as_ref().unwrap()
                    ))
                    .unwrap(),
                );

                // Send request
                let res = self
                    .client
                    .get(URL)
                    .header(header.0, header.1)
                    .send()
                    .await?;

                match res.json::<ResponseData<Agent>>().await? {
                    ResponseData::Data(d) => {
                        self.agent = Some(d.clone());
                        Ok(d)
                    }
                    ResponseData::Error { data, .. } => Err(
                        SpaceTradersError::RegisterAgentExistsError(data.symbol[0].to_owned()),
                    ),
                }
            }
        } else {
            Err(SpaceTradersError::TokenNotSet)
        }
    }

    fn _query_waypoint(
        &self,
        system_symbol: &str,
        waypoint_symbol: &str,
    ) -> STResult<waypoint::Waypoint> {
        let _url = format!(
            "https://api.spacetraders.io/v2/systems/{}/waypoints/{}",
            system_symbol, waypoint_symbol
        );

        // TODO: Make request!
        todo!("Make waypoint request")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_register_agent() {
        let callsign = {
            use std::time::Instant;

            let t1 = Instant::now();
            let t2 = Instant::now();

            format!(
                "{}_TEST_{}",
                t1.elapsed().subsec_nanos(),
                t2.elapsed().subsec_nanos()
            )
        };

        let mut st_client = SpaceTradersClient::new();
        st_client.register_callsign(&callsign).await.unwrap();

        dbg!(&st_client);

        // TODO: Test for default response to callsign registration
        let agent = st_client.agent.unwrap();
        assert_eq!(
            agent.symbol.as_str().to_lowercase(),
            callsign.to_lowercase()
        );
        assert_eq!(agent.headquarters.as_str(), "X1-DF55-20250Z");
        assert_eq!(agent.credits, 100_000);
    }

    #[tokio::test]
    async fn can_query_agent() {
        use dotenv::dotenv;
        dotenv().ok();

        let token = std::env::var("token").expect("token not found");

        let mut st_client = SpaceTradersClient::initialize_with_token(&token);

        let agent = st_client.query_agent().await.unwrap();

        assert_eq!(agent.account_id.as_str(), "clhipuznr0f9os60d1balygxc");
        assert_eq!(agent.symbol.as_str(), "4260_TEST_6216");
        assert_eq!(agent.headquarters.as_str(), "X1-DF55-20250Z");
        assert_eq!(agent.credits, 100_000);
    }
}
