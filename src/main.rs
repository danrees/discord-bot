use std::env;
use serenity::{
  async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    framework::standard::macros::{command,group}
};
use serenity::framework::StandardFramework;
use serenity::framework::standard::CommandResult;
use serenity::http::routing::Route::UsersId;

#[group]
#[description = "Basic functions against the valheim server"]
#[commands(valheim_status)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let bot_id = env::var("BOT_ID").expect("You need to provide the bot id");

    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .on_mention(Some(bot_id.parse().unwrap()))
            .prefix("~")
            .delimiters(vec![", ", ","])
            )
        .group(&GENERAL_GROUP);

    let token = env::var("BOT_TOKEN")
        .expect("You need to provide DISCORD_TOKEN");
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    
    if let Err(why) = client.start().await {
        println!("Client err: {:?}", why)
    }
}

#[command]
async fn valheim_status(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, ":construction:").await?;
    Ok(())
}