#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;
#[macro_use] extern crate lazy_static;

mod store;
mod routes;

fn main() {
    rocket::ignite().mount("/", routes![routes::index, routes::get_json, routes::add_new_item, routes::get_item]).launch();
}

