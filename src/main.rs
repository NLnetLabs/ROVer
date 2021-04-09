mod constants;
mod validity;

use serenity::{
    async_trait,
    framework::{
        standard::{
            macros::{command, group},
            Args, CommandResult,
        },
        StandardFramework,
    },
    model::channel::Message,
    prelude::*,
};

use std::env;

use crate::{
    constants::{APP_VERSION, LONGEST_EXPECTED_ASN, LONGEST_EXPECTED_PREFIX},
    validity::{AddressOrigin, ValidityResponse},
};

#[group]
#[commands(about, help, validity)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Missing environment variable DISCORD_TOKEN");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let about = format!(
        r#"
ROVer - an open source bot by NLnet Labs for in-chat feedback about the state of the RPKI.

Bot Version: {version}
Service URI: {service_base_uri}

See https://github.com/NLnetLabs/ROVer for more information.
"#,
        version = APP_VERSION,
        service_base_uri = service_base_uri()
    );

    msg.reply(ctx, format!("```{}```", about)).await?;

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let help_text = if let Ok(command_name) = args.single::<String>() {
        match &command_name[..] {
            "about" => r#"
!about

Learn more about this bot.
"#
            .to_string(),
            "help" => r#"
!help [<COMMAND>]

Get help using this bot.
"#
            .to_string(),
            "validity" => r#"
!validity <AS NUMBER> <PREFIX>

Describes whether the route announcement given by its origin AS number and
address prefix is RPKI valid, invalid, or not found.

<AS NUMBER> can optionally be prefixed by 'AS' (case insensitive).
"#
            .to_string(),
            cmd => {
                format!("Unknown command '{}'", cmd)
            }
        }
    } else {
        r#"
Commands:
  !about     About this bot
  !help      Shows this message
  !validity  Describes the RPKI validity of a route announcement

Type !help command for more info on a command.
"#
        .to_string()
    };

    msg.reply(ctx, format!("```{}```", help_text)).await?;

    Ok(())
}

#[command]
async fn validity(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let as_number_res = args.single::<String>();
    let prefix_res = args.single::<String>();

    let report = match (as_number_res, prefix_res) {
        (Err(err), _) => {
            format!("Invalid AS number: {}", err)
        }
        (_, Err(err)) => {
            format!("Invalid prefix: {}", err)
        }
        (Ok(as_number), Ok(prefix)) => {
            let as_number = if as_number[0..2].to_lowercase() == "as" {
                as_number[2..].to_string()
            } else {
                as_number
            };

            let query_url = format!("{}/api/v1/validity/AS{}/{}", service_base_uri(), as_number, prefix);

            match ureq::get(&query_url).call() {
                Err(ureq::Error::Status(400, _)) => "Validity check failed: Invalid AS number or prefix".to_string(),
                Err(ureq::Error::Status(code, _)) => {
                    format!("Validity check failed: Status code {}", code)
                }
                Err(_) => "Validity check failed: Unable to contact the service".to_string(),
                Ok(res) => {
                    let json_res = res.into_json();

                    match json_res {
                        Err(err) => {
                            format!("Validity check failed: Bad response: {}", err)
                        }
                        Ok(json) => build_validity_report(json),
                    }
                }
            }
        }
    };

    msg.reply(ctx, format!("```{}```", report)).await?;

    Ok(())
}

fn service_base_uri() -> String {
    let host = env::var("ROUTINATOR_HOST").expect("Missing environment variable ROUTINATOR_HOST");
    format!("https://{}", host)
}

fn build_validity_report(json: ValidityResponse) -> String {
    let mut report = String::new();
    report.push_str(&format!(
        "Results for {asn} - {prefix}: {state}\n",
        asn = json.validated_route.route.origin_asn,
        prefix = json.validated_route.route.prefix,
        state = json.validated_route.validity.state.to_uppercase()
    ));
    report.push_str(&format!("{}\n", json.validated_route.validity.description));

    if !json.validated_route.validity.vrps.matched.is_empty() {
        report.push('\n');
        report.push_str("Matched VRPs\n");
        report.push_str(&address_origins_to_string(&json.validated_route.validity.vrps.matched));
    }

    if !json.validated_route.validity.vrps.unmatched_as.is_empty() {
        report.push('\n');
        report.push_str("Unmatched VRPs - ASN\n");
        report.push_str(&address_origins_to_string(
            &json.validated_route.validity.vrps.unmatched_as,
        ));
    }

    if !json.validated_route.validity.vrps.unmatched_length.is_empty() {
        report.push('\n');
        report.push_str("Unmatched VRPs - Length\n");
        report.push_str(&address_origins_to_string(
            &json.validated_route.validity.vrps.unmatched_length,
        ));
    }

    report
}

