# dlp-fetch-bot

A Telegram bot who devotedly downloads the links you send it using `yt-dlp`!

## Installation

`dlp-fetch-bot` runs directly out of a terminal. To setup the bot, you can follow the quick start...

### Quick Start

First, ensure that your system has the necessary dependencies (yt-dlp, OpenSSH, etc.). Then...

1. Download a build for your computer [from GitHub Actions](https://github.com/onkoe/dlp-fetch-bot/actions)
2. Drag it somewhere on your $PATH (maybe `~.local/bin`?)
3. Set your Teloxide environment variables:
  a. [TELOXIDE_TOKEN](https://core.telegram.org/bots/api#authorizing-your-bot) - a bot token obtained from @BotFather
  b. [TELOXIDE_PROXY](https://docs.rs/teloxide/latest/teloxide/struct.Bot.html#method.from_env) - (optional) a proxy link, either HTTP or HTTPS. Please use the `https://user:pass@example.com` format for your proxy.
4. Launch it! *Grab your fire extinguisher*, then run `dlp-fetch-bot` in a terminal! 🧯️

### Downloads

If you're looking for a download for your system, you can either compile it yourself, or use a link below: **TODO**

- Windows: [x64]() / [x86]() / [aarch64]()
- macOS: [Intel]() / [ARM]()
- Linux: [x64]() / [x86]() / [aarch64]() / [rv64]()

## Contributions

If you'd like to fix problems or add new functionality to the bot, please follow [the contributors guide](./CONTRIBUTORS.md)!

## Configuration

There are currently no configuration values for the bot.

## License

This project is licensed under the WTFPL. See the [LICENSE file](./LICENSE) for details. Speaking on warranty, you should assume the following statement holds true for all source code files included here:

```
This program is free software. It comes without any warranty, to
the extent permitted by applicable law. You can redistribute it
and/or modify it under the terms of the Do What The Fuck You Want
To Public License, Version 2, as published by Sam Hocevar. See
<http://www.wtfpl.net/> for more details.
```
