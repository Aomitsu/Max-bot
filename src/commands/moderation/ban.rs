use serenity::{framework::standard::{Args, CommandResult, macros::command}, model::prelude::*, prelude::*};

use crate::db::case::*;
use crate::functions::{
    id::*,
    base_messages::*
};


// TODO: Optimiser cette... merde
#[command]
#[description = "Avertir un utilisateur."]
#[min_args(1)]
#[required_permissions(MANAGE_MESSAGES)]
async fn ban(ctx: &mut Context, msg: &Message, mut arg: Args) -> CommandResult {
    // On obtiens l'id de la personne a bannir + on vérifie si il est sur le serveur
    let mention_id = str_to_uid(Box::from(arg.current().unwrap())).await;
    let user = ctx.http.get_member(
        msg.guild_id.unwrap().0,
        mention_id.parse()?).await;
    match user {
        Ok(u) => {
            arg.advance();
            let reason = arg.rest();

            let ban_reason = format!("MaxBot | {} | Par {}",
                                     reason.clone(),
                                     msg.author.tag());

            let _ = u.ban(&ctx, &(0, ban_reason)).await;
            user_ban(ctx, u, msg.channel_id, reason.parse()?).await;
        }
        Err(_0) => {
            // User pas sur le serveur
            error_user(ctx, msg, mention_id.parse()?).await;
        },
    }


    Ok(())
}

async fn user_ban(ctx: &mut Context, u: Member, chan: ChannelId, reason: String) {


    let banned = ctx.http.get_member(
        u.guild_id.0,
        u.user_id().await.0).await;
    match banned {
        Ok(_u) => {
            // Membre pas ban
            not_banned_message(ctx, chan).await;
        },
        Err(_0) => {
            banned_message(ctx, u, chan, reason.clone()).await;
        }
    }
}
async fn banned_message(ctx: &mut Context, u: Member, chan: ChannelId, reason: String){

    let _ = chan.send_message(
        &ctx,
        |m| m.embed(|e|{
            e.title("Bann")
                .field("Banni", "L'utilisateur a bien été banni.", true)
                .field("Raison", format!("MaxBot : {}", reason.clone()), true)
        })
    ).await;

    let case = Case{
        case_type: Some("ban".parse().unwrap()),
        guild_id: u.guild_id.0,
        user_id: u.user_id().await.0,
        message: Some(reason.clone())
    };

    add_case(case).await;
}
async fn not_banned_message(ctx: &mut Context, chan: ChannelId){

    let _ = chan.send_message(
        &ctx,
        |m| m.embed(|e|{
            e.title("Non banni !")
                .description("Cet utilisateur n'as pas été banni.")
        })
    ).await;
}