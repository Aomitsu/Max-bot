use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

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
