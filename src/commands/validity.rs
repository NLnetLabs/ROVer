use serenity::{client::Context, framework::standard::{Args, CommandResult, macros::command}, model::channel::Message};

use crate::{types::{AddressOrigin, ValidityResponse}, util::service_base_uri};

use crate::constants::{LONGEST_EXPECTED_ASN, LONGEST_EXPECTED_PREFIX};

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
