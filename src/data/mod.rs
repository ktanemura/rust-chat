mod db;
mod models;

use rusqlite::{Result};
use self::models::chat_room;
pub use self::models::chat_message::{ChatMessage};

pub fn get_chat_rooms() -> Result<Vec<chat_room::ChatRoom>> {
    let conn = db::get_connection()?;
    models::get_chat_rooms(conn)
}

pub fn get_room(_id: u32) -> Result<chat_room::RoomTemplate> {
    let conn = db::get_connection()?;
    let conn2 = db::get_connection()?;
    models::get_room(conn, conn2, _id)
}

pub fn create_message(username: &String, room_id: &u32, content: &String) -> Result<ChatMessage> {
    let conn = db::get_connection()?;
    models::create_message(conn, username, room_id, content)
}

pub fn get_room_messages_time(_id: u32, time: String, name: String) -> Result<Vec<ChatMessage>> {
    let conn = db::get_connection()?;
    models::get_room_messages_time(conn, _id, time, name)
}

pub fn auth_user(username: &String, password: &String) -> Result<()> {
    let conn = db::get_connection()?;
    let conn2 = db::get_connection()?;
    models::auth_user(conn, conn2, username, password)
}

pub fn db_init() -> Result<()> {
    db::init()
}