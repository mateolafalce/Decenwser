mod functions;
mod pages;

#[macro_use]
extern crate rocket;
use functions::encode::encode;
use functions::get_page::get_page;
use functions::create_app::create_app;
use functions::send_app::{send_html, send_css, send_js};
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
        .mount("/create_app", routes![create_app::index])
        .mount("/send_html", routes![send_html::index])
        .mount("/send_css", routes![send_css::index])
        .mount("/send_js", routes![send_js::index])
        .mount("/", FileServer::from(relative!("/public")))
        .attach(Template::fairing())
}
