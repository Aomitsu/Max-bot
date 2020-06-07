use serenity::framework::standard::macros::group;

use owner::admsay::*;
use owner::test::*;

pub mod owner;

#[group]
#[commands(admsay, test)]
struct Owner;
