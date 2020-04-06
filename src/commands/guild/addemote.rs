use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[owners_only]
#[description = "Répond pong."]
fn addemote(_ctx: &mut Context, _msg: &Message, _arg: Args) -> CommandResult {
    Ok(())
}
