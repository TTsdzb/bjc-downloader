use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::error;
use reqwest::blocking as request;
use std::{
    ffi::OsStr,
    fs::File,
    io::{self, BufWriter, Read, Write},
    path::Path,
};
use thiserror::Error;
use url::Url;

const DEFAULT_FILENAME: &str = "video";

#[derive(Error, Debug)]
pub enum Ev1DownloadError {
    #[error("Could not parse given decoded URL: {0}")]
    InvalidDecodedUrl(#[from] url::ParseError),
    #[error("Failed to write video file: {0}")]
    FileOutputFailed(#[from] io::Error),
    #[error("Failed to make http request: {0}")]
    HttpRequestFailed(#[from] reqwest::Error),
}

pub fn download_ev1_file(
    decoded_url: &str,
    multi_progress: &MultiProgress,
) -> Result<(), Ev1DownloadError> {
    let mut res = request::get(decoded_url)?;

    let url = Url::parse(decoded_url)?;
    let filename = Path::new(url.path())
        .file_name()
        .unwrap_or(OsStr::new(DEFAULT_FILENAME))
        .to_str()
        .unwrap_or(DEFAULT_FILENAME);

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

fn new_progress_style() -> ProgressStyle {
    ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} {bytes_per_sec} (eta {eta_precise})")
        .unwrap()  // This unwrap should be ok, since template is string literal
        .progress_chars("#>-")
}
