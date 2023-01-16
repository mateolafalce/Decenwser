mod functions;
mod pages;

#[macro_use]
extern crate rocket;
use functions::encode::encode;
use functions::get_page::get_page;
use pages::convert_page;
use pages::main_page;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![main_page::index])
        .mount("/encode", routes![encode::index])
        .mount("/get_page", routes![get_page::index])
        .mount("/convert", routes![convert_page::index])
        .mount("/", FileServer::from(relative!("/public")))
        .attach(Template::fairing())
}
