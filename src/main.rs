use clap::Parser;
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use log::{debug, error};
use std::process::ExitCode;

mod bjc_url_decoder;
mod ev1_downloader;

#[derive(Parser, Debug)]
#[command(version, about = "从百家云 bjcloudvod 链接中下载视频。", long_about = None)]
struct Args {
    #[arg(help = "待下载 URL", long_help = "指定要下载的 bjcloudvod URL。")]
    url: String,
    #[arg(short, long, help = "显示调试信息", long_help = "启用调试信息的输出。")]
    debug: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let mut clog = colog::default_builder();
    if args.debug {
        clog.filter(None, log::LevelFilter::Debug);
    };

    let multi = MultiProgress::new();
    match LogWrapper::new(multi.clone(), clog.build()).try_init() {
        Err(err) => {
            eprintln!("{}", err);
            return ExitCode::FAILURE;
        }
        _ => (),
    };

    let url = &args.url;
    debug!("原始 URL：{}", url);

    let decoded_url = match bjc_url_decoder::decode_bjc_url(url) {
        Ok(result) => result,
        Err(err) => {
            error!("解码 URL 时遇到错误：{}", err);
            return ExitCode::FAILURE;
        }
    };

    debug!("视频链接：{}", &decoded_url);

    match ev1_downloader::download_ev1_file(&decoded_url, &multi) {
        Err(err) => {
            error!("下载视频时遇到错误：{}", err);
            return ExitCode::FAILURE;
        }
        _ => (),
    };

    ExitCode::SUCCESS
}
