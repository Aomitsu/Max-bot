use serenity::{
    framework::standard::{Args, CommandResult, macros::command},
    model::prelude::*,
    prelude::*,
};

use log::info;

use crate::db::case::*;
use crate::functions::{
    id::*,
    base_messages::*
};

#[command]
#[description = "Avertir un utilisateur."]
#[min_args(2)]
#[required_permissions(MANAGE_MESSAGES)]
async fn warn(ctx: &mut Context, msg: &Message, mut arg: Args) -> CommandResult {

    let mut errored = 0;

    let mention_id = str_to_uid(Box::from(arg.current().unwrap())).await;
    let user = ctx.http.get_member(
        msg.guild_id.unwrap().0,
        mention_id.parse()?).await;

    match user {
        Err(_0) => {
            // User inconnu erreur
            error_user(ctx, msg, mention_id.parse()?).await;
            errored = 1;

        },
        _ => {}
    }
    if errored != 1 {
        info!("{}", mention_id);

        arg.advance();

        let message: Option<String> = Some(arg.rest().to_owned());
        let case_type: Option<String> = Some("warn".to_owned());

        let case = Case{
            case_type,
            guild_id: msg.guild_id.unwrap().0,
            user_id: mention_id.parse()?,
            message : message.clone(),
        };

        add_case(case).await;

        //@TODO : MP user warned

        let member = ctx.http.get_user(mention_id.parse()?).await.unwrap();

        let _ = msg.channel_id.send_message(&ctx, |m| m.embed(
            |f|
            f.title("Warn")
                .description("Un warn a été enregistré.\n\
                Le staff du bot ne peux pas retirer de warn, sauf en cas de spam.")
                .field("Membre", member.clone().name, true)
                .field("ID du membre", member.clone().id.0, true)
                .field("Raison du warn", message.clone().unwrap(), false)
        )).await;
    }

    Ok(())
}