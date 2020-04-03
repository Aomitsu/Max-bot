use std::env;

use apiai::client::{ApiAIClient, ApiRequest};
use log::info;
use serenity::{model::prelude::*, prelude::*};

pub fn message(_ctx: Context, msg: Message) {
    let bot_id = _ctx.http.get_current_user().unwrap().id.0;
    let author_id = msg.author.id.0;
    if msg.channel_id.name(&_ctx).unwrap().starts_with("max-chat") && bot_id != author_id {
        let _successful = msg.channel_id.broadcast_typing(&_ctx);
        let said = msg.clone().content;

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

        if response.result.action.starts_with("nsfw.") {
            if msg.channel(&_ctx).unwrap().is_nsfw() {
                let _ = msg
                    .channel_id
                    .say(&_ctx, format!("> {}", response.result.fulfillment.speech));
            } else {
                let _ = msg.channel_id.say(&_ctx, format!("> Ouh ! Cette réponse contiens du `NSFW`, or, ce channel ne le permets pas !\n\
                > Je te conseille de supprimer ton message avant qu'un modérateur arrive."));
            }
        } else {
            let _ = msg
                .channel_id
                .say(&_ctx, format!("> {}", response.result.fulfillment.speech));
        }
    }
}
