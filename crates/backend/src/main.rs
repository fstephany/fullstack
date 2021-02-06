use rocket::Rocket;
use rocket::{get, launch, routes};

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/hello/<name>")]
fn hi(name: String) -> String {
    name
}

#[launch]
fn start() -> Rocket {
    rocket::ignite().mount("/", routes![hello, hi])
}
