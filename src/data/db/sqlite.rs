// SQL
use rusqlite::{Connection, Result, NO_PARAMS, Row, MappedRows, ToSql};

pub fn get_connection() -> Result<Connection> {
    Connection::open("ace_chat.db")
}

pub fn init() -> Result<()> {
    let conn = get_connection()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS chat_rooms (
            id integer primary key,
            name text not null unique
        )",
        NO_PARAMS
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS chat_users (
            id integer primary key,
            username text not null unique,
            password text not null
        )",
        NO_PARAMS
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS chat_messages (
            id integer primary key,
            room_id integer not null,
            user text not null,
            content text not null,
            time TIMESTAMP not null default CURRENT_TIMESTAMP,
            FOREIGN KEY(room_id) REFERENCES chat_rooms(id),
            FOREIGN KEY(user) REFERENCES chat_users(username)
        )",
        NO_PARAMS
    )?;

    Ok(())
}

pub fn query_map_row<T: std::iter::IntoIterator, F>(query: &str, params: T, map_function: &Fn(&Row) -> Result<F>) -> Result<F> where <T as std::iter::IntoIterator>::Item: ToSql {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(query)?;

    let row = stmt.query_row(params, map_function)?;

    Ok(row)
}

// MappedRows<'a, &'a Fn(&Row) -> Result<F>>

pub fn query_map_rows <T: std::iter::IntoIterator, F>(query: &str, params: T, map_function: &Fn(&Row) -> Result<F>) -> Result<Vec<F>> where <T as std::iter::IntoIterator>::Item: ToSql {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(query)?;

    let rows = stmt.query_map(params, map_function)?;
    let mut objs: Vec<F> = Vec::new();

    for row in rows {
        objs.push(row?);
    }

    Ok(objs)
}