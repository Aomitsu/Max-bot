use serenity::{model::prelude::*, prelude::*};
use crate::db::antispam::*;
use crate::db::guild::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::borrow::Borrow;


pub async fn message(ctx: Context, msg: Message) {
    gld_init(msg.guild_id.unwrap().0).await;
    let guild = gld_get(msg.guild_id.unwrap().0).await.unwrap();

    if msg.author.bot && msg.webhook_id.is_none() { return; }
    if !guild.anti_spam { return; }
    if guild.anti_spam_exclude.contains(msg.channel_id.0.borrow()) { return; }


    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");


    match as_get(msg.author.id.0).await {
        Ok(mut asdb) => {
            if since_the_epoch.as_secs() - asdb.timestamp  < 10 {
                if asdb.alert > 5 {
                    let _ = msg.delete(&ctx).await;
                    asdb.timestamp = since_the_epoch.as_secs();
                }
                if msg.content.contains("@") || msg.content.contains("www") || msg.content.contains("http"){
                    asdb.alert = asdb.alert + 2;
                }
                if asdb.latest_message == msg.content {
                    asdb.alert = asdb.alert + 2;
                } else {
                    asdb.alert = asdb.alert + 1;
                }
                if asdb.alert > 10 {
                    if msg.webhook_id.is_some(){
                        let _ = msg.webhook_id.unwrap().to_webhook(&ctx).await.unwrap().delete(&ctx).await;
                    }

                    let _ = msg.guild(&ctx).await
                        .unwrap()
                        .kick_with_reason(&ctx, msg.clone().author, "AntiSpam")
                        .await;

                    for (_x, m) in ctx.http.get_messages(msg.channel_id.0, "?limit=25").await.unwrap().iter().enumerate() {
                        if m.author.id.0 == msg.clone().author.id.0 {
                            let _ = m.delete(&ctx).await;
                        }
                    };
                }
            } else {
                asdb.timestamp = since_the_epoch.as_secs();
                asdb.alert = 0;
            }
            as_set(asdb).await

        }
        Err(_e) => {
            let as_member = AntiSpamDB{
                uid: msg.author.id.0,
                timestamp: since_the_epoch.as_secs(),
                alert: 0,
                latest_message: msg.content.to_string()
            };
            as_set(as_member).await
        }
    }

}