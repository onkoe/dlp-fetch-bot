use anyhow::{bail, Context, Result};
use log::{debug, error, info, warn};
use mime_guess::Mime;
use not_so_human_panic::setup_panic;
use reqwest::Client;
use std::env;
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};
use tokio::fs::File;
use url::Url;
use which::which;
use youtube_dl::YoutubeDl;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    setup_panic!();

    // TODO: Parse command-line arguments
    // arg: save location
    // arg: yt-dlp socket timeout ("Time to wait before giving up, in seconds")
    // arg: log to file?

    // Setup logging...
    // TODO: setup logging to a file!

    // If we're in debug mode, let's have debug logs enabled!
    let logging_level = match cfg!(debug_assertions) {
        true => log::Level::Debug,
        false => log::Level::Info,
    };

    simple_logger::init_with_level(logging_level)?;
    info!("Welcome to dlp-fetch-bot!");

    #[cfg(debug_assertions)]
    debug!("Debug logging is enabled. 🧯 💨");

    // Check if TELOXIDE_TOKEN and TELOXIDE_PROXY are both set env variables.
    if env::var("TELOXIDE_TOKEN").is_err() {
        error!("This bot requires an environment variable, TELOXIDE_TOKEN, to be set in order to run. \
         Please see Telegram's documentation for more information: \
         https://core.telegram.org/bots/api#authorizing-your-bot");
        bail!("No token detected.");
    }

    if env::var("TELOXIDE_PROXY").is_err() {
        info!("No proxy was detected.");
    }

    // Test for presence of yt-dlp
    let _yt_dlp_location = check_for_yt_dlp().await?;

    // Create the bot!
    let bot = Bot::from_env();

    Command::repl(bot, answer).await;

    Ok(())
}

struct DownloadedFile {
    path: String,
    opened_file: File,
    #[allow(unused)] // fields are used
    file: youtube_dl::SingleVideo,
    filesize: u64,
    filetype: Mime,
    filename: String,
}

/// Downloads a given yt-dlp-compatible URL. Returns either some error,
/// or a File that's usable immediately.
async fn download(url: Url) -> Result<DownloadedFile, anyhow::Error> {
    // TODO: custom error type to tell user if something went wrong
    // (kinda don't want to use yt-dlp's exact error because of shell nonsense)

    // attempt download

    let video = YoutubeDl::new(url)
        .socket_timeout("15") // seconds
        .download(true)
        .output_directory("dlp-downloads/")
        .output_template("%(id)s.%(ext)s")
        .run_async()
        .await?
        .into_single_video() // TODO: support playlists (create folder for all new files in one command. for each file, upload that mf)
        .context("a downloaded video should exist")?;

    info!("Download successful! Checking filename...");

    // Get a file name using an expected extension, if it has one
    let filename: String = match video.ext.clone() {
        Some(ext) => {
            format!("{}.{}", video.id, ext)
        }
        None => video.id.clone(),
    };

    debug!("Downloaded file name should be: {filename}");

    // TODO: use dynamic path
    let path = format!("dlp-downloads/{filename}");

    Ok(DownloadedFile {
        path: path.clone(),
        opened_file: File::open(&path).await?,
        file: video,
        filesize: std::fs::metadata(&path)?.len(),
        filetype: mime_guess::from_path(path)
            .first()
            .context("a file should have a filetype")?,
        filename,
    })
}

/// Scans the system for a `yt-dlp` binary.
/// If it's not found, then the program exits.
async fn check_for_yt_dlp() -> anyhow::Result<std::path::PathBuf> {
    // check if we have yt-dlp on the system
    let potential_dlp_path = which("yt-dlp");

    match potential_dlp_path {
        Ok(found_path) => {
            info!(
                "yt-dlp was located on your system! It's at: {}",
                found_path.display()
            );
            Ok(found_path)
        }
        Err(error) => {
            error!(
                "Failed to install yt-dlp to your system! \
            Please install it using your package manager: \
            https://github.com/yt-dlp/yt-dlp/wiki/Installation#third-party-package-managers"
            );
            bail!("Required binary not found: {}.", error.to_string());
        }
    }
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "The following commands are supported by this bot:"
)]
enum Command {
    #[command(description = "display this help text.")]
    Help,
    #[command(description = "download a given link with yt-dlp.")]
    Download(String),
}

/// Replies to any given command with a certain operation.
async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    // get message author
    let author: String = msg
        .from()
        .map_or("Unknown Author".to_owned(), |guy| guy.full_name());

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Download(mut link) => {
            // If the video doesn't have a scheme, add it
            if !link.contains("https://") && !link.contains("http://") {
                link = format!("https://{}", link);
            }

            // Check video URL validity
            match Url::parse(&link) {
                Ok(valid_url) => {
                    info!(
                        "User `{}` submitted a valid video URL: `{}`",
                        author,
                        valid_url.as_str()
                    );

                    bot.send_message(msg.chat.id, "Downloading video! This may take a moment...")
                        .await?;

                    match download(valid_url.clone()).await {
                        Ok(dl) => {
                            debug!("Downloaded file size is {}", dl.filesize);
                            // [TODO: make a function for the following..? needs bot.send...() though]

                            // if we get a file, check if it fits file limits: 10mb photos, 50mb others
                            // TODO: check if Telegram uses ISO or SI units
                            if dl.filetype.type_() == mime::IMAGE {
                                debug!("sending image");
                                bot.send_photo(msg.chat.id, InputFile::file(&dl.path))
                                    .await?;
                            } else if dl.filesize < 52428800 {
                                // file's in bounds, send it using Telegram API

                                match dbg!(dl.filetype.type_()) {
                                    mime::VIDEO => {
                                        debug!("sending video");
                                        bot.send_video(msg.chat.id, InputFile::file(&dl.path))
                                            .await?;
                                    }
                                    mime::AUDIO => {
                                        debug!("sending audio");
                                        bot.send_audio(msg.chat.id, InputFile::file(&dl.path))
                                            .await?;
                                    }
                                    other => {
                                        debug!("sending {other}");
                                        bot.send_document(msg.chat.id, InputFile::file(&dl.path))
                                            .await?;
                                    }
                                }
                            } else {
                                debug!("uhhh otherwise..!");
                                // otherwise, upload to some site..?
                                // - https://www.keep.sh: easy https 500mb upload. no account required
                                // - https://temp.sh: easy https _4gb_ upload. no account required
                                // - idk probably some others. magic wormhole would be a pain but works no matter the size /shrug
                            }

                            // send the user the link
                        }
                        Err(dl_error) => {
                            warn!(
                                "The video that user `{author}` submitted failed to download: {dl_error}"
                            );

                            // FIXME: youtube_dl lib has problems with unexpected link parts (i.e. https://www.youtube.com/watch?v=CDWHVRqhfto&t=930s)
                            // see: https://github.com/GyrosOfWar/youtube-dl-rs/issues/49
                            bot.send_message(msg.chat.id, format!("The video link you gave, {valid_url}, failed to download. \
                                Please ensure that your link is \"cleaned,\" like removing timestamps in YouTube links. \
                                Alternatively, try again with another link."))
                                .await?;
                        }
                    }
                }
                Err(error) => {
                    warn!(
                        "Wasn't able to parse user `{}`'s given URL, `{link}`. Parse error: {error}",
                        author
                    );

                    bot.send_message(
                        msg.chat.id,
                        "The URL you provided has an unexpected format, so no content was available for download. \
                        Please check that your URL is what you meant to send. \
                        You can also run /help for other commands and information.",
                    )
                    .await?;
                }
            }
        }
    };

    Ok(())
}
