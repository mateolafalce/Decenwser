/*
  -Roots
  TODO: Loading gift
  TODO: Create Apps 
  TODO: Connect it with the decenwser network 
  TODO: Account stats
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
      get_data_domain,
      store_web_command
    }, 
    create_app::create_app, 
    send_app::{
      send_app, 
      store_wallet, 
      get_iter,
      encode
    },
    store_app::store_app, 
    delete::{
      delete_pda, 
      delete_stored_app
    },
    config_settings::{
      get_config_settings, 
      modify_network
    },
    encrypt::new_key,
};
use pages::{ 
  main_page, 
  config_page, 
  data_domain,
  upload_a_web,
  create_a_domain,
  html,
  js
};
use rocket::{
  fs::{
    relative, 
    FileServer
  }
};
use rocket_dyn_templates::Template;
use std::{process::Command, io};

#[rocket::main]
async fn main() {
  new_key::new_key().unwrap();
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
        .mount("/main", routes![upload_a_web::index])
        .mount("/main/upload_a_web", routes![create_a_domain::index])
        .mount("/main/upload_a_web/create_a_domain", routes![html::index])
        .mount("/main/upload_a_web/create_a_domain/html", routes![js::index])
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
        .mount("/store_web_command", routes![store_web_command::index])
        .mount("/store_app", routes![store_app::index])
        .mount("/delete_stored_app", routes![delete_stored_app::index])
        .mount("/create_app", routes![create_app::index])
        .mount("/delete_pda", routes![delete_pda::index])
        .mount("/send_app", routes![send_app::index])
        .mount("/", FileServer::from(relative!("/public")))
        .attach(Template::fairing())
        .launch()
        .await;
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
}