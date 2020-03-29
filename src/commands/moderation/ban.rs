use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::{
    prelude::*
};
use serenity::prelude::*;

#[command]
#[description = "Faire dire quelque chose au bot."]
#[min_args(1)]
fn ban(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let _ = msg.delete(&ctx);
    let mention = msg.mentions.get(0).unwrap();
    msg.guild_id.unwrap().ban(&ctx, mention, &(0, "test"));

    //let _ = msg.channel_id.say(&ctx.http, result);

    Ok(())
}