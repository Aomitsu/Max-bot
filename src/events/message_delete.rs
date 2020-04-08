use serenity::model::id::*;
use serenity::prelude::*;

use crate::db::vote::*;

pub async fn message_delete(ctx: Context, _channel: ChannelId, msg: MessageId) {
    /*if is_vote(msg.0).await.is_ok() {
        rem_vote(msg.0).await
    }*/
}