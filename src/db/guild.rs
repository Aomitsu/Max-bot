use rustbreak::{Database, BreakError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildDB {
    pub(crate) uid: u64,

    // Module AntiSpam
    pub(crate) anti_spam: bool,
    pub(crate) anti_spam_exclude: Vec<u64>,

    // Module Maintenance
    pub(crate) maintenance: bool,
    pub(crate) maintenance_message: String,

    // Module BotProtection
    pub(crate) bot_protection: bool,
    pub(crate) bot_allowed: Vec<u64>,
}

pub async fn gld_get(uid: u64) -> Result<GuildDB, BreakError> {
    let db = Database::<u64>::open("guilds").unwrap();

    let value : GuildDB = db.retrieve(&uid)?;
    Ok(value)
}
pub async fn gld_set(guild: GuildDB){
    let db = Database::<u64>::open("guilds").unwrap();

    db.insert(&guild.clone().uid, guild).unwrap();

    db.flush().unwrap();
}

pub async fn gld_init(guild: u64){
    if gld_get(guild).await.is_err() {
        let base = GuildDB {
            uid: guild,
            anti_spam: false,
            anti_spam_exclude: vec![],
            maintenance: false,
            maintenance_message: "".to_string(),
            bot_protection: false,
            bot_allowed: vec![]
        };

        gld_set(base).await;
    }
}