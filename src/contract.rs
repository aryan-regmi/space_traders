use crate::{
    conditional_types::{Id, Symbol},
    faction::FactionSymbol,
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
