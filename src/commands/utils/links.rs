use rand::{Rng, thread_rng};
use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;

#[command]
#[description = "Seul noah connaît les secrets de cette commande"]
#[aliases(link, liens, lien)]
fn links(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut rng = thread_rng();

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Liens")
                .color(Colour::new(rng.gen_range(0, 16777215)))
                .description("Différent liens pour nous suivre partout !\n
             **Github** : [Ici](https://github.com/Aomitsu/Max-bot)\n\
             **Github du Créateur** : [Ici](https://github.com/Aomitsu)\n\n\
             **Ajouter le bot** : -- Il arrivera un jour --\n\
             **Support** : [Ici](https://discord.gg/DRwfvJ4)\n\n\
             Bot OpenSource créé par `Max¹#0064`, en [`Rust`](https://www.rust-lang.org), utilisant principalement le package [`Serenity`](https://crates.io/crates/serenity) version `0.8.0`\n\
             Inspiré dans la manière de faire d'un autre bot, [`Eli Bot`](https://github.com/Elinvynia/bot) de `Elinvynia` ! [`License MIT`](https://github.com/Elinvynia/bot/blob/master/LICENSE.md)");

            e
        })
    });

    Ok(())
}
