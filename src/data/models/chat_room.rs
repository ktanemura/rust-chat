use rusqlite::{Connection, Result, NO_PARAMS, Row};

use super::chat_message;
use crate::data::db::sql;

// // Serde (Trait Macros)
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRoom {
    id: u32,
    name: String
}

impl ChatRoom {
    pub fn get_name(&self) -> &String{
        &self.name
    }

    pub fn get_id(&self) -> &u32 {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomTemplate {
    room_name: String,
    room_id: u32,
    messages: Vec<chat_message::ChatMessage>

}

fn map_room(row: &Row) -> Result<ChatRoom> {
    Ok(
        ChatRoom {
            id: row.get(0)?,
            name: row.get(1)?
        }
    )
}

// fn map_rooms(row: &Row)

pub fn get_chat_rooms(conn: Connection) -> Result<Vec<ChatRoom>> {
    sql::query_map_rows("SELECT id, name FROM chat_rooms", NO_PARAMS, &map_room)
}


fn get_chat_room(_id: u32) -> Result<ChatRoom> {
    sql::query_map_row("SELECT id, name FROM chat_rooms WHERE id = ?", &[_id.to_string()], &map_room)
}

pub fn get_room(conn: Connection, conn2: Connection, _id: u32) -> Result<RoomTemplate> {
    // Get room info first so if room DNE messages call will not occur
    let room = get_chat_room(_id)?;
    let msgs = chat_message::get_room_messages(conn2, _id)?;

    Ok(RoomTemplate {
        room_name: room.get_name().to_string(),
        room_id: *room.get_id(),
        messages: msgs
    })
}