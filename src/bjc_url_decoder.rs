use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::collections::VecDeque;
use thiserror::Error;

const URL_PREFIX: &str = "bjcloudvod://";

#[derive(Error, Debug)]
pub enum BjcUrlDecodeError<'a> {
    #[error("Given str `{0}` is not a valid bjcloudvod URL")]
    InvalidUrl(&'a str),
    #[error("Could not decode base64 string: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
}

pub fn decode_bjc_url(url: &str) -> Result<String, BjcUrlDecodeError> {
    if !url.starts_with(URL_PREFIX) {
        return Err(BjcUrlDecodeError::InvalidUrl(url));
    }

    let base64 = &url[13..];

    let mut bytes: VecDeque<u8> = URL_SAFE_NO_PAD.decode(base64)?.into();
    let c: usize = (bytes[0] % 8).into();
    bytes.pop_front();

    let mut result = String::new();
    for i in 0..bytes.len() {
        let step = (i % 4) * c + (i % 3) + 1;
        result.push((bytes[i] - step as u8) as char);
    }

    Ok(result)
}
