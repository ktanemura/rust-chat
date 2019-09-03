// StdLib
use std::collections::HashMap;

use serde::{Serialize, Deserialize};


// Rocket (WebServer)
use rocket::http::{Cookie, Cookies, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, Request, FromRequest, Form};
use rocket::response::{Redirect};
use rocket_contrib::templates::Template;
use rocket_contrib::json::{Json};

// Custom data models
use super::data as data_models;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    username: String
}

/* Custom Request Guards (validators) */
impl<'a, 'r> FromRequest<'a, 'r> for UserSession {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let u = request.cookies()
            .get_private("uInfo")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|user| UserSession {
                username: user
            });
        
        let result = u.or_forward(());

        result
    }
}

/* Route Handlers */

// Handler for root with authenticated user
#[get("/", rank=1)]
pub fn index_auth_user(_a: UserSession) -> Redirect {
    Redirect::to("/rooms")
}

// Handler for root (login) and optional username QP
#[get("/?<username>", rank=2)]
pub fn index(username: Option<String>) -> Template {
    let mut context = HashMap::new();
    let mut u = "".to_string();
    
    if username.is_some() {
        u = username.unwrap();
    }

    context.insert("username", u);

    Template::render("index", context)
}

#[derive(FromForm)]
pub struct LoginRequest {
   pub username: String,
   pub password: String
}

#[post("/login", data = "<user>")]
pub fn login(user: Form<LoginRequest>, mut cookies: Cookies) -> Result<Redirect, Status> {
    let check = data_models::auth_user(&user.username, &user.password);
    match check {
        Ok(()) => {
            cookies.add_private(Cookie::new("uInfo", user.username.clone()));
            Ok(
                Redirect::to("/rooms")
            )
        },
        Err(_e) => {
            let response = Status::new(500, "error logging in");
            Err(
                response
            )
        }
    }
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("uInfo"));
    Redirect::to("/")
}


// Handler for rooms list
#[get("/rooms", rank=1)]
pub fn rooms(_a: UserSession) -> Template {
    let rooms = data_models::get_chat_rooms();
    
    match rooms {
        Ok(rms) => {
            let mut context = HashMap::new();
            context.insert("rooms", rms);
            Template::render("rooms", context)
        },
        Err(e) => {
            let mut context = HashMap::new();
            context.insert("error", e.to_string());
            Template::render("error", context)
        }
    }
}

// Handler for rooms list with no user session
#[get("/rooms", rank=2)]
pub fn rooms_no_auth() -> Redirect {
    Redirect::to("/")
}

// Handler for chat room page
#[get("/rooms/<_id>", rank=1)]
pub fn room(_id: u32, _a: UserSession) -> Result<Template, &'static str> {
    let room_info = data_models::get_room(_id);

    match room_info {
        Ok(context) => {
            Ok(
                Template::render("room", context)
            )
        },
        Err(e) => {
            println!("ERROR {}", e);
            Err(
                "There was an error"
            )
        }
    }
}

#[get("/rooms/<_id>", rank=2)]
pub fn room_no_auth(_id: u32) -> Redirect {
    Redirect::to("/")
}


#[derive(FromForm)]
pub struct ChatMessageRequest {
    pub room_id: u32,
    pub content: String
}

#[post("/messages", data = "<message>")]
pub fn message(message: Form<ChatMessageRequest>, _a: UserSession) -> Result<Json<data_models::ChatMessage>, Status> {
    let check = data_models::create_message(&_a.username, &message.room_id, &message.content);

    match check {
        Ok(msg) => {
            Ok(
                Json(
                    msg
                )
            )
        },
        Err(e) => {
            println!("ERROR SENDING MESSAGE {}", e);
            Err(
                Status::new(500, "Error Sending Message")
            )
        }
    }
}

#[get("/messages/<room_id>?<_d>")]
pub fn message_poll(room_id: u32, _d: String, _a: UserSession) -> Result<Json<Vec<data_models::ChatMessage>>, Status> {
    let check = data_models::get_room_messages_time(room_id, _d, _a.username);
    match check {
        Ok(msgs) => {
            Ok(
                Json(
                    msgs
                )
            )
        },
        Err(e) => {
            println!("ERROR POLLING {}", e);
            Err(
                Status::new(500, "error polling")
            )
        }
    }
}
