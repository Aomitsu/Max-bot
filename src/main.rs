use std::{
    collections::HashSet,
    env,
    sync::Arc,
};

use log::{error, info};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{
        standard::macros::{group, help},
        StandardFramework,
    },
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use serenity::framework::standard::{Args, CommandGroup, CommandResult, help_commands, HelpOptions};
use serenity::framework::standard::DispatchError::Ratelimited;
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::model::prelude::{Activity, OnlineStatus};

use commands::{
    owner::*,
    utils::*,
};

mod commands;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        let activity = Activity::streaming("Harcking in progress...", "https://www.twitch.tv/zerator");
        let status = OnlineStatus::DoNotDisturb;

        ctx.set_presence(Some(activity), status);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(ping, say)]
struct Utils;

#[group]
#[commands(quit, admsay)]
struct Owner;

#[help]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

fn main() {
    env_logger::init();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix("!"))
        .help(&MY_HELP)
        .on_dispatch_error(|context, message, error| match error {
            Ratelimited(e) => {
                error!("{} failed: {:?}", message.content, error);
                let _ = message.channel_id.say(
                    &context,
                    format!("Ratelimited: Try again in {} seconds.", e),
                );
            }
            _ => error!("{} failed: {:?}", message.content, error),
        })
        .group(&UTILS_GROUP)
        .group(&OWNER_GROUP));


    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}