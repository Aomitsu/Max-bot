use serenity::framework::standard::macros::group;

use owner::admsay::*;
use utils::vote::*;
use utils::ping::*;
use utils::guild_reset::*;
use moderation::warn::*;
use moderation::ban::*;

pub mod owner;

#[group]
#[commands(admsay)]
struct Owner;

pub mod utils;

#[group]
#[commands(vote, ping, guild_reset)]
struct Utils;

pub mod moderation;

#[group]
#[commands(warn, ban)]
struct Moderation;