use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::framework::StandardFramework;
use serenity::Client;
use std::env;

use regex::Regex;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use std::process::Command;

#[group]
#[prefixes("valheim")]
#[commands(ping, status, about, restart)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("could not read env file");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = env::var("BOT_TOKEN").expect("Need to supply BOT_TOKEN");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client {:?}", why)
    }
}

// fn application_status() {
//     let output = Command::new("systemctl")
//         .stdout()
//         .arg("status")
//         .arg("valheim.service")
//         .output()
//
//         .expect("could not execute application status");
//     let pattern = Regex::new(r"(?x).*Active: (?P<msg>.*)$").expect("Could not parse regex");
//     let status = String::from_utf8(output.stdout)?.lines()
//         .filter_map(|cap|pattern.captures(cap))
//         .map(|cap| cap[1]);
//
//     ()
// }

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "command under :construction:").await?;
    Ok(())
}

#[command]
async fn restart(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "command under :construction:").await?;
    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "This is a bot meant to manage the GameNight Valheim \
    server, you can use `status` to check whether the server is running and `restart` to restart \
    the server (This will also cause it to pick up any updates.)",
        )
        .await?;
    Ok(())
}
