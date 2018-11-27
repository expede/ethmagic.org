#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::Redirect;

#[get("/<path>")]
fn path(path: String) -> Redirect {
    Redirect::to(format!("https://ethereum-magicians.org/{}", path))
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("https://ethereum-magicians.org/")
}

fn main() {
    rocket::ignite().mount("/", routes![path, index]).launch();
}
