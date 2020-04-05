use std::env;

use apiai::client::{ApiAIClient, ApiRequest};
use log::info;
use serenity::{model::prelude::*, prelude::*};

pub fn message(_ctx: Context, msg: Message) {
    let bot_id = _ctx.http.get_current_user().unwrap().id.0;
    let author_id = msg.author.id.0;
    if msg.channel_id.name(&_ctx).unwrap().starts_with("max-chat") && bot_id != author_id {
        let desc_channel_defaut = format!(
            "Parlez a Max Bot ! Pour qu'il ne réponde pas, mettez un - devant votre message :eyes:"
        );
        let _ = msg.channel_id.edit(&_ctx, |c| {
            c.topic(desc_channel_defaut);
            c
        });

        let _successful = msg.channel_id.broadcast_typing(&_ctx);
        let said = msg.clone().content;

        let mut settings = config::Config::default();
        settings
            .merge(config::File::with_name("config"))
            .expect("Failed to open the config file.");

        let token = settings
            .get_str("dialogflow_token")
            .expect("dialogflow_token not found in settings.");

        let client = ApiAIClient {
            access_token: token,
            ..Default::default()
        };

        let req = ApiRequest {
            query: Option::Some(String::from(said)),
            ..Default::default()
        };

        let response = client.query(req).unwrap();
        let message_bot = str::replace(response.result.fulfillment.speech.as_ref(), "everyone", "_everyone").replace("here", "_here");

        if response.result.action.starts_with("interract.") {
            if response.result.action.ends_with("like") {
                let any = response.result.parameters.get("any").unwrap();

                let _ = msg.channel_id.say(
                    &_ctx,
                    format!("> Oui, {} est l'une de mes passions ^^", any),
                );
            }
        } else {
            if response.result.action.starts_with("nsfw.") {
                if msg.channel(&_ctx).unwrap().is_nsfw() {
                    let _ = msg
                        .channel_id
                        .say(&_ctx, format!("> {}", message_bot));
                } else {
                    let _ = msg.channel_id.say(&_ctx, format!("> Ouh ! Cette réponse contiens du `NSFW`, or, ce channel ne le permets pas !\n\
                    > Je te conseille de supprimer ton message avant qu'un modérateur arrive."));
                }
            } else {
                let _ = msg
                    .channel_id
                    .say(&_ctx, format!("> {}", message_bot));
            }
        }
    }
}
