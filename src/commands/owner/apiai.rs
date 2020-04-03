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
    let _ = msg.delete(&ctx);
    let said = arg.message();

    let my_token = env::var("DFLOW_TOKEN").expect("Expected a token in the environment");

    let client = ApiAIClient {
        access_token: my_token,
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
