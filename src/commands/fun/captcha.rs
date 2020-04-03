use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[description = "Devenez le fameux ReCaptcha ( NekoBotAPI )"]
fn captcha(ctx: &mut Context, msg: &Message) -> CommandResult {
    let json: serde_json::Value = reqwest::get(&format!(
        "https://nekobot.xyz/api/imagegen?type={}&url={}&username={}&raw=0",
        "captcha",
        msg.author.avatar_url().unwrap(),
        msg.author.name
    ))
        .expect("couldn't retrieve image")
        .json()?;

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Captcha").image(json["message"].as_str().unwrap());

            e
        })
    });

    Ok(())
}
