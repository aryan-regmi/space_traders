#![allow(unused)]
// TODO: Add basic examples etc to the crate docs.
// TODO: Add README to crate docs: #[doc = include_str!("../README.md")]
//
//! A client for the [SpaceTraders API](https://spacetraders.io/).
//!
//! This crate provides methods to register a new account/agent, load a saved agent, and automate
//! various aspects of the game.

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
    /// The callsign passed to [register_callsign](space_traders_client::SpaceTradersClient::register_callsign) already exists.
    #[error("{0}")]
    RegisterAgentExistsError(String),

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
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorInnerData {
    symbol: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ResponseData<T> {
    Data(T),

    #[serde(rename_all = "camelCase")]
    Error {
        _message: String,
        _code: i32,
        data: ErrorInnerData,
    },
}

pub(crate) type STResult<T> = Result<T, SpaceTradersError>;
