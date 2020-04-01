/*use mysql::*;
use mysql::prelude::*;
use std::env;

pub fn db_conn() -> Result<PooledConn> {
    let url = env::var("MYSQL_URL")
        .expect("Expected a MySQL url in the environment"); // Ex : mysql://root:password@localhost:3307/db_name
    let pool = Pool::new(url);


    let mut conn = pool.unwrap().get_conn();
    Ok(conn.unwrap())
}

pub fn db_create() {
    let db = db_conn().unwrap();
}*/