#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Agent {
    account_id: String,
    symbol: String,
    headquarters: String,
    credits: i32,
}
