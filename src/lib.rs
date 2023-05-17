//! A client for the [SpaceTraders API](https://spacetraders.io/).
//!
//! This crate provides methods to register a new account/agent, load a saved agent, and automate
//! various aspects of the game.

use std::fmt::Display;

use meta::Meta;

mod contract;
mod faction;
mod meta;
mod ship;
mod waypoint;

pub mod agent;
pub mod conditional_types;
pub mod space_traders_client;

pub mod prelude {
    //! Provides common structs and functions.

    pub use crate::agent::*;
    pub use crate::conditional_types::ints::*;
    pub use crate::conditional_types::strings::*;
    pub use crate::conditional_types::*;
    pub use crate::space_traders_client::*;
}

/// Represents all possible errors for the [SpaceTradersClient](space_traders_client::SpaceTradersClient).
#[derive(thiserror::Error, Debug)]
pub enum SpaceTradersError {
    /// The callsign passed to [register_callsign](space_traders_client::SpaceTradersClient::register_callsign) is too
    /// short or too long.
    #[error("The callsign must be in between 3 and 14 characters")]
    InvalidCallsignLength,

    /// Tried making API calls without a token.
    #[error("Token must be set first: use `register_callsign` or `initialize_with_token` to set the token.")]
    TokenNotSet,

    /// Errors forwarded from HTTP requests.
    #[error("ReqwestError: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// Forwarded HTTP header errors.
    #[error("ReqwestHeaderError: {0}")]
    ReqwestHeaderError(#[from] reqwest::header::InvalidHeaderValue),

    /// Error occured during file creation/access.
    #[error("FileError: {0}")]
    FileError(#[from] std::io::Error),

    /// Error occured during (de)seralization of the client.
    #[error("SerializeError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    /// The client data was not populated correctly.
    #[error("EmptyCache: The cache should not be empty if the client was initalized properly\ndata: {:?}", 0)]
    EmptyCache(Option<String>),

    /// The format of the savefile was incorrect, or the savefile is corrupted.
    #[error("InvalidSave: There was an error reading the savefile: {0}")]
    InvalidSave(String),

    /// Errors from the SpaceTraders API.
    #[error("SpaceTradersResponseError: There was an error with the API response: {0}")]
    ResponseError(#[from] ResponseError),

    #[error("UrlParseError: There was an error with parsing the URL: {0}")]
    UrlParseError(String),

    #[error("The contract ID `{0}` does not exist in the current client")]
    InvalidContractId(String),

    #[error("The ship `{0}` does not exist in the current client")]
    InvalidShipSymbol(String),
}

#[derive(serde::Deserialize, Debug, thiserror::Error)]
#[serde(rename_all = "camelCase")]
pub struct ResponseError {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ResponseError {{ code: {}, message: {}, data: {:?} }}",
            self.code, self.message, self.data
        ))
    }
}

#[allow(unused)]
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub(crate) enum ResponseData<T> {
    Data { data: T },
    PaginatedData { data: Vec<T>, meta: Meta },
    Error { error: ResponseError },
}

pub(crate) type STResult<T> = Result<T, SpaceTradersError>;
