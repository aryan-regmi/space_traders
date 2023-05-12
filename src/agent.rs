//! Provides functionality to manipulate/interact with a [SpaceTraders Agent](https://spacetraders.stoplight.io/docs/spacetraders/db315e27786ad-agent).

use crate::conditional_types::{Id, NonEmptyString, Symbol};

/// Represents an `Agent` in the API.
#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub(crate) account_id: Id,
    pub(crate) symbol: Symbol,
    pub(crate) headquarters: NonEmptyString,
    pub(crate) credits: i32,
}
