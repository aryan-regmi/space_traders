//! A client for the [SpaceTraders API](https://spacetraders.io/).
//!
//! This crate provides methods to register a new account/agent, load a saved agent, and automate
//! various aspects of the game.

use std::fmt::Display;

mod contract;
mod faction;
mod ship;
mod waypoint;

pub mod agent;
pub mod conditional_types;
pub mod space_traders_client;

pub mod prelude {
    //! Provides common structs and functions.

    pub use crate::agent::Agent;
    pub use crate::conditional_types::*;
    pub use crate::space_traders_client::SpaceTradersClient;
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

    /// Error occured during file creation/access.
    #[error("FileError: {0}")]
    FileError(#[from] std::io::Error),

    /// Error occured during (de)seralization of the client.
    #[error("SerializeError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    /// The client data was not populated correctly.
    #[error("EmptyCache: The cache should not be empty if the client was initalized properly")]
    EmptyCache,

    /// The format of the savefile was incorrect, or the savefile is corrupted.
    #[error("InvalidSave: There was an error reading the savefile: {0}")]
    InvalidSave(String),

    /// Errors from the SpaceTraders API.
    #[error("SpaceTradersResponseError: There was an error with the API response: {0}")]
    SpaceTradersResponseError(#[from] ResponseError),
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ErrorInnerData {
    _symbol: Vec<String>,
}

#[derive(serde::Deserialize, Debug, thiserror::Error)]
#[serde(rename_all = "camelCase")]
pub struct ResponseError {
    message: String,
    code: i32,
    data: ErrorInnerData,
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ResponseError {{ code: {}, message: {}, data: {:?} }}",
            self.code, self.message, self.data
        ))
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ResponseData<T> {
    Data(T),

    Error(ResponseError),
}

pub(crate) type STResult<T> = Result<T, SpaceTradersError>;
