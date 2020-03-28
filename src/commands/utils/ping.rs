use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Répond pong."]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = {
        let mut messagebot = msg.channel_id.say(&ctx.http, "> Pingting... !").unwrap();
        let ms = messagebot.timestamp - msg.timestamp;

        messagebot.edit(&ctx, |m| {
            m.content(format!("> **Pong :ping_pong: !**\n\
            > {} ms ( Messages )", ms.num_milliseconds()))
        })
    };

    Ok(())
}