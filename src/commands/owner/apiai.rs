extern crate apiai;

use std::env;

use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;

use self::apiai::client::{ApiAIClient, ApiRequest};

#[command]
#[description = "Faire dire quelque chose au bot."]
#[min_args(1)]
#[owners_only]
fn apiai(ctx: &mut Context, msg: &Message, arg: Args) -> CommandResult {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config"))
        .expect("Failed to open the config file.");

    let token = settings
        .get_str("dialogflow_token")
        .expect("dialogflow_token not found in settings.");

    let _ = msg.delete(&ctx);
    let said = arg.message();

    let client = ApiAIClient {
        access_token: token,
        ..Default::default()
    };

    let req = ApiRequest {
        query: Option::Some(String::from(said)),
        ..Default::default()
    };

    let response = client.query(req).unwrap();
    let _ = msg.channel_id.say(
        &ctx.http,
        format!("> {}", response.result.fulfillment.speech),
    );

    Ok(())
}
