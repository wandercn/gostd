use std::{num::ParseIntError, str::Utf8Error};

use anyhow::Result;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum HTTPConnectError {
    #[error("DNS name conversion failed: {0}")]
    DnsNameConversion(#[from] rustls::pki_types::InvalidDnsNameError),

    #[error("Failed to connect to server: {0}")]
    ConnectionFailure(String),

    #[error("TLS handshake failed: {0}")]
    TlsHandshakeFailure(#[from] rustls::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("From UTF-8 error: {0}")]
    FromUtf8Error(#[from] Utf8Error),

    #[error("Parse integer error: {0}")]
    ParseIntError(#[from] ParseIntError),
}

impl From<String> for HTTPConnectError {
    fn from(err: String) -> Self {
        HTTPConnectError::ConnectionFailure(err)
    }
}

impl From<&'static str> for HTTPConnectError {
    fn from(err: &'static str) -> Self {
        HTTPConnectError::ConnectionFailure(err.to_string())
    }
}

pub type HttpResult<T> = Result<T, HTTPConnectError>;
