use std::env;
use serenity::all::*;
mod info;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("running fragger on {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN") //let token = info::token(); // store token in env variable so as to not post it to github, otherwise use this commented code.
        .expect("expected a token in the environment."); 
    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("error creating client");
    if let Err(why) = client.start().await {
        println!("error occurred: {:?}", why);
    }
}