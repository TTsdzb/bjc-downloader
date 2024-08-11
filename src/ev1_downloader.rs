//! Utilities for downloading and decrypting ev1 video.

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::debug;
use reqwest::blocking as request;
use std::{
    fs::File,
    io::{self, BufWriter, Read, Write},
    path::Path,
};
use thiserror::Error;
use url::Url;

const DEFAULT_FILENAME: &str = "video";

/// Error occurs while downloading an ev1 video file.
#[derive(Error, Debug)]
pub enum Ev1DownloadError {
    /// The given URL is not valid, and fails when trying to parse it.
    #[error("Could not parse given decoded URL: {0}")]
    InvalidDecodedUrl(#[from] url::ParseError),

    /// Could not create or output to the video file on local filesystem.
    #[error("Failed to write video file: {0}")]
    FileOutputFailed(#[from] io::Error),

    /// Http request on the given url fails.
    #[error("Failed to make http request: {0}")]
    HttpRequestFailed(#[from] reqwest::Error),
}

/// Download an ev1 video file from a decoded URL.
///
/// Decryption is done before writing to file, thus resulting a decrypted flv video.
/// If not specified, the filename is the filename in the given URL, with an additional `.flv` suffix.
///
/// Please note that this function does not overwrite file.
/// If the target file already exists, it fails.
///
/// # Params
///
/// - `decoded_url`: Decoded URL of the ev1 file
/// - `output_filename`: Filename of the output video file. Suffix should be omitted.
/// - `multi_progress`: A `MultiProgress` instance of indicatif. Used to show progress bar.
///
/// # Examples
///
/// ```
/// let url = "http://localhost/test.ev1"
/// let multi = MultiProgress::new();
///
/// download_ev1_file(&decoded_url, None, &multi).unwrap();
///
/// assert!(Path::new("test.ev1.flv").exists());
/// ```
pub fn download_ev1_file(
    decoded_url: &str,
    output_filename: Option<&str>,
    multi_progress: &MultiProgress,
) -> Result<(), Ev1DownloadError> {
    let mut res = request::get(decoded_url)?;

    let url = Url::parse(decoded_url)?;
    let filename = match output_filename {
        Some(name) => name,
        None => match Path::new(url.path()).file_name() {
            Some(name) => name.to_str().unwrap_or(DEFAULT_FILENAME),
            None => DEFAULT_FILENAME,
        },
    };

    debug!("输出文件名：{}", filename);

    let mut file = BufWriter::new(File::create_new(format!("{}.flv", filename))?);

    let pb = multi_progress.add(match res.content_length() {
        Some(len) => ProgressBar::new(len),
        None => ProgressBar::new_spinner(),
    });
    pb.set_style(new_progress_style());

    let mut buf = [0u8; 100];
    let mut read_len = res.read(&mut buf)?;
    // Revert first 100 bytes to decrypt video
    for byte in &mut buf {
        *byte = !*byte;
    }
    file.write(&buf[..read_len])?;
    pb.inc(read_len as u64);
    // Save rest of file
    let mut buf = [0u8; 8192];
    read_len = res.read(&mut buf)?;
    while read_len > 0 {
        file.write(&buf[..read_len])?;
        pb.inc(read_len as u64);
        read_len = res.read(&mut buf)?;
    }

    Ok(())
}

/// Get a `ProgressStyle` for new progress bars.
fn new_progress_style() -> ProgressStyle {
    ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} {bytes_per_sec} (eta {eta_precise})")
        .unwrap()  // This unwrap should be ok, since template is string literal
        .progress_chars("#>-")
}
