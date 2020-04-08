use std::error::Error;
use std::io::Read;

use log::info;
use sqlx::prelude::*;
use sqlx::sqlite::*;

use super::get_db;

pub struct Vote {
    pub channel_id: u64,
    pub message_id: u64,
    pub user_id: u64,
}

pub async fn add_vote(vote: Box<Vote>) {
    let mut conn = get_db().await;

    let _ = sqlx::query("INSERT INTO vote VALUES (?, ?, ?)")
        .bind(vote.channel_id.to_string())
        .bind(vote.message_id.to_string())
        .bind(vote.user_id.to_string())
        .execute(&mut conn).await;

    info!("Vote créé");
    let test = vote_user_id(vote.message_id).await.unwrap();
    info!("Test : {}", test)
}
/*pub async fn is_vote(msg_id: u64) -> Result<bool, sqlx::Error> {

}
pub async fn rem_vote(msg_id: u64){
    let mut conn = get_db().await;

    info!("Vote supprimé");

    let _ = sqlx::query("DELETE FROM vote WHERE message_id = ?")
        .bind(msg_id.to_string())
        .execute(&mut conn).await;
}*/
pub async fn vote_user_id(msg_id: u64) -> Result<String, sqlx::Error> {
    let mut conn = get_db().await;

    let request: String = sqlx::query("SELECT user_id FROM vote WHERE message_id = ?")
        .bind(msg_id.to_string())
        .fetch(&mut conn)
        .next()
        .await?
        .unwrap()
        .try_get(0)?;

    Ok(request)
}