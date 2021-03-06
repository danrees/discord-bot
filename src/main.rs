use serenity::client::{EventHandler,Context};
use serenity::async_trait;
use serenity::framework::standard::{macros::{
    command,
    group
}, CommandResult};
use std::env;
use serenity::framework::StandardFramework;
use serenity::Client;

use serenity::model::channel::Message;

#[group]
#[commands(ping,status)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler{}

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