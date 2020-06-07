use rustbreak::{Database, BreakError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AntiSpamDB {
    pub(crate) uid: u64,
    pub(crate) timestamp: u64,
    pub(crate) alert: i32,
    pub(crate) latest_message: String,
}

pub async fn as_get(uid: u64) -> Result<AntiSpamDB, BreakError> {
    let db = Database::<u64>::open("tmp/antispam").unwrap();

    let value : AntiSpamDB = db.retrieve(&uid)?;
    Ok(value)
}
pub async fn as_set(anti_spam: AntiSpamDB){
    let db = Database::<u64>::open("tmp/antispam").unwrap();

    db.insert(&anti_spam.clone().uid, anti_spam).unwrap();

    db.flush().unwrap();
}