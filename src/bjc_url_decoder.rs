//! Utilities for decoding bjcloudvod URLs.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::collections::VecDeque;
use thiserror::Error;

const URL_PREFIX: &str = "bjcloudvod://";

/// Error occurs while decoding bjc URL.
#[derive(Error, Debug)]
pub enum BjcUrlDecodeError<'a> {
    /// The given URL is not a bjcloudvod URL, and might be something else.
    #[error("Given str `{0}` is not a valid bjcloudvod URL")]
    InvalidUrl(&'a str),

    /// Could not decode the base64 string in the given URL's body.
    #[error("Could not decode base64 string: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
}

/// Decode a bjcloudvod URL to get the file URL inside it.
///
/// # Params
///
/// - `url`: Given bjcloudvod URL to decode
///
/// # Returns
///
/// Decoded URL string of the given URL, usually a link for an ev1 video file.
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
