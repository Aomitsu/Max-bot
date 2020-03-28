use serenity::{model::prelude::*, prelude::*};

pub mod ready;
pub mod resume;

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready)
    }
    fn resume(&self, ctx: Context, ready: ResumedEvent) {
        resume::resume(ctx, ready)
    }
}