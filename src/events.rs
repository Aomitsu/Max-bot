use serenity::{async_trait, model::prelude::*, prelude::*};

pub struct Handler;

pub mod ready;
pub mod message;
pub mod member_add;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, new_member: Member) {
        member_add::guild_member_addition(ctx, guild_id, new_member).await
    }
    async fn message(&self, ctx: Context, msg: Message) {
        message::message(ctx, msg).await
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await
    }
}
