use serenity::{
    framework::standard::{Args, CommandResult, macros::command},
    model::prelude::*,
    prelude::*,
};
use serenity::model::channel::ReactionType::Unicode;
use serenity::utils::Colour;

use crate::db::vote::*;

#[command]
#[description = "Créer un sondage"]
#[min_args(1)]
#[required_permissions(MANAGE_MESSAGES)]
async fn vote(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let _ = msg.delete(&ctx).await;

    let message = arg.message();

    let new_msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Vote !")
                    .description(message)
                    .color(Colour::new(100))
                    .footer(|f| {
                        f.text(format!(" | Lançé par {}", msg.author.name))
                            .icon_url(msg.author.avatar_url().unwrap());

                        f
                    });

                e
            })
        })
        .await.unwrap();

    let _ = new_msg.react(&ctx, { Unicode("👍".parse()?) }).await;
    let _ = new_msg.react(&ctx, { Unicode("👎".parse()?) }).await;

    let vote_db = Vote {
        message_id: new_msg.id.0,
        user_id: msg.author.id.0,
    };


    add_vote(vote_db).await;

    //notify_vote_user(msg.id.0).await;

    Ok(())
}
