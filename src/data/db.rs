use rusqlite::{Connection, Result};

pub mod sqlite;
// mod mysql;

pub use self::sqlite as sql;

pub fn get_connection() -> Result<Connection> {
    sql::get_connection()
}

pub fn init() -> Result<()> {
    sql::init()
}