use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::{
    prelude::*
};
use serenity::prelude::*;

#[command]
#[description = "Faire dire quelque chose au bot."]
#[min_args(1)]
fn ban(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {

    Ok(())
}