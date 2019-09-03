use serde::{Serialize, Deserialize};
use rusqlite::{Connection, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatUser{
    id: u32,
    username: String
}

// Check if user with username exists in db
fn check_user_exists(conn: Connection, username: &String) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * from chat_users WHERE username = ?")?;

    let _row = stmt.query_row(&[username], |_r|
            Ok(true)
        )?;

    Ok(())
}

// Login user by checking username/password combo
fn login_user(conn: Connection, username: &String, password: &String) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * from chat_users WHERE username = ? AND password = ?")?;

    let _row = stmt.query_row(&[username, password], |r|
            Ok(true)
        )?;
    Ok(())
}

fn create_user(conn: Connection, username: &String, password: &String) -> Result<()>  {
    let mut stmt = conn.prepare("INSERT INTO chat_users (username, password) VALUES(?1, ?2)")?;

    stmt.execute(&[username, password])?;

    Ok(())
}

// Login User or Create Profile if needed
pub fn auth_user(conn: Connection, conn2: Connection, username: &String, password: &String) -> Result<()> {
    let user_exists = check_user_exists(conn, username);

    match user_exists {
        Ok(()) => {
            login_user(conn2, username, password)
        },
        Err(_e) => {
            create_user(conn2, username, password)
        }
    }
}