fn address_origins_to_string(aos: &[AddressOrigin]) -> String {
    let mut builder = String::new();

    builder.push_str(&format!(
        "{asn:asn_width$} {prefix:prefix_width$} {max_length}\n",
        asn = "ASN",
        asn_width = LONGEST_EXPECTED_ASN,
        prefix = "Prefix",
        prefix_width = LONGEST_EXPECTED_PREFIX,
        max_length = "Max Length"
    ));
    for ao in aos {
        builder.push_str(&format!(
            "{asn:asn_width$} {prefix:prefix_width$} {max_length}\n",
            asn = ao.asn,
            asn_width = LONGEST_EXPECTED_ASN,
            prefix = ao.prefix,
            prefix_width = LONGEST_EXPECTED_PREFIX,
            max_length = ao.max_length
        ));
    }

    builder
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_json_parsing() {
        use crate::validity::ValidityResponse;

        let routinator_validity_http_endpoint_output_sample = r#"
{
  "validated_route": {
    "route": {
      "origin_asn": "AS43996",
      "prefix": "5.57.16.0/24"
    },
    "validity": {
      "state": "valid",
      "description": "At least one VRP Matches the Route Prefix",
      "VRPs": {
        "matched": [
          {
            "asn": "AS43996",
            "prefix": "5.57.16.0/24",
            "max_length": "24"
          }

        ],
        "unmatched_as": [
          {
            "asn": "AS26415",
            "prefix": "5.57.16.0/24",
            "max_length": "24"
          }
,
          {
            "asn": "AS19905",
            "prefix": "5.57.16.0/24",
            "max_length": "24"
          }

        ],
        "unmatched_length": [
          {
            "asn": "AS43996",
            "prefix": "5.57.16.0/21",
            "max_length": "21"
          }
,
          {
            "asn": "AS43996",
            "prefix": "5.57.16.0/22",
            "max_length": "22"
          }

        ]      }
    }
  }
}
"#;

        let json: ValidityResponse = serde_json::from_str(routinator_validity_http_endpoint_output_sample).unwrap();

        assert_eq!(&json.validated_route.route.origin_asn, "AS43996");
        assert_eq!(&json.validated_route.route.prefix, "5.57.16.0/24");

        assert_eq!(&json.validated_route.validity.state, "valid");
        assert_eq!(
            &json.validated_route.validity.description,
            "At least one VRP Matches the Route Prefix"
        );

        assert_eq!(json.validated_route.validity.vrps.matched.len(), 1);
        assert_eq!(&json.validated_route.validity.vrps.matched[0].asn, "AS43996");
        assert_eq!(&json.validated_route.validity.vrps.matched[0].prefix, "5.57.16.0/24");
        assert_eq!(&json.validated_route.validity.vrps.matched[0].max_length, "24");

        assert_eq!(json.validated_route.validity.vrps.unmatched_as.len(), 2);
        assert_eq!(&json.validated_route.validity.vrps.unmatched_as[0].asn, "AS26415");
        assert_eq!(
            &json.validated_route.validity.vrps.unmatched_as[0].prefix,
            "5.57.16.0/24"
        );
        assert_eq!(&json.validated_route.validity.vrps.unmatched_as[0].max_length, "24");
        assert_eq!(&json.validated_route.validity.vrps.unmatched_as[1].asn, "AS19905");
        assert_eq!(
            &json.validated_route.validity.vrps.unmatched_as[1].prefix,
            "5.57.16.0/24"
        );
        assert_eq!(&json.validated_route.validity.vrps.unmatched_as[1].max_length, "24");

        assert_eq!(json.validated_route.validity.vrps.unmatched_length.len(), 2);
        assert_eq!(&json.validated_route.validity.vrps.unmatched_length[0].asn, "AS43996");
        assert_eq!(
            &json.validated_route.validity.vrps.unmatched_length[0].prefix,
            "5.57.16.0/21"
        );
        assert_eq!(&json.validated_route.validity.vrps.unmatched_length[0].max_length, "21");
        assert_eq!(&json.validated_route.validity.vrps.unmatched_length[1].asn, "AS43996");
        assert_eq!(
            &json.validated_route.validity.vrps.unmatched_length[1].prefix,
            "5.57.16.0/22"
        );
        assert_eq!(&json.validated_route.validity.vrps.unmatched_length[1].max_length, "22");
    }
}
