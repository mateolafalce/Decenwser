mod functions;

#[macro_use]
extern crate rocket;
use functions::{get_page, render_app, App};
use rocket::{
    fs::{relative, FileServer},
    serde::json::Json,
};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        context! {
            foo: 123,
        },
    )
}

#[post("/", data = "<domain>")]
fn get_html_css_js(domain: String) -> Json<App> {
    Json(get_page(domain).unwrap())
}

/*#[get("/")]
fn render() -> Template {
    render_app();
    Template::render(
        "web/index",
        context! {
            foo: 123,
        },
    )
}*/

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/get_app", routes![get_html_css_js])
        //.mount("/render", routes![render])
        //.mount("/web3", routes![web3])
        .mount("/", FileServer::from(relative!("/public")))
        .attach(Template::fairing())
}
