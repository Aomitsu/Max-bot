use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Répond pong."]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong :ping_pong: !");

    Ok(())
}


#[command]
#[description = "Faire dire quelque chose au bot."]
#[min_args(1)]
fn say(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let _ = msg.delete(&ctx);

    let said = arg.message();
    let result = str::replace(said, "everyone", " everyone").replace("here", " here");


    let _ = msg.channel_id.say(&ctx.http, result);

    Ok(())
}


#[command]
#[description = "Seul noah connaît les secrets de cette commande"]
fn links(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Liens")
                .description("Différent liens pour nous suivre partout !\n
             **Github** : [Ici](https://github.com/Aomitsu/Max-bot)\n\
             **Support** : -- Il arrivera un jour --\n\
             **Github du Créateur** : [Ici](https://github.com/Aomitsu)\n\n\
             **Ajouter le bot** : [Ici](https://github.com/Aomitsu)");

            e
        })
    });

    Ok(())
}