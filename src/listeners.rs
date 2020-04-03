use serenity::{model::prelude::*, prelude::*};

pub mod guild_create;
pub mod message;
pub mod ready;
pub mod resume;

pub struct Handler;

impl EventHandler for Handler {
    fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        guild_create::guild_create(ctx, guild, _is_new)
    }
    fn message(&self, _ctx: Context, _new_message: Message) {
        message::message(_ctx, _new_message)
    }
    fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready)
    }
    fn resume(&self, ctx: Context, resume: ResumedEvent) {
        resume::resume(ctx, resume)
    }
}
