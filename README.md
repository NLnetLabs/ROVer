[![Documentation Status](https://readthedocs.org/projects/rpki/badge/?version=latest)](https://rpki.readthedocs.io/en/latest/?badge=latest)
[![](https://img.shields.io/discord/818584154278199396?label=rpki%20on%20discord&logo=discord)](https://discord.gg/8dvKB5Ykhy)

# ROVer

A simple Rust Discord bot for rendering the output of the [Routinator](https://nlnetlabs.nl/projects/rpki/routinator/) validity HTTP API endpoint as a textual table:

For example given the command `!validity AS43996 5.57.16.0/24` the bot prints:

```
Results for ASAS43996 - 5.57.16.0/24: VALID
At least one VRP Matches the Route Prefix

Matched VRPs
ASN       Prefix        Max Length
AS43996   5.57.16.0/24  24

Unmatched VRPs - ASN
ASN       Prefix        Max Length
AS19905   5.57.16.0/24  24
AS26415   5.57.16.0/24  24

Unmatched VRPs - Length
ASN       Prefix        Max Length
AS43996   5.57.16.0/22  22
AS43996   5.57.16.0/21  21
```

Note: The AS prefix in the AS argument is optional and case insensitive.

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

Tested with [Routinator](https://nlnetlabs.nl/projects/rpki/routinator/) v0.8.3.

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
- Create an OAuth 2 URL with scope `bot` and viist the URL as a user with 'Manage Server' right.
- The ROVer bot should show up as joining the Discord server.
- Send the bot a message, e.g. with `!help`.

# Icon credit

Thanks to [pixabay](https://pixabay.com/) for the [ROVer image](https://pixabay.com/illustrations/dog-male-animal-comic-hybrid-pet-4524609/). Pixabay states for this image that is licensed under the Pixabay License which is _"Free for commercial use"_ with _"no attribution required"_. If you think this image is actually yours and not free to use **please let us know**!
