mod commands;
mod constants;
mod types;
mod util;

use serenity::{async_trait, framework::{standard::macros::group, StandardFramework}, model::prelude::{Activity, Ready}, prelude::*};

use commands::{about::*, help::*, validity::*};

use std::env;

#[group]
#[commands(about, help, validity)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
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
