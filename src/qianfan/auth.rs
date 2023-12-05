use anyhow::Result;
use serde::Deserialize;

use crate::DOTENV_FILEPATH;

lazy_static::lazy_static! {
    static ref QIANFAN_ACCESS_KEY: String = {
        let _ = DOTENV_FILEPATH.as_ref();
        dotenv::var("QIANFAN_ACCESS_KEY").unwrap()
    };
    static ref QIANFAN_SECRET_KEY: String = {
        let _ = DOTENV_FILEPATH.as_ref();
        dotenv::var("QIANFAN_SECRET_KEY").unwrap()
    };
}

pub async fn get_access_token() -> Result<String> {
    Ok(
        reqwest::Client::new()
            .post("https://aip.baidubce.com/oauth/2.0/token")
            .query(&[
                ("grant_type", "client_credentials"),
                ("client_id", QIANFAN_ACCESS_KEY.as_str()),
                ("client_secret", QIANFAN_SECRET_KEY.as_str()),
            ])
            .send()
            .await?
            .json::<TokenResponse>()
            .await?
            .access_token
    )
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
}
