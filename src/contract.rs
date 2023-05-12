use crate::{
    common::{Id, NonEmptyString, Symbol},
    faction::FactionSymbol,
};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Contract {
    id: Id,
    faction_symbol: FactionSymbol,
    #[serde(rename = "type")]
    contract_type: ContractType,
    terms: ContractTerms,
    accepted: bool,
    fulfilled: bool,
    expiration: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ContractType {
    Procurement,
    Transport,
    Shuttle,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ContractTerms {
    deadline: chrono::DateTime<chrono::Utc>,
    payment: Payment,
    deliver: Vec<DeliverInfo>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Payment {
    on_accepted: i32,
    on_fulfilled: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DeliverInfo {
    trade_symbol: Symbol,
    destination_symbol: Symbol,
    units_required: i32,
    units_fulfilled: i32,
}
