use serenity::{model::prelude::*, prelude::*};

pub mod ready;
pub mod resume;
pub mod guild_create;

pub struct Handler;

impl EventHandler for Handler {
    fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        guild_create::guild_create(ctx, guild, _is_new)
    }
    fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready)
    }
    fn resume(&self, ctx: Context, resume: ResumedEvent) {
        resume::resume(ctx, resume)
    }
}