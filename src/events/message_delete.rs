use serenity::model::id::*;
use serenity::prelude::*;

use crate::db::vote::*;

pub async fn message_delete(_ctx: Context, _channel: ChannelId, msg: MessageId) {

        rem_vote(msg.0).await;

}