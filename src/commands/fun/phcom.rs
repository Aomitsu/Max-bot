use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[description = "Générer un commentaire d'un célèbre site pour adulte ( NekoBotAPI )"]
#[min_args(1)]
#[usage("<message>")]
#[example("Sah quel plaisir")]
fn phcom(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let txt = arg.message();

    let json: serde_json::Value = reqwest::get(&format!(
        "https://nekobot.xyz/api/imagegen?type={}&image={}&text={}&username={}&raw=0",
        "phcomment",
        msg.author.avatar_url().unwrap(),
        txt,
        msg.author.name
    ))
        .expect("couldn't retrieve image")
        .json()?;

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("P.H. Comment")
                .image(json["message"].as_str().unwrap());

            e
        })
    });

    Ok(())
}
