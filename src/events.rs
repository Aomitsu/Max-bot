use serenity::{async_trait, model::prelude::*, prelude::*};

pub struct Handler;

pub mod ready;
pub mod message_delete;

#[async_trait]
impl EventHandler for Handler {
    async fn message_delete(&self, ctx: Context, channel: ChannelId, message_id: MessageId) {
        message_delete::message_delete(ctx, channel, message_id).await
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await
    }
}
