use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::{
    channel::Message
};
use serenity::prelude::*;

#[command]
#[description = "Et les taches s'évanouissent"]
fn vanish(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Vanish")
                .description("Et les taches s'évanouissent.")
                .image("https://img.tesco.com/Groceries/pi/470/5011417570470/IDShot_540x540.jpg");

            e
        })
    });

    Ok(())
}

#[command]
#[description = "Seul noah connaît les secrets de cette commande"]
fn harck(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Harck")
                .description("Sette api ais en devve");

            e
        })
    });

    Ok(())
}

#[command]
#[description = "Générer un screen de Clyde qui parle ( NekoBotAPI )"]
#[min_args(1)]
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


#[command]
#[description = "Faire Tweeter Trump ( NekoBotAPI )"]
#[min_args(1)]
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

#[command]
#[description = "Générer un commentaire d'un célèbre site pour adulte ( NekoBotAPI )"]
#[min_args(1)]
fn phcom(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let txt = arg.message();

    let json: serde_json::Value = reqwest::get(&format!(
        "https://nekobot.xyz/api/imagegen?type={}&image={}&text={}&username={}&raw=0",
        "phcomment",
        msg.author.avatar_url().unwrap(),
        txt,
        msg.author.name))
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