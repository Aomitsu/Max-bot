use serenity::{
    framework::standard::{Args, CommandResult, macros::command},
    model::prelude::*,
    prelude::*,
};
use std::ptr::null;

#[command]
#[description = "Options membre"]
async fn moption(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {



    Ok(())
}