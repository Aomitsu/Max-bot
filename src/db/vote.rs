use sql_builder::{SqlBuilder, quote};

use super::db_connect;
use mysql_async::prelude::*;


use log::info;

pub struct Vote {
    pub message_id: u64,
    pub user_id: u64,
}


pub(crate) async fn add_vote(vote: Vote){
    let db = db_connect().await.unwrap();


    let sql = async{ SqlBuilder::insert_into("vote")
        .field("message_id")
        .field("user_id")
        .values(&[&quote(vote.message_id.to_string()), &quote(&vote.user_id.to_string())])
        .sql().unwrap() };

    let req = sql.await;

    info!("{}", req.to_string());


    let _ = db.drop_exec(req, ()).await;


}

pub(crate) async fn rem_vote(vote: u64){
    let db = db_connect().await.unwrap();
    let sql = async{ SqlBuilder::delete_from("vote")
        .and_where_eq("message_id", &quote(vote.to_string()))
        .sql().unwrap()
    };
    let req = sql.await;

    info!("{}", req.to_string());


    let _ = db.drop_exec(req, ()).await;

}