#![feature(proc_macro_hygiene, decl_macro)]

use std::path::PathBuf;
use rocket::response::Redirect;

#[macro_use]
extern crate rocket;

#[get("/<path..>")]
fn path(path: PathBuf) -> Redirect {
    let path_str = path.into_os_string().into_string().unwrap();
    Redirect::to(format!("https://ethereum-magicians.org/{}", path_str))
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("https://ethereum-magicians.org/")
}

fn main() {
    rocket::ignite().mount("/", routes![path, index]).launch();
}
