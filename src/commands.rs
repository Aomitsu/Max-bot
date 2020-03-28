use serenity::framework::standard::macros::group;

use fun::captcha::*;
use fun::clyde::*;
use fun::harck::*;
use fun::phcom::*;
use fun::trumptweet::*;
use fun::vanish::*;
use owner::admsay::*;
use owner::quit::*;
use owner::setstatus::*;
use utils::links::*;
use utils::ping::*;
use utils::say::*;

pub mod utils;

#[group]
#[commands(ping, say, links)]
#[description = "Commandes utiles."]
struct Utils;

pub mod fun;

#[group]
#[commands(vanish, harck, clyde, trumptweet, phcom, captcha)]
#[description = "Commandes souvent amusantes."]
struct Fun;

pub mod owner;

#[group]
#[commands(quit, admsay, setstatus)]
#[description = "**Réservé au créateur du bot.**"]
struct Owner;