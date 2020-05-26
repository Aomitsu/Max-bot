use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError,
};

#[derive(Debug)]
pub enum BotError {
    DbError(mysql_async::error::Error),
    ParseError(ParseIntError),
    CustomError(String),
}

impl Display for BotError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for BotError {}

impl From<mysql_async::error::Error> for BotError {
    fn from(err: mysql_async::error::Error) -> BotError {
        BotError::DbError(err)
    }
}

impl From<String> for BotError {
    fn from(err: String) -> BotError {
        BotError::CustomError(err)
    }
}

impl From<ParseIntError> for BotError {
    fn from(err: ParseIntError) -> BotError {
        BotError::ParseError(err)
    }
}