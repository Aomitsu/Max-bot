use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[description = "Seul noah connaît les secrets de cette commande"]
fn harck(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Harck").description("Sette api ais en devve");

            e
        })
    });

    Ok(())
}
