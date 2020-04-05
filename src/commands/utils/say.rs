use std::borrow::Borrow;

use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::permissions::Permissions;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::functions::*;

#[command]
#[description = "Faire dire quelque chose au bot."]
#[min_args(1)]
fn say(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let _ = msg.delete(&ctx);
    let mut result = "error";

    let said = arg.message();
    let hasperm = has_permission(
        ctx,
        msg.member(&ctx).unwrap(),
        msg.guild(&ctx).unwrap().read().to_owned(),
        Permissions { bits: 8 },
    );

    if hasperm.contains(&true) {
        let _ = msg.channel_id.say(&ctx.http, said);
    } else {
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.description(str::replace(said, "everyone", "_everyone").replace("here", "_here"));

                e
            })
        });
    }

    // TODO: Please HELP ME : How to check if the author has the permission " Mention everyone "

    Ok(())
}
