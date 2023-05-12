use crate::common::{Id, NonEmptyString, Symbol};

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub(crate) account_id: Id,
    pub(crate) symbol: Symbol,
    pub(crate) headquarters: NonEmptyString,
    pub(crate) credits: i32,
}
