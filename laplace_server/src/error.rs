use std::io;
use std::net::AddrParseError;

use flexi_logger::FlexiLoggerError;
use laplace_common::lapp::Permission;
use rusqlite::Error as SqlError;
use thiserror::Error;

use crate::lapps::{LappInstanceError, LappSettingsError};
use crate::service::gossipsub;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Web error: {0}")]
    WebError(#[from] hyper::Error),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Parse addr error: {0}")]
    AddrParseError(#[from] AddrParseError),

    #[error("Logger error: {0}")]
    LoggerError(#[from] FlexiLoggerError),

    #[error("TLS error: {0:?}")]
    TlsError(#[from] rustls::Error),

    #[error("Certificate generation error: {0:?}")]
    RcgenError(#[from] rcgen::Error),

    #[error("Missing private key")]
    MissingPrivateKey,

    #[error("Error while generate token")]
    TokenGenerationFail,
}

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Lapp wasm error: {0}")]
    LappWasm(#[from] anyhow::Error),

    #[error("Web error: {0}")]
    WebError(#[from] hyper::Error),

    #[error("Web server error: {0}")]
    WebServerError(#[from] axum::Error),

    #[error("Http error: {0}")]
    HttpError(#[from] axum::http::Error),

    #[error("P2p error: {0}")]
    P2pError(#[from] gossipsub::Error),

    #[error("Wrong parse JSON: {0}")]
    ParseJsonError(#[from] serde_json::Error),

    #[error("Zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("Lapps manager poisoned lock: another task failed inside")]
    LappsManagerNotLock,

    #[error("Lapps poisoned lock")]
    LappNotLock,

    #[error("Lapp '{0}' does not exist")]
    LappNotFound(String),

    #[error("Lapp '{0}' is not enabled")]
    LappNotEnabled(String),

    #[error("Lapp '{0}' is not loaded")]
    LappNotLoaded(String),

    #[error("Lapp '{0}' already exists")]
    LappAlreadyExists(String),

    #[error("Path '{0}' is not lapp directory")]
    WrongLappDirectory(String),

    #[error("Unknown lapp name")]
    UnknownLappName,

    #[error("Permission '{perm}' denied for lapp '{0}'", perm = .1.as_str())]
    LappPermissionDenied(String, Permission),

    #[error("Lapp config operation error: {0}")]
    LappSettingsFail(#[from] LappSettingsError),

    #[error("Lapp file operation error: {0}")]
    LappIoError(#[from] io::Error),

    #[error("Wasm result value has wrong data length")]
    WrongResultLength,

    #[error("Wasm result value cannot be parsed")]
    ResultNotParsed,

    #[error("Lapp instance operation error: {0}")]
    LappInstanceFail(#[from] LappInstanceError),

    #[error("Lapp database operation error: {0:?}")]
    LappDatabaseError(#[from] SqlError),

    #[error("Lapp initialization error: {0:?}")]
    LappInitError(String),

    #[error("Fail to send lapp service for lapp '{0}'")]
    LappServiceSendError(String),
}
