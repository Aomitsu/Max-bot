use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Faire dire quelque chose au bot."]
#[min_args(1)]
fn say(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let _ = msg.delete(&ctx);
    let mut result = "error";

    let said = arg.message();
    let perms = msg
        .channel(&ctx)
        .unwrap()
        .guild()
        .unwrap()
        .read()
        .clone()
        .permission_overwrites;

    let _ = msg.channel_id.say(
        &ctx.http,
        str::replace(said, "everyone", "_everyone").replace("here", "_here"),
    );
    // TODO: Please HELP ME : How to check if the author has the permission " Mention everyone "

    Ok(())
}
