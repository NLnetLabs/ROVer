mod validity;

use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::{async_trait, framework::standard::Args};

use std::env;

use crate::validity::{AddressOrigin, ValidityResponse};

#[group]
#[commands(validity)]
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
async fn validity(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let as_number = args.single::<String>()?;
    let prefix = args.single::<String>()?;

    let as_number = if as_number[0..2].to_lowercase() == "as" {
        as_number[2..].to_string()
    } else {
        as_number
    };

    let routinator_host =
        env::var("ROUTINATOR_HOST").expect("Missing environment variable ROUTINATOR_HOST");

    let res: ValidityResponse = ureq::get(&format!(
        "https://{}/api/v1/validity/AS{}/{}",
        routinator_host, as_number, prefix
    ))
    .call()?
    .into_json()?;

    fn address_origins_to_string(aos: &[AddressOrigin]) -> String {
        let mut builder = String::new();

        builder.push_str("ASN       Prefix        Max Length\n");
        for ao in aos {
            builder.push_str(&format!(
                "{asn:9} {prefix:13} {max_length}\n",
                asn = ao.asn,
                prefix = ao.prefix,
                max_length = ao.max_length
            ));
        }

        builder
    }

    let mut report = String::new();
    report.push_str("```\n");
    report.push_str(&format!(
        "Results for AS{asn} - {prefix}: {state}\n",
        asn = res.validated_route.route.origin_asn,
        prefix = res.validated_route.route.prefix,
        state = res.validated_route.validity.state.to_uppercase()
    ));
    report.push_str(&res.validated_route.validity.description);
    report.push('\n');
    report.push('\n');
    report.push_str("Matched VRPs\n");
    report.push_str(&address_origins_to_string(
        &res.validated_route.validity.vrps.matched,
    ));
    report.push('\n');
    report.push_str("Unmatched VRPs - ASN\n");
    report.push_str(&address_origins_to_string(
        &res.validated_route.validity.vrps.unmatched_as,
    ));
    report.push('\n');
    report.push_str("Unmatched VRPs - Length\n");
    report.push_str(&address_origins_to_string(
        &res.validated_route.validity.vrps.unmatched_length,
    ));
    report.push_str("```\n");

    msg.reply(ctx, report).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ureq_json_parsing() {
        use crate::validity::ValidityResponse;

        let data = r#"
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

        let json: ValidityResponse = serde_json::from_str(data).unwrap();

        println!("{:?}", json);
    }
}
