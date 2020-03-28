use log::info;
use serenity::{model::prelude::*, prelude::*};

pub fn resume(_: Context, _: ResumedEvent) {
    info!("Resumed");
}