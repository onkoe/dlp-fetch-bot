use anyhow::{bail, Result};
#[allow(unused)]
use log::{debug, error, info, log, warn};
use not_so_human_panic::setup_panic;
use std::env;
use teloxide::{prelude::*, utils::command::BotCommands};
use which::which;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    setup_panic!();

    // Setup logging...

    // If we're in debug mode, let's have debug logs enabled!
    let logging_level = match cfg!(debug_assertions) {
        true => log::Level::Debug,
        false => log::Level::Info,
    };

    simple_logger::init_with_level(logging_level)?;
    info!("Welcome to dlp-fetch-bot!");

    #[cfg(debug_assertions)]
    info!("Debug logging is enabled.");

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
async fn download() -> Result<std::fs::File, anyhow::Error> {
    // attempt download

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
            // let's try to install it for them
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
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Download(video) => {
            bot.send_message(msg.chat.id, format!("The given video link was: `{video}`!"))
                .await?
        }
    };

    Ok(())
}
