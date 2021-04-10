use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

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
