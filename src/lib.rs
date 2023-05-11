#![allow(unused)]

use agent::Agent;
use std::{collections::HashMap, error::Error, path::Path};

mod agent;
mod contract;
mod faction;
mod ship;
mod waypoint;

#[derive(thiserror::Error, Debug)]
enum SpaceTradersError {
    #[error("{0}")]
    RegisterAgentExistsError(String),
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
        message: String,
        code: i32,
        data: ErrorInnerData,
    },
}

type STRes<T> = Result<T, Box<dyn Error>>;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SpaceTradersClient {
    #[serde(skip)]
    client: reqwest::Client,

    #[serde(rename = "token")]
    access_token: Option<String>,

    // Values cached from initial registration
    agent: Option<agent::Agent>,
    contract: Option<contract::Contract>,
    faction: Option<faction::Faction>,
    ship: Option<ship::Ship>,
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
        }
    }

    // FIXME: callsign can't be more than 14 chars
    pub async fn register_callsign(&mut self, callsign: &str) -> STRes<()> {
        use reqwest::header::{HeaderName, HeaderValue, CONTENT_TYPE};

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

        // dbg!(res.status());
        // dbg!(&res.text().await?);

        match res.json::<ResponseData<SpaceTradersClient>>().await? {
            ResponseData::Data(d) => {
                self.access_token = d.access_token;
                self.agent = d.agent;
                self.contract = d.contract;
                self.faction = d.faction;
                self.ship = d.ship;

                Ok(())
            }
            ResponseData::Error { data, .. } => Err(Box::new(
                SpaceTradersError::RegisterAgentExistsError(data.symbol[0].to_owned()),
            )),
        }
    }

    pub fn store_token<T: AsRef<Path>>(&self, filepath: T) -> STRes<()> {
        use std::fs::File;
        use std::io::Write;

        let mut secrets = File::create(filepath)?;

        let token = self
            .access_token
            .as_ref()
            .ok_or("A token must be created using the `register_callsign` function first")?;

        let file_contents = format!("token = {}", token);

        secrets.write_all(file_contents.as_bytes())?;

        Ok(())
    }

    // FIXME: Can only be called after a token is set!
    pub async fn query_agent(&self) -> STRes<Agent> {
        if let Some(agent) = &self.agent {
            Ok(agent.clone())
        } else {
            use reqwest::header::{
                HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE,
            };

            const URL: &str = "https://api.spacetraders.io/v2/my/agent";
            let header: (HeaderName, HeaderValue) = (
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token.as_ref().unwrap()))
                    .unwrap(),
            );

            let mut headers = HeaderMap::new();
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token.as_ref().unwrap()))
                    .unwrap(),
            );

            // Send request
            let res = self.client.get(URL).headers(headers).send().await?;

            match res.json::<ResponseData<Agent>>().await? {
                ResponseData::Data(d) => Ok(d),
                ResponseData::Error { data, .. } => Err(Box::new(
                    SpaceTradersError::RegisterAgentExistsError(data.symbol[0].to_owned()),
                )),
            }
        }
    }

    fn query_waypoint(&self, system_symbol: &str, waypoint_symbol: &str) {
        let url = format!(
            "https://api.spacetraders.io/v2/systems/{}/waypoints/{}",
            system_symbol, waypoint_symbol
        );
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

        st_client.store_token(".env").unwrap();

        // TODO: Check for default values!!!!
        dbg!(st_client);
    }

    #[tokio::test]
    async fn can_query_agent() {
        use dotenv::dotenv;
        dotenv().ok();

        let token = std::env::var("token").expect("token not found");

        let st_client = SpaceTradersClient::initialize_with_token(&token);

        let agent = st_client.query_agent().await;

        dbg!(agent);
    }
}
