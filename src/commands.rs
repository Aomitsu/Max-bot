use serenity::framework::standard::macros::group;

use owner::admsay::*;

pub mod owner;

#[group]
#[commands(admsay)]
struct Owner;
