use super::db_connect;
use mysql_async::prelude::*;
use sql_builder::{SqlBuilder, quote};

use log::info;

pub struct Case<> {
    pub case_type: Option<String>,
    pub guild_id: u64,
    pub user_id: u64,
    pub message: Option<String>,
}


pub(crate) async fn add_case(case: Case){
    let db = db_connect().await.unwrap();

    let sql = async{ SqlBuilder::insert_into("cases")
        .field("type")
        .field("guild_id")
        .field("user_id")
        .field("message")
        .values(&[
            &quote(case.case_type.unwrap()),
            &case.guild_id.to_string(),
            &case.user_id.to_string(),
            &quote(&case.message.unwrap())
        ])
        .sql().unwrap() };

    let req = sql.await;

    info!("{}", req.to_string());


    let _ = db.drop_exec(req, ()).await;

}