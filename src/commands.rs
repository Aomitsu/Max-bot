use serenity::framework::standard::macros::group;

use owner::admsay::*;
use owner::test::*;

use admin::antispam::*;

pub mod owner;
pub mod admin;

#[group]
#[commands(admsay, test)]
struct Owner;

#[group]
#[commands(antispam)]
struct Admin;
