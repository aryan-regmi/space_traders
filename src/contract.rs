use crate::{
    conditional_types::{Id, Symbol},
    faction::FactionSymbol,
    prelude::Agent,
    space_traders_client::SpaceTradersClient,
    ResponseData, STResult, SpaceTradersError,
};

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub(crate) id: Id,
    pub(crate) faction_symbol: FactionSymbol,
    #[serde(rename = "type")]
    pub(crate) contract_type: ContractType,
    pub(crate) terms: ContractTerms,
    pub(crate) accepted: bool,
    pub(crate) fulfilled: bool,
    pub(crate) expiration: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ContractType {
    Procurement,
    Transport,
    Shuttle,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ContractTerms {
    pub(crate) deadline: chrono::DateTime<chrono::Utc>,
    pub(crate) payment: Payment,
    pub(crate) deliver: Vec<DeliverInfo>,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Payment {
    pub(crate) on_accepted: i32,
    pub(crate) on_fulfilled: i32,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeliverInfo {
    pub(crate) trade_symbol: Symbol,
    pub(crate) destination_symbol: Symbol,
    pub(crate) units_required: i32,
    pub(crate) units_fulfilled: i32,
}

impl SpaceTradersClient {
    /// Accept a specific contract given its ID.
    pub async fn accept_contract(&mut self, contract_id: Id) -> STResult<()> {
        use reqwest::header::{
            HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE,
        };

        let cache = self.cache.as_mut().ok_or(SpaceTradersError::EmptyCache)?;

        let mut idx: Option<usize> = None; // Stores index of the given contract
        for (i, contract) in cache.contracts.iter_mut().enumerate() {
            if contract.id == *contract_id {
                // Return w/out making API calls if the contract is already accepted
                if contract.accepted {
                    return Ok(());
                } else {
                    idx = Some(i);
                    break;
                }
            }
        }

        // The contract id was not found in the cached data
        if idx.is_none() {
            return Err(SpaceTradersError::InvalidContractId(
                contract_id.to_string(),
            ));
        }

        let url = format!(
            "https://api.spacetraders.io/v2/my/contracts/{}/accept",
            contract_id
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token.as_ref().unwrap()))
                .map_err(SpaceTradersError::ReqwestHeaderError)?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

        // Send request
        let res = self.client.post(url).headers(headers).send().await?;

        #[derive(Debug, serde::Deserialize)]
        struct AcceptContractResponse {
            #[serde(rename = "agent")]
            _agent: Agent,
            #[serde(rename = "contract")]
            _contract: Contract,
        }

        if let ResponseData::Error { error } =
            res.json::<ResponseData<AcceptContractResponse>>().await?
        {
            return Err(SpaceTradersError::ResponseError(error));
        }

        // Find the contract with the given id, and set accepted to true
        cache.contracts[idx.unwrap()].accepted = true;
        cache.agent.credits += cache.contracts[idx.unwrap()].terms.payment.on_accepted;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn gen_callsign() -> String {
        use uuid::Uuid;

        let callsign = Uuid::new_v4();

        callsign.to_string().get(0..13).unwrap().to_uppercase()
    }

    #[test]
    fn can_list_contracts() {
        let client = SpaceTradersClient::load_saved().unwrap();

        let contracts = client.contracts().unwrap();

        assert_eq!(contracts.len() as i32, 1);

        let contract = &contracts[0];
        assert_eq!(contract.id, "clhr6zx0r07s2s60daxqce7b1");
        assert_eq!(contract.faction_symbol, FactionSymbol::Cosmic);
        assert_eq!(contract.contract_type, ContractType::Procurement);

        let terms = &contract.terms;
        assert_eq!(
            terms.deadline,
            chrono::DateTime::<chrono::Utc>::from_str("2023-05-24T04:18:05.930Z").unwrap()
        );
        assert_eq!(terms.payment.on_accepted, 100_280);
        assert_eq!(terms.payment.on_fulfilled, 401_120);

        let deliver = &terms.deliver[0];
        assert_eq!(terms.deliver.len(), 1);
        assert_eq!(deliver.trade_symbol, "IRON_ORE");
        assert_eq!(deliver.destination_symbol, "X1-ZA40-15970B");
        assert_eq!(deliver.units_required, 10_900);
        assert_eq!(deliver.units_fulfilled, 0);

        assert!(!contract.accepted);
        assert!(!contract.fulfilled);
        assert_eq!(
            contract.expiration,
            chrono::DateTime::<chrono::Utc>::from_str("2023-05-20T04:18:05.930Z").unwrap()
        );
    }

    #[tokio::test]
    async fn can_accept_contract() {
        let mut client = SpaceTradersClient::new();
        client
            .register_callsign(&gen_callsign(), None)
            .await
            .unwrap();

        let id = client.contracts().unwrap()[0].id.clone();
        client.accept_contract(id).await.unwrap();

        let contract = &client.contracts().unwrap()[0];
        assert!(contract.accepted);

        let credits = 100_000 + contract.terms.payment.on_accepted;
        assert_eq!(client.agent().unwrap().credits, credits);
    }
}
