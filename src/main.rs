#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template { 
    Template::render("card", context! {
        title: "Hello",
        items: vec!["One", "Two", "Three"],
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .attach(Template::fairing())
    .mount("/", FileServer::from(relative!("/static")))
}
