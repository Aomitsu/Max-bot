use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[description = "Et les taches s'évanouissent"]
fn vanish(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Vanish")
                .description("Et les taches s'évanouissent.")
                .image("https://img.tesco.com/Groceries/pi/470/5011417570470/IDShot_540x540.jpg");

            e
        })
    });

    Ok(())
}
