use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Donne des infos sur le serveur."]
fn guildinfo(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_cached = msg.guild_id.unwrap().to_guild_cached(&ctx);
    let guild_info = guild_cached.unwrap();

    let no_icon = "https://www.boilersupplies.com/img/no_image.png";

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Informations sur ce serveur")
                .description("Les informations données ici sont limité : Je n'ai pas encore compris comment avoir toute les infos d'une guild avec la lib que j'utilise :/")
                .thumbnail(guild_info.read().icon_url().unwrap_or(no_icon.parse().unwrap()))
                .field("-- **Base + Stats** --",
                       format!("**Nom :** {}\n\
                       **ID :** {}\n\
                       **Membres :** {}\n\
                       **Bans :** {}\n\
                       **Rôles :** {}\n\
                       [`Lien de l'icône`]({})",
                               guild_info.read().name.as_str(),
                               guild_info.read().id.0,
                               guild_info.read().members.len(),
                               guild_info.read().bans(&ctx).unwrap().len(),
                               guild_info.read().roles.len(),
                               guild_info.read().icon_url().unwrap_or(no_icon.parse().unwrap())),
                       true);


            e
        })
    });

    Ok(())
}