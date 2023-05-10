#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Contract {
    id: String,
    faction_symbol: String,

    #[serde(rename = "type")]
    contract_type: ContractType,

    terms: ContractTerms,
    accepted: bool,
    fulfilled: bool,

    // FIXME: Turn into datetime object
    expiration: String,
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
    // FIXME: Turn into datetime object
    deadline: String,

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
    trade_symbol: String,
    destination_symbol: String,
    units_required: i32,
    units_fulfilled: i32,
}
