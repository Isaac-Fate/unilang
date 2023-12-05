pub mod chat;
pub mod embedding;
pub mod openai;
pub mod qianfan;

use std::path::PathBuf;
use lazy_static::lazy_static;
use tracing::info;

lazy_static! {
    static ref DOTENV_FILEPATH: Option<PathBuf> = {
        info!("Loading dotenv");
        dotenv::dotenv().ok()
    };
}
