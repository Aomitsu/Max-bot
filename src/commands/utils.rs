use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Répond pong."]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong :ping_pong: !");

    Ok(())
}


#[command]
#[description = "Faire dire quelque chose au bot."]
fn say(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let _ = msg.delete(&ctx);

    let said = arg.message();
    let _ = msg.channel_id.say(&ctx.http, said);

    Ok(())
}