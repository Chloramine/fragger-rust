use std::env;
use serenity::{all::*, framework::standard::{Configuration, macros::group}};
mod files;
mod commands;

use crate::commands::misc::ping::*;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("running fragger on {}", ready.user.name);
    }
}

#[group]
#[commands(ping)]
struct General;


#[tokio::main]
async fn main() {
    let token = files::info::token(); // store token in env variable so as to not post it to github.
    let intents = GatewayIntents::all();
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(
        Configuration::new().with_whitespace(true)
            .prefix(files::info::prefix())); // todo: incorporate info::ADMINS into .owners()
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("error creating client");
    if let Err(why) = client.start().await {
        println!("error occurred: {:?}", why);
    }
}