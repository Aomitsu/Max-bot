use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::ShardManagerContainer;

#[command]
#[owners_only]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        manager.lock().shutdown_all();
    } else {
        let _ = msg.reply(&ctx, "There was a problem getting the shard manager");

        return Ok(());
    }

    let _ = msg.reply(&ctx, "Shutting down!");

    Ok(())
}

#[command]
#[owners_only]
#[min_args(2)]
fn admsay(ctx: &mut Context, msg: &Message, mut arg: Args) -> CommandResult {
    let _ = {
        let _ = msg.delete(&ctx);
        let chan = ctx.http.get_channel(arg.clone().current().unwrap().parse()?).unwrap().id();
        arg.advance();
        let _say = chan.say(&ctx.http, arg.clone().rest());
    };

    Ok(())
}

#[command]
#[owners_only]
#[min_args(1)]
fn setstatus(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let _ = {
        let _ = msg.delete(&ctx);
        let _ = msg.channel_id.say(&ctx.http, "Je change mon status !");

        let activity = Activity::streaming(arg.message(), "https://www.twitch.tv/zerator");
        let status = OnlineStatus::DoNotDisturb;

        let _ = ctx.set_presence(Some(activity), status);
    };

    Ok(())
}
