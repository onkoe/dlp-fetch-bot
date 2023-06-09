use anyhow::{bail, Result};
use log::{debug, error, info, warn};
use not_so_human_panic::setup_panic;
use std::{env, path::PathBuf, process};
use teloxide::{prelude::*, utils::command::BotCommands};
use url::Url;
use which::which;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    setup_panic!();

    // Parse command-line arguments
    // TODO

    // Setup logging...

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

#[allow(unused)]
async fn download(url: Url) -> Result<std::fs::File, anyhow::Error> {
    // attempt download
    process::Command::new("yt-dlp");

    let video = YoutubeDl::new(url)
        .socket_timeout("30") // seconds
        .run_async()
        .await;

    if let Ok(output) = video {
        //output.into_single_video().unwrap().

        info!(
            "YeAAAAAAHH Downloaded output: {}",
            output.into_single_video().unwrap().title
        );
    }

    // if we get a file, check if it fits file limits: 10mb photos, 50mb others

    // if file is in bounds, send it using Telegram API

    // otherwise, upload to some site..?
    // - https://www.keep.sh: easy https 500mb upload. no account required
    // - https://temp.sh: easy https _4gb_ upload. no account required
    // - idk probably some others. magic wormhole would be a pain but works no matter the size /shrug

    // send the user the link
    Ok(std::fs::File::open("path")?)
}

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
                "yt-dlp was not detected on your system! \
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
        Command::Download(mut video) => {
            // If the video doesn't have a scheme, add it
            if !video.contains("https://") {
                video = format!("https://{}", video);
            }

            // Check video URL validity
            match Url::parse(&video) {
                Ok(valid_url) => {
                    info!(
                        "User `{}` submitted a valid video URL: `{}`",
                        author,
                        valid_url.as_str()
                    );

                    bot.send_message(msg.chat.id, "Downloading video...")
                        .await?;

                    let downloaded_video = download(valid_url);
                }
                Err(error) => {
                    warn!(
                        "Wasn't able to parse user `{}`'s given URL, `{video}`. Parse error: {error}",
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

            #[cfg(debug_assertions)]
            bot.send_message(msg.chat.id, format!("The given video link was: `{video}`!"))
                .await?;
        }
    };

    Ok(())
}
