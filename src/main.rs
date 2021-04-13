mod commands;
mod constants;
mod types;
mod util;

use serenity::{async_trait, framework::{standard::macros::group, StandardFramework}, model::prelude::{Activity, Ready}, prelude::*};

use serenity::model::guild::GuildStatus;

use commands::{about::*, help::*, validity::*};

use std::env;

#[group]
#[commands(about, help, validity)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Ready, listing joined guilds:");
        for guild_status in &ready.guilds {
            match guild_status {
                GuildStatus::OnlinePartialGuild(p) => println!("  OnlinePartialGuild: name={}", p.name),
                GuildStatus::OnlineGuild(g) => println!("  OnlineGuild: name={}", g.name),
                GuildStatus::Offline(u) => {
                    match ctx.http.get_guild(*u.id.as_u64()).await {
                        Ok(p) => println!("  OfflineGuild: name={}", p.name),
                        Err(_) => println!("  OfflineGuild: id={}", u.id),
                    }
                }
                _ => println!("  UnknownGuildStatus"),
            }
        }

        ctx.set_activity(Activity::playing("!help")).await;
    }
}

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
