use serenity::{
    framework::standard::{Args, CommandResult, macros::command},
    model::prelude::*,
    prelude::*,
};

#[command]
#[owners_only]
#[min_args(2)]
async fn admsay(ctx: &mut Context, msg: &Message, mut arg: Args) -> CommandResult {
    let _ = msg.delete(&ctx).await;

    let id = arg.current().unwrap();
    let channel = ctx.http.get_channel(id.parse()?).await;
    let channel_id = channel.unwrap().id().await;

    arg.advance();

    let _ = channel_id.say(&ctx, arg.rest()).await;

    Ok(())
}
