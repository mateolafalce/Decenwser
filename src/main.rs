/*
  -Roots
  TODO: Test with 9000
*/

mod functions;
mod pages;

#[macro_use]
extern crate rocket;
use functions::{
    get_page::{
      store_len, 
      get_page, 
      clear, 
      store_domain, 
      store_pdas, 
      get_data_domain
    }, 
    create_app::create_app, 
    send_app::{
      send_app, 
      store_wallet, 
      get_iter,
      encode
    },
    store_app::store_app, 
    delete::{delete_pda, delete_stored_app},
    config_settings::{get_config_settings, modify_network},
};
use pages::{convert_page, main_page, config_page, data_domain};
use rocket::{fs::{relative, FileServer}};
use rocket_dyn_templates::Template;
use std::process::{Command};

#[rocket::main]
async fn main() {
Command::new("C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe")
        .arg("http://127.0.0.1:2004/main")
        .spawn()
        .expect("Failed to start chrome");
Command::new("node")
        .arg("server.js")
        .spawn()
        .expect("failed to start node server");
    let _rocket = rocket::build()
        .mount("/main", routes![main_page::index])
        .mount("/main", routes![data_domain::index])
        .mount("/main", routes![config_page::index])
        .mount("/modify_network", routes![modify_network::index])
        .mount("/clear", routes![clear::index])
        .mount("/get_config", routes![get_config_settings::index])
        .mount("/encode", routes![encode::index])
        .mount("/get_iter", routes![get_iter::index])
        .mount("/get_data_domain", routes![get_data_domain::index])
        .mount("/store_pdas", routes![store_pdas::index])
        .mount("/store_wallet", routes![store_wallet::index])
        .mount("/store_domain", routes![store_domain::index])
        .mount("/get_len", routes![store_len::index])
        .mount("/get_page", routes![get_page::index])
        .mount("/store_app", routes![store_app::index])
        .mount("/delete_stored_app", routes![delete_stored_app::index])
        .mount("/main", routes![convert_page::index])
        .mount("/create_app", routes![create_app::index])
        .mount("/delete_pda", routes![delete_pda::index])
        .mount("/send_app", routes![send_app::index])
        .mount("/", FileServer::from(relative!("/public")))
        .attach(Template::fairing())
        .launch()
        .await;
}