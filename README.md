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
$ sudo systemctl edit rover.service
[Service]
Environment="DISCORD_TOKEN=<Enter Discord token here>"
Environment="ROUTINATOR_HOST=<Enter Routinator host here>"
:x
$ sudo systemctl enable --now
```
