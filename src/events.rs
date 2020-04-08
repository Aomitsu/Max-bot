use serenity::{async_trait, model::prelude::*, prelude::*};

pub struct Handler;

pub mod ready;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await
    }
}
