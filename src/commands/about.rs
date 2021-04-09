use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::constants::APP_VERSION;
use crate::util::service_base_uri;

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
