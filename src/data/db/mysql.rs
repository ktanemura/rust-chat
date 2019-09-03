use rusqlite::{Connection, Result, NO_PARAMS,};
use super::sqlite as sql;
// use super::mysql as sql;

pub fn get_connection() -> Result<Connection> {
    sql::get_connection()
}

pub fn init() -> Result<()> {
    sql::init()
}