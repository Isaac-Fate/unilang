use thiserror::Error;
use serde::Deserialize;
// use anyhow::Error;

#[derive(Debug, Error, Deserialize)]
#[error("QianfanError: {error_code} {error_msg}")]
pub struct QianfanError {
    pub error_code: u32,
    pub error_msg: String,
}
