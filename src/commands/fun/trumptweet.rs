use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::{
    channel::Message
};
use serenity::prelude::*;

#[command]
#[description = "Faire Tweeter Trump ( NekoBotAPI )"]
#[min_args(1)]
#[usage("<message>")]
#[example("Je vous ai compris !")]
fn trumptweet(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let txt = arg.message();

    let json: serde_json::Value = reqwest::get(&format!("https://nekobot.xyz/api/imagegen?type={}&text={}&raw=0", "trumptweet", txt))
        .expect("couldn't retrieve image")
        .json()?;

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Trump")
                .image(json["message"].as_str().unwrap());

            e
        })
    });

    Ok(())
}
