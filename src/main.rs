#[macro_use]
extern crate rocket;

use rocket::{
    fs::{relative, FileServer},
    serde::{Deserialize, Serialize},
};
use rocket_dyn_templates::{context, Template};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Card {
    title: String,
    code: String,
    question: String,
    answer: Vec<Key>,
    explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Key {
    name: String,
    description: String,
}
#[get("/")]
fn index() -> Template {
    Template::render(
        "card",
        context! {
            card: Card {
                title: "Vim".to_string(),
                code: r#"let foo = String::from(<mark>"b<mark class="cursor">a</mark>r"</mark>);"#.to_string(),
                question: r#"Delete the word <strong>"bar"</strong> including the quotes"#.to_string(),
                answer: vec![Key{name: "d".to_string(), description: "delete".to_string()},
                Key{name: "a".to_string(), description: "around".to_string()},
                Key{name: r#"""#.to_string(), description: "around this character".to_string()}],
                ..Default::default()
            }
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")))
}
