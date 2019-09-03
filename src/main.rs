#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

/* Imports */

// Rocket (WebServer)
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;


/* Custom module imports */
mod data;
mod router;

// Entry point for RUST program
fn main() {
    let db_status = data::db_init();
    match db_status {
        Ok(()) => {
            rocket::ignite()
                .attach(Template::fairing())
                .mount("/css", StaticFiles::from("public/css")) // Static css files, route handler will be relative to "/css" instead of root
                .mount("/js", StaticFiles::from("public/js")) // Static css files, route handler will be relative to "/css" instead of root
                .mount("/", routes![router::index, router::login, router::logout, router::index_auth_user]) // Attach handlers relative to root path
                .mount("/", routes![router::rooms, router::rooms_no_auth, router::room, router::room_no_auth]) // Attach handlers relative to root path
                .mount("/", routes![router::message, router::message_poll]) // Attach handlers relative to root path
                .launch();
        },
        Err(e) => {
            println!("Error init db {}", e);
        }
    }

}