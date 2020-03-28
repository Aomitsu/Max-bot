use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Répond pong."]
fn addemote(ctx: &mut Context, msg: &Message) -> CommandResult {
    Ok(())
}