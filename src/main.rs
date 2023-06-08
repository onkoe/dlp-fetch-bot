use anyhow::Result;
#[allow(unused)]
use log::{debug, error, info, log, warn};
use not_so_human_panic::setup_panic;
#[allow(unused)]
use smol::{prelude::*, Async};
use teloxide::{prelude::*, utils::command::BotCommands};

fn main() -> Result<(), anyhow::Error> {
    smol::block_on(async {
        setup_panic!();
        pretty_env_logger::init();
        info!("Welcome to dlp-fetch-bot!");

        // Create the bot!
        // Please ensure that TELOXIDE_TOKEN and TELOXIDE_PROXY are both set env variables.

        let bot = Bot::from_env();

        Command::repl(bot, answer).await;

        Ok(())
    })
}

#[allow(unused)]
async fn download() -> Result<std::fs::File, ()> {
    // attempt download

    // if we get a file, check if it fits file limits: 10mb photos, 50mb others

    // if file is in bounds, send it using Telegram API

    // otherwise, upload to some site..?
    // - https://www.keep.sh: easy https 500mb upload. no account required
    // - https://temp.sh: easy https _4gb_ upload. no account required
    // - idk probably some others. magic wormhole would be a pain but works no matter the size /shrug

    // send the user the link
    Err(())
}

#[allow(unused)]
async fn try_update_yt_dlp() {
    // check if we have yt-dlp on the system (pkg-config?)

    // if not, try to install a local copy from github using architecture
    // match arch {
    // x64 => https://github.com/yt-dlp.../x64,
    // ...
    // _ => {return Err("Couldn't get a yt-dlp package for your computer!")}
    // }

    // if we still can't get it working, return some error...
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
    };

    Ok(())
}
