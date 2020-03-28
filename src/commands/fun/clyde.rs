use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::{
    channel::Message
};
use serenity::prelude::*;

#[command]
#[description = "Générer un screen de Clyde qui parle ( NekoBotAPI )"]
#[min_args(1)]
#[usage("<message>")]
#[example("Je suis clyde")]
fn clyde(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let txt = arg.message();

    let json: serde_json::Value = reqwest::get(&format!("https://nekobot.xyz/api/imagegen?type={}&text={}&raw=0", "clyde", txt))
        .expect("couldn't retrieve image")
        .json()?;

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Clyde")
                .image(json["message"].as_str().unwrap());

            e
        })
    });

    Ok(())
}
