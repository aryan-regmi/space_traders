#![allow(unused)]

mod agent;
mod common;
mod contract;
mod faction;
mod ship;
mod waypoint;

pub mod space_traders_client;

#[derive(thiserror::Error, Debug)]
pub enum SpaceTradersError {
    #[error("{0}")]
    RegisterAgentExistsError(String),

    #[error("The callsign must be in between 3 and 14 characters")]
    InvalidCallsignLength,

    #[error("Token must be set first: use `register_callsign` or `initialize_with_token` to set the token.")]
    TokenNotSet,

    #[error("ReqwestError: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("FileError: {0}")]
    FileError(#[from] std::io::Error),

    #[error("SerializeError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("EmptyCache: The cache should not be empty if the client was initalized properly")]
    EmptyCache,

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
