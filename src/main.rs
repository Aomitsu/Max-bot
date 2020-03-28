use std::{
    collections::HashSet,
    env,
    sync::Arc,
};

use log::{error, info};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{
        standard::macros::help,
        StandardFramework,
    },
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use serenity::framework::standard::{Args, CommandGroup, CommandResult, help_commands, HelpOptions};
use serenity::framework::standard::DispatchError::{NotEnoughArguments, Ratelimited, TooManyArguments};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::model::prelude::{Activity, OnlineStatus};

use commands::*;

mod commands;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        let guildsnum = ready.guilds.len();
        info!("Connected as {}, for {} guilds.", ready.user.name, guildsnum);

        let activity = Activity::listening(format!("Je harck {} serveurs !", guildsnum).as_ref());
        let status = OnlineStatus::Online;

        ctx.set_presence(Some(activity), status);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}



#[help]
#[command_not_found_text("Commande introuvable !")]
#[individual_command_tip("**Menu d'aide**\n\n\nCe menu d'aide vous permet de voir toute les commandes de notre bot ! Plus de détail : `help <commande/catégorie>`")]
#[strikethrough_commands_tip_in_guild("Certaines commandes sont barrées, c'est que vous n'avez pas la permission, ou qu'elle se fait dans un serveur !")]
#[strikethrough_commands_tip_in_dm("Certaines commandes sont barrées, c'est que vous n'avez pas la permission, ou qu'elle se fait en messages privés !")]
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
            },

            NotEnoughArguments {
                min, given
            } => {
                error!("{} failed: {:?}", message.content, error);
                let _ = message.channel_id.say(
                    &context,
                    format!("> **Erreur**: Vous avez donné seulement *{}* arguments, il en faut minimum *{}*.\n> Un doute ? Utilisez la commande `help`", given, min),
                );
            }
            TooManyArguments {
                max, given,
            } => {
                error!("{} failed: {:?}", message.content, error);
                let _ = message.channel_id.say(
                    &context,
                    format!("> **Erreur**: Vous avez donné *{}* arguments, la limite est de *{}*.\n> Un doute ? Utilisez la commande `help`", given, max),
                );
            }

            _ => error!("{} failed: {:?}", message.content, error),
        })
        .group(&UTILS_GROUP)
        .group(&FUN_GROUP)
        .group(&OWNER_GROUP)
    );


    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}