use log::info;
use serenity::{model::prelude::*, prelude::*};

pub fn ready(ctx: Context, ready: Ready) {
    let guildsnum = ready.guilds.len();

    info!(
        "Connected as {}, for {} guilds.",
        ready.user.name, guildsnum
    );

    let activity = Activity::listening(format!("Je harck {} serveurs !", guildsnum).as_ref());
    let status = OnlineStatus::Online;

    ctx.set_presence(Some(activity), status);
}
