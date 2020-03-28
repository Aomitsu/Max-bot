use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Seul noah connaît les secrets de cette commande"]
#[aliases(link, liens, lien)]
fn links(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Liens")
                .description("Différent liens pour nous suivre partout !\n
             **Github** : [Ici](https://github.com/Aomitsu/Max-bot)\n\
             **Github du Créateur** : [Ici](https://github.com/Aomitsu)\n\n\
             **Ajouter le bot** : -- Il arrivera un jour --\n\
             **Support** : [Ici](https://discord.gg/DRwfvJ4)");

            e
        })
    });

    Ok(())
}