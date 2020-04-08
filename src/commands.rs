use serenity::framework::standard::macros::group;

use owner::admsay::*;
use utils::vote::*;

pub mod owner;

#[group]
#[commands(admsay)]
struct Owner;

pub mod utils;

#[group]
#[commands(vote)]
struct Utils;