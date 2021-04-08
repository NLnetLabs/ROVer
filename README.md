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
$ ./ROVer
```

Tested with [Routinator](https://nlnetlabs.nl/projects/rpki/routinator/) v0.8.3.