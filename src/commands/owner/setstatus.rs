use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[owners_only]
#[min_args(1)]
fn setstatus(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let _ = {
        let _ = msg.delete(&ctx);
        let _ = msg.channel_id.say(&ctx.http, "Je change mon status !");


        let activity = Activity::listening(arg.message());
        let status = OnlineStatus::Online;

        let _ = ctx.set_presence(Some(activity), status);
    };

    Ok(())
}
