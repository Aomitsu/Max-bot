use serenity::{
    framework::standard::{Args, CommandResult, macros::command},
    model::prelude::*,
    prelude::*,
};

#[command]
#[description = "Réponds avec les ms du bot."]
async fn ping(ctx: &mut Context, msg: &Message, _arg: Args) -> CommandResult {
    let mut pingting = msg.channel_id.send_message(&ctx, |f| {
        f.content("Pingting...");
        f
    }).await.unwrap();


    let ms = pingting.timestamp.timestamp_millis() - msg.timestamp.timestamp_millis();

    let _ = pingting.edit(&ctx, |f| {

        f.content("")
            .embed(|e| {
            e.title("Ping")
                .description(format!("Pong ! {} ms", ms));
            e
        });

        f
    }).await;

    Ok(())
}