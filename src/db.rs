use mysql_async::Conn;

pub mod vote;
pub mod case;

use crate::error::BotError;

pub(crate) async fn db_connect() -> Result<Conn, BotError>{
    let mut config = config::Config::default();
    config
        .merge(config::File::with_name("config"))
        .expect("Config file not found 😱");

    let url = config
        .get_str("mysql_url")
        .expect("mysql_url not found in settings file 😱");

    let database_url = url;

    let pool = mysql_async::Pool::new(database_url);
    let conn = pool.get_conn().await?;

    Ok(conn)
}