#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;


mod routes;

fn main() {
    rocket::ignite().mount("/", routes![routes::index]).launch();
}

