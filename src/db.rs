use std::error::Error;
use std::fs::File;
use std::path::Path;

use log::error;
use sqlx::{Pool, SqlitePool};
use sqlx::pool::PoolConnection;
use sqlx::prelude::*;
use sqlx::prelude::*;
use sqlx::sqlite::SqliteConnection;

pub mod vote;

pub async fn create_db() {
    let db = Path::new("db.sqlite3");
    if !db.exists() {
        match File::create(&db) {
            Ok(_) => (),
            Err(e) => error!("Failed to create database file: {}", e),
        }
    }
    create_tables().await;
}

pub async fn get_db() -> PoolConnection<SqliteConnection> {
    let pool = SqlitePool::new("sqlite://db.sqlite3").await.unwrap();
    return pool.acquire().await.unwrap();
}

async fn create_tables() {
    let mut conn = get_db().await;

    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS vote (channel_id TEXT NOT NULL, message_id TEXT NOT NULL, user_id TEXT NOT NULL);")
        .execute(&mut conn).await;
}