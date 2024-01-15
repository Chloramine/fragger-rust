use serenity::{all::*, framework::standard::{CommandResult, macros::command}};

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong").await?;
    Ok(())
}