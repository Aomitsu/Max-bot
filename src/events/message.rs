use serenity::{model::prelude::*, prelude::*};
use crate::db::antispam::*;
use std::time::{SystemTime, UNIX_EPOCH};



use log::info;

pub async fn message(ctx: Context, msg: Message) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    match as_get(msg.author.id.0).await {
        Ok(mut asdb) => {
            info!("User déjà dans la DB");
            if since_the_epoch.as_secs() - asdb.timestamp  < 10 {
                if asdb.alert > 5 {
                    let _ = msg.delete(&ctx).await;
                    asdb.timestamp = since_the_epoch.as_secs();
                }
                if msg.content.contains("@everyone") || msg.content.contains("@here"){
                    asdb.alert = asdb.alert + 2;
                }
                if asdb.latest_message == msg.content {
                    asdb.alert = asdb.alert + 2;
                } else {
                    asdb.alert = asdb.alert + 1;
                }
                if asdb.alert > 10 {
                    info!("Kick !");
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
            info!("User pas dans la DB");
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