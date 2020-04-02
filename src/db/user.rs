use mysql::prelude::{Queryable, TextQuery};
use serenity::prelude::Context;

use super::db_conn;

struct DBUser<'a> {
    id: u32,
    notifvote: &'a str,
}

pub fn get_user(_userid: u32, _ctx: &Context) {}