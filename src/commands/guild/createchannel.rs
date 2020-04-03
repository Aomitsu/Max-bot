use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::channel::ChannelType::{Category, Text, Voice};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Créer un salon."]
#[min_args(2)]
#[usage("<text/voice/category> <nom>")]
#[required_permissions(MANAGE_CHANNELS)]
fn createchannel(ctx: &mut Context, msg: &Message, mut arg: Args) -> CommandResult {
    let mut chantype: ChannelType = Text;
    let mut valide = 1;

    match arg.clone().current().unwrap() {
        "text" => chantype = Text,
        "voice" | "voix" => chantype = Voice,
        "category" | "catégorie" => chantype = Category,
        _ => {
            let _ = msg.channel_id.say(
                &ctx.http,
                "> **Houp !**\n\
                 > La commande s'utilise ainsi : `createchannel <text/voice/category> <nom>`",
            );
            valide = 0
        }
    }

    if valide == 1 {
        arg.advance();
        create(ctx, msg, chantype, arg.clone().rest());
    }

    Ok(())
}

fn create(ctx: &mut Context, msg: &Message, chantype: ChannelType, name: &str) {
    let _ = msg.guild_id.unwrap().create_channel(&ctx.http, |chan| {
        chan.name(name).kind(chantype);
        chan
    });
}
