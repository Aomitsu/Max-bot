use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Faire dire quelque chose au bot."]
#[min_args(1)]
fn say(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let _ = msg.delete(&ctx);

    let said = arg.message();
    let result = str::replace(said, "everyone", " everyone").replace("here", " here");


    let _ = msg.channel_id.say(&ctx.http, result);

    Ok(())
}