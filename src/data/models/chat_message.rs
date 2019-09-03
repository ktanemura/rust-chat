use serde::{Serialize, Deserialize};
use rusqlite::{Connection, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    id: u32,
    room: u32,
    user: String,
    content: String,
    time: String
}

pub fn get_room_messages(conn: Connection, _id: u32) -> Result<Vec<ChatMessage>> {
    let mut stmt = conn.prepare("SELECT * FROM chat_messages WHERE room_id = ?")?;
    let rows = stmt.query_map(&[_id], |row|
            Ok(
                ChatMessage {
                    id: row.get(0)?,
                    room: row.get(1)?,
                    user: row.get(2)?,
                    content: row.get(3)?,
                    time: row.get(4)?
                }
            )
        )?;

    let mut msgs : Vec<ChatMessage> = Vec::new();

    for msg_result in rows {
        msgs.push(msg_result?);
    }    


    Ok(msgs)
}

pub fn create_message(conn: Connection, username: &String, room_id: &u32, content: &String) -> Result<ChatMessage> {
    let mut stmt = conn.prepare("INSERT INTO chat_messages (user, room_id, content) VALUES(?, ?, ?)")?;

    stmt.execute(&[username, &(room_id.to_string()), content])?;
    
    let last_id = conn.last_insert_rowid();
    stmt = conn.prepare("SELECT * FROM chat_messages WHERE id = ?")?;
    let row = stmt.query_row(&[last_id], |row|
            Ok(
                ChatMessage {
                    id: row.get(0)?,
                    room: row.get(1)?,
                    user: row.get(2)?,
                    content: row.get(3)?,
                    time: row.get(4)?
                }
            )
        )?;

    Ok(row)
}

pub fn get_room_messages_time(conn: Connection, _id: u32, time: String, name: String) -> Result<Vec<ChatMessage>> {
    let mut stmt = conn.prepare("SELECT * FROM chat_messages WHERE room_id = ? AND time >= ? AND user != ?")?;
    let rows = stmt.query_map(&[_id.to_string(), time, name], |row|
            Ok(
                ChatMessage {
                    id: row.get(0)?,
                    room: row.get(1)?,
                    user: row.get(2)?,
                    content: row.get(3)?,
                    time: row.get(4)?
                }
            )
        )?;

    let mut msgs : Vec<ChatMessage> = Vec::new();

    for msg_result in rows {
        msgs.push(msg_result?);
    }    


    Ok(msgs)
}