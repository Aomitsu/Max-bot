use serenity::{
    framework::standard::{Args, CommandResult, macros::command},
    model::prelude::*,
    prelude::*,
};
use crate::db::guild::*;
use serenity::builder::CreateEmbed;
use std::borrow::Borrow;

#[command]
async fn antispam(ctx: &Context, msg: &Message, arg: Args) -> CommandResult {
    let _ = msg.delete(ctx).await;
    gld_init(msg.guild_id.unwrap().0).await;

    let mut guild = gld_get(msg.guild_id.unwrap().0).await.unwrap();

    match arg.current().unwrap_or("help") {
        "help" => {

            let _ = msg.channel_id.send_message(&ctx, |f|f.embed(|e| e.title("AntiSpam - Aide")
                .field("**AntiSpam activé ?**", guild.anti_spam.to_string(), true)
                .field("Commande **toggle**", "`antispam toggle` permet\
                facilement de désactiver / activer l'antispam !", false)
                .field("Commande **toggle_chan**", "`antispam toggle_chan` permet\
                facilement de désactiver / activer l'antispam pour un salon spécifique, celui où la commande est entrée.", false))).await;
        }
        "toggle" => {
            match guild.anti_spam {
                true => {
                    guild.anti_spam = false;

                    let _ = msg.channel_id.send_message(&ctx, |f|f.embed(|e|
                        e.title("AntiSpam - Désactivation")
                            .description("L'antispam est désormais **désactivé** sur ce salon.")

                    )).await;

                }
                false => {
                    guild.anti_spam = true;

                    let _ = msg.channel_id.send_message(&ctx, |f|f.embed(|e|
                        e.title("AntiSpam - Activation")
                            .description("L'antispam est désormais **activé** sur ce salon.")

                    )).await;

                }
            }
        }
        "toggle_chan" => {
            match guild.anti_spam_exclude.contains(msg.channel_id.0.borrow()){
                true => {
                    let mut vec: Vec<u64> = vec![];
                    let _ = guild.anti_spam_exclude.iter().map(|a|{
                        if *a != msg.channel_id.0 { vec.push(*a); }
                    });
                    guild.anti_spam_exclude = vec;

                    let _ = msg.channel_id.send_message(&ctx, |f|f.embed(|e|
                        e.title("AntiSpam - Désactivation")
                            .description("L'antispam est désormais **désactivé** sur l'ensemble du serveur.")

                    )).await;

                }
                false => {
                    guild.anti_spam_exclude.push(*msg.channel_id.0.borrow());

                    let _ = msg.channel_id.send_message(&ctx, |f|f.embed(|e|
                        e.title("AntiSpam - Activation")
                            .description("L'antispam est désormais **activé** sur l'ensemble du serveur. ( Hors channel exclu )")

                    )).await;

                }
            }
        }
        _ => {
            let _ = msg.channel_id.say(&ctx, "Cette sous commande n'existe pas ! Utilisez `antispam help` pour obtenir la liste des commandes.").await;
        }
    }

    gld_set(guild).await;

    Ok(())
}
