use std::ffi::OsString;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::constants::APP_VERSION;
use crate::routinator::types::StatusResponse;
use crate::util::{http_client, service_base_uri};

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let hostname = hostname::get()
        .unwrap_or_else(|_| OsString::from("Unknown"))
        .into_string()
        .unwrap_or_else(|_| "Invalid".to_string());

    let about = format!(
        r#"
ROVer - an open source bot by NLnet Labs for in-chat feedback about the state of the RPKI.

ROVer Version  : {version}
Host FQDN      : {host_fqdn}
Service URI    : {service_base_uri}
Service Version: {service_version}

See https://github.com/NLnetLabs/ROVer for more information.
"#,
        version = APP_VERSION,
        host_fqdn = hostname,
        service_base_uri = service_base_uri(),
        service_version = get_routinator_version().unwrap_or_else(|_| "Unavailable".to_string()),
    );

    msg.reply(ctx, format!("```{}```", about)).await?;

    Ok(())
}

fn get_routinator_version() -> Result<String, String> {
    let status_url = format!("{}/api/v1/status", service_base_uri());
    match http_client().get(&status_url).call() {
        Err(ureq::Error::Status(code, _)) => Err(format!("Version check failed: Status code {}", code)),
        Err(_) => Err("Version check failed: Unable to contact the service".to_string()),
        Ok(res) => {
            let json_res: std::io::Result<StatusResponse> = res.into_json();

            match json_res {
                Err(err) => Err(format!("Version check failed: Bad response: {}", err)),
                Ok(status_json) => Ok(status_json.version),
            }
        }
    }
}
