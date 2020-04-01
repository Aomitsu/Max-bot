use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Donne des infos sur le serveur."]
fn guildinfo(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_frmmsg = msg.guild_id.unwrap();
    let guild_frmbot = &ctx.http.get_guild(msg.guild_id.unwrap().0).unwrap();

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Informations sur ce serveur")
                .description("Les informations données ici sont limité : Je n'ai pas encore compris comment avoir toute les infos d'une guild avec la lib que j'utilise :/")
                .thumbnail(guild_frmbot.icon_url().unwrap_or("https://www.pinclipart.com/picdir/middle/1-16405_american-red-cross-computer-icons-christian-cross-symbol.png".parse().unwrap()))
                .field("-- **Base + Stats** --",
                       format!("**Nom :** {}\n\
                       **ID :** {}\n\
                       **Membres :** {}\n\
                       **Bans :** {}\n\
                       **Rôles :** {}",
                               guild_frmbot.name.as_str(),
                               guild_frmbot.id.0,
                               guild_frmmsg.to_guild_cached(&ctx).clone().unwrap().read().members.len(),
                               guild_frmmsg.to_guild_cached(&ctx).clone().unwrap().read().bans(&ctx).unwrap().len(),
                               guild_frmmsg.to_guild_cached(&ctx).clone().unwrap().read().roles.len()),
                       true)
            ;


            e
        })
    });

    Ok(())
}