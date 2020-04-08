use std::collections::HashSet;

use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::prelude::*;

use commands::*;
use db::*;
use events::Handler;

mod events;
mod commands;

mod db;

#[tokio::main]
async fn main() {
    env_logger::init();

    create_db().await;

    /* - Init config file - */
    let mut config = config::Config::default();
    config
        .merge(config::File::with_name("config"))
        .expect("Config file not found 😱");

    /* - Get token from Config File - */
    let token = config
        .get_str("discord_token")
        .expect("discord_token not found in settings file 😱");
    let http = Http::new_with_token(&token);

    /* - Get Bot infos - */
    let (owners, botid, _ownerid) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id, info.owner.id)
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    /* - Define and configure framework - */
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).on_mention(Some(botid)).prefix("&"))
        .group(&OWNER_GROUP)
        .group(&UTILS_GROUP);

    /* - Init and start Client - */
    let mut client = Client::new_with_framework(token, Handler, framework)
        .await
        .expect("Client not init 😱");
    client
        .start_autosharded()
        .await
        .expect("Failed to start the client 😱");
}
