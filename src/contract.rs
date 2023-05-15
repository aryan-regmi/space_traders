use crate::{
    conditional_types::{Id, Symbol},
    faction::FactionSymbol,
};

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    id: Id,
    pub(crate) faction_symbol: FactionSymbol,
    #[serde(rename = "type")]
    pub(crate) contract_type: ContractType,
    pub(crate) terms: ContractTerms,
    pub(crate) accepted: bool,
    pub(crate) fulfilled: bool,
    expiration: chrono::DateTime<chrono::Utc>,
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
    deadline: chrono::DateTime<chrono::Utc>,
    payment: Payment,
    pub(crate) deliver: Vec<DeliverInfo>,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Payment {
    on_accepted: i32,
    on_fulfilled: i32,
}

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeliverInfo {
    trade_symbol: Symbol,
    destination_symbol: Symbol,
    units_required: i32,
    pub(crate) units_fulfilled: i32,
}
