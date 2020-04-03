use serenity::framework::standard::macros::group;

use fun::captcha::*;
use fun::clyde::*;
use fun::harck::*;
use fun::phcom::*;
use fun::trumptweet::*;
use fun::vanish::*;
use guild::addemote::*;
use guild::createchannel::*;
use moderation::ban::*;
use owner::admsay::*;
use owner::apiai::*;
use owner::quit::*;
use owner::setstatus::*;
use utils::guildinfo::*;
use utils::links::*;
use utils::ping::*;
use utils::say::*;
use utils::vote::*;

pub mod utils;

#[group]
#[commands(ping, say, links, vote, guildinfo)]
#[description = "Commandes utiles."]
struct Utils;

pub mod fun;

#[group]
#[commands(vanish, harck, clyde, trumptweet, phcom, captcha)]
#[description = "Commandes souvent amusantes."]
struct Fun;

pub mod owner;

#[group]
#[commands(quit, admsay, setstatus, apiai)]
#[description = "**Réservé au créateur du bot.**"]
struct Owner;

pub mod guild;

#[group]
#[commands(createchannel, addemote)]
#[description = "**Réservé a la gestion de serveurs.**"]
struct Guild;

pub mod moderation;

#[group]
#[commands(ban)]
#[description = "**Réservé a la modération de serveurs.**"]
struct Moderation;
