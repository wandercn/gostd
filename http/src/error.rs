#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
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

    #[error("http: request method or response status code does not allow body")]
    ErrBodyNotAllowed,

    #[error("http: connection has been hijacked")]
    ErrHijacked,

    #[error("http: wrote more than the declared Content-Length")]
    ErrContentLength,

    #[error("unused")]
    ErrWriteAfterFlush,
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
