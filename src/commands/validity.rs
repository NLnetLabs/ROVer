use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::{
    ripestat::types::AsOverviewResponse,
    routinator::types::{AddressOrigin, StatusResponse, ValidityResponse},
    util::{http_client, service_base_uri},
};

use crate::constants::{LONGEST_EXPECTED_ASN, LONGEST_EXPECTED_PREFIX};

// TODO: Simplify this with the ? operator
// TODO: Work out what Serenity does with returned errors
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

            match get_last_update_done_at() {
                Err(err) => err,
                Ok(last_update) => match get_validity_report(&as_number, &prefix) {
                    Err(err) => err,
                    Ok(validity_report) => {
                        let as_name = get_as_holder(&as_number).unwrap_or(None);
                        render_validity_report(validity_report, as_name, last_update)
                    }
                },
            }
        }
    };

    msg.reply(ctx, format!("```{}```", report)).await?;

    Ok(())
}

/// Fetch the Routinator validity report for the specified AS and prefix.
fn get_validity_report(as_number: &str, prefix: &str) -> Result<ValidityResponse, String> {
    let validity_url = format!("{}/api/v1/validity/AS{}/{}", service_base_uri(), as_number, prefix);
    match http_client().get(&validity_url).call() {
        Err(ureq::Error::Status(400, _)) => Err("Validity check failed: Invalid AS number or prefix".to_string()),
        Err(ureq::Error::Status(code, _)) => Err(format!("Validity check failed: Status code {}", code)),
        Err(_) => Err("Validity check failed: Unable to contact the service".to_string()),
        Ok(res) => {
            let json_res = res.into_json();

            match json_res {
                Err(err) => Err(format!("Validity check failed: Bad response: {}", err)),
                Ok(validity_json) => Ok(validity_json),
            }
        }
    }
}

/// Fetch the timestamp at which Routinator last completed an update of its data
fn get_last_update_done_at() -> Result<String, String> {
    let status_url = format!("{}/api/v1/status", service_base_uri());
    match http_client().get(&status_url).call() {
        Err(ureq::Error::Status(code, _)) => Err(format!("Status check failed: Status code {}", code)),
        Err(_) => Err("Status check failed: Unable to contact the service".to_string()),
        Ok(res) => {
            let json_res: std::io::Result<StatusResponse> = res.into_json();

            match json_res {
                Err(err) => Err(format!("Status check failed: Bad response: {}", err)),
                Ok(status_json) => Ok(status_json.last_update_done),
            }
        }
    }
}

/// Fetch the name of an AS holder using the RIPEstat service
fn get_as_holder(as_number: &str) -> Result<Option<String>, String> {
    let as_overview_url = format!(
        "https://stat.ripe.net/data/as-overview/data.json?resource=AS{}&sourceapp=ROVer",
        as_number
    );
    match http_client().get(&as_overview_url).call() {
        Err(ureq::Error::Status(code, _)) => Err(format!("RIPEstat AS Overview failed: Status code {}", code)),
        Err(_) => Err("RIPEstat AS Overview failed: Unable to contact the service".to_string()),
        Ok(res) => {
            let json_res: std::io::Result<AsOverviewResponse> = res.into_json();

            match json_res {
                Err(err) => Err(format!("RIPEstat AS Overview failed: Bad response: {}", err)),
                Ok(json) => {
                    if json.status == "ok" {
                        Ok(json.data.holder)
                    } else {
                        Err(format!(
                            "RIPEstat AS Overview failed: Bad response status: {}",
                            json.status
                        ))
                    }
                }
            }
        }
    }
}

/// Pretty print the given validity report for the given AS name based on Routinator data last updated as specified.
fn render_validity_report(json: ValidityResponse, as_name_opt: Option<String>, last_update: String) -> String {
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

    report.push('\n');
    if let Some(as_name) = as_name_opt {
        report.push_str(&format!("AS Name: {}\n", as_name));
    }
    report.push_str(&format!("Validation last updated at: {}", last_update));

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
