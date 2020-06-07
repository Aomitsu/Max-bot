use serenity::{async_trait, model::prelude::*, prelude::*};

pub struct Handler;

pub mod ready;
pub mod message;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        message::message(ctx, msg).await
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await
    }
}
