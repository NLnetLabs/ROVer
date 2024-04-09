[![Documentation Status](https://readthedocs.org/projects/rpki/badge/?version=latest)](https://rpki.readthedocs.io/en/latest/?badge=latest)
[![](https://img.shields.io/discord/818584154278199396?label=rpki%20on%20discord&logo=discord)](https://discord.gg/8dvKB5Ykhy)

# ROVer

A simple Rust Discord bot for rendering the output of the [Routinator](https://nlnetlabs.nl/projects/rpki/routinator/) validity HTTP API endpoint as a textual table:

For example given the command `!validity AS16509 185.49.143.0/24` the bot prints:

![image](https://user-images.githubusercontent.com/3304436/114357357-f0e3c200-9b71-11eb-98c0-822eeb22a99e.png)

Since v0.1.2 it also shows the AS name (powered by the [RIPEstat Data API](https://stat.ripe.net/docs/data_api)):

![image](https://user-images.githubusercontent.com/3304436/114722410-128fa580-9d3a-11eb-9f35-eeba4eeace00.png)

Note: The AS prefix in the AS argument is optional and case insensitive.

# Requirements

- A Discord account with the right to create an application and bot.
- Discord 'Manage Server' permission on the Discord server to which the bot should be invited.
- Rust 1.51.0 _(might actually work with older versions but currently only tested with current Rust stable which is 1.51.0 at the time of writing)_
- Routinator 0.9.0

# Building

```
$ cargo build --release --locked
```

# Running

```
$ export DISCORD_TOKEN=xxx
$ export ROUTINATOR_HOST=some.fqdn.com
$ ./target/release/rover
```

# Using the Debian package

```
$ sudo apt install -y ./path/to/rover.deb
$ export EDITOR=$(which vi)
$ sudo systemctl edit rover
[Service]
Environment="DISCORD_TOKEN=<Enter Discord token here>"
Environment="ROUTINATOR_HOST=<Enter Routinator host here>"
:x
$ sudo systemctl enable --now rover
```

# Using the bot

- Create a Discord application and add a bot. See [the official help](https://discord.com/developers/docs/topics/oauth2#bots).
- Copy the bot token and use that as the `DISCORD_TOKEN` environment variable.
- Run the Rust bot either with `cargo run` or as a systemd service (if using the Debian package).
- Create an OAuth 2 URL with scope `bot` and visit the URL as a user with 'Manage Server' right.
- The ROVer bot should show up as joining the Discord server.
- Send the bot a message, e.g. with `!help`.

# Icon credit

Thanks to [pixabay](https://pixabay.com/) for the [ROVer image](https://pixabay.com/illustrations/dog-male-animal-comic-hybrid-pet-4524609/). Pixabay states for this image that is licensed under the Pixabay License which is _"Free for commercial use"_ with _"no attribution required"_. If you think this image is actually yours and not free to use **please let us know**!
