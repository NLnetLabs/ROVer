mod commands;
mod constants;
mod routinator {
    pub mod types;
}
mod ripestat {
    pub mod types;
}
mod util;

use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework, Framework},
    model::{
        channel::Message,
        prelude::{Activity, Ready}, event::MessageUpdateEvent,
    },
    prelude::*, utils::CustomMessage,
};

use serenity::model::guild::GuildStatus;

use commands::{about::*, help::*, validity::*};

use std::{env, sync::Arc};

#[group]
#[commands(about, help, validity)]
struct General;

struct Handler {
    framework: Arc<Box<(dyn Framework + Sync + 'static + Send)>>,
}

impl Handler {
    fn new(framework: Arc<Box<(dyn Framework + Sync + 'static + Send)>>) -> Self { Self { framework } }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Ready, listing joined guilds:");
        for guild_status in &ready.guilds {
            match guild_status {
                GuildStatus::OnlinePartialGuild(p) => println!("  OnlinePartialGuild: name={}", p.name),
                GuildStatus::OnlineGuild(g) => println!("  OnlineGuild: name={}", g.name),
                GuildStatus::Offline(u) => match ctx.http.get_guild(*u.id.as_u64()).await {
                    Ok(p) => println!("  OfflineGuild: name={}", p.name),
                    Err(_) => println!("  OfflineGuild: id={}", u.id),
                },
                _ => println!("  UnknownGuildStatus"),
            }
        }

        ctx.set_activity(Activity::playing("!help")).await;
    }

    /// When a Discord user edits a previous message, dispatch the edited message so that
    /// the appropriate command handler handles it, as otherwise edited messages are just
    /// ignored.
    /// See also: https://github.com/serenity-rs/serenity/issues/1229
    async fn message_update(
        &self,
        ctx: Context,
        _old_if_available: Option<Message>,
        _new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        // In my testing the old and new message are always None so I extract the required
        // data from the raw event instead.
        if let (Some(ts), Some(guild_id), Some(author), Some(content)) = (event.edited_timestamp, event.guild_id, event.author, event.content) {
            let mut msg = CustomMessage::new();
            msg.timestamp(ts);
            msg.id(event.id);
            msg.channel_id(event.channel_id);
            msg.guild_id(guild_id);
            msg.author(author);
            msg.content(content);
            self.framework.dispatch(ctx, msg.build()).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);

    let framework: Arc<Box<(dyn Framework + Sync + 'static + Send)>> = Arc::new(Box::new(framework));

    let handler = Handler::new(framework.clone());

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Missing environment variable DISCORD_TOKEN");
    let mut client = Client::builder(token)
        .event_handler(handler)
        .framework_arc(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
