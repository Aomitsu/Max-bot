use rand::{Rng, thread_rng};
use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::{
    prelude::*
};
use serenity::model::prelude::ReactionType::Unicode;
use serenity::prelude::*;
use serenity::utils::Colour;

#[command]
#[description = "Créer un sondage"]
#[min_args(1)]
#[required_permissions(MANAGE_MESSAGES)]
fn vote(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let mut rng = thread_rng();
    let _ = msg.delete(&ctx);

    let said = arg.message();

    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Vote !")
                .description(said)
                .color(Colour::new(rng.gen_range(0, 16777215)))
                .footer(|f| {
                    f.text(format!(" | Lançé par {}", msg.author.name))
                        .icon_url(msg.author.avatar_url().unwrap());

                    f
                });

            e
        })
    }).unwrap();

    let _ = msg.react(&ctx, { Unicode("👍".parse()?) });
    let _ = msg.react(&ctx, { Unicode("👎".parse()?) });

    Ok(())
}