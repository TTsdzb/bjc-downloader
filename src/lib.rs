//! Utilities for downloading bjcloudvod URLs.
//!
//! This can be used in another executable file.

use indicatif::MultiProgress;
use thiserror::Error;

pub mod bjc_url_decoder;
pub mod ev1_downloader;

/// Error occurs while downloading bjc URL.
#[derive(Error, Debug)]
pub enum BjcDownloadError {
    /// Could not decode the given bjcloudvod URL.
    #[error("Could not decode given URL: {0}")]
    UrlDecodeError(#[from] bjc_url_decoder::BjcUrlDecodeError),

    /// Could not download the ev1 file from decoded URL.
    #[error("Could not download video file: {0}")]
    Ev1DownloadError(#[from] ev1_downloader::Ev1DownloadError),
}

/// Download a video file from a bjcloudvod URL.
///
/// Decryption is done before writing to file, thus resulting a decrypted flv video.
/// If not specified, the filename is the filename in the given URL, with an additional `.flv` suffix.
///
/// Please note that this function does not overwrite file.
/// If the target file already exists, it fails.
///
/// # Params
///
/// - `url`: Bjcloudvod URL to download
/// - `output_filename`: Filename of the output video file. Suffix should be omitted.
/// - `multi_progress`: A `MultiProgress` instance of [indicatif](https://docs.rs/indicatif/latest/indicatif/). Used to show progress bar.
pub fn download_bjc_url(
    url: &str,
    output_filename: Option<&str>,
    multi_progress: &MultiProgress,
) -> Result<(), BjcDownloadError> {
    let decoded_url = bjc_url_decoder::decode_bjc_url(url)?;
    ev1_downloader::download_ev1_file(&decoded_url, output_filename, multi_progress)?;

    Ok(())
}
