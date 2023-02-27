/*
    -This function interprets the data from the blockchain and renders the web. 
    First, it checks if the page is saved locally. If it is, it is rendered, if it is not, 
    the app is built.
*/

use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signature::{keypair_from_seed}},
    Client, Program,
};
use anyhow::{Result, Ok};
use rocket::serde::json::from_str;
use std::{ 
    rc::Rc, str::FromStr, 
    fs, 
    io::Write, 
    fs::{write, read_to_string, File, OpenOptions,},
    path::Path
};
use decenwser::state::StoreAccount;
use crate::functions::{
    config_settings::cluster::cluster,
    send_app::{
        store_iter::store_iter,
        get_wallet::get_wallet
    },
    get_page::{
        get_len::get_len,
        get_domain::get_domain
    },
    constants::{program_id, pdas::Pdas}
};

pub async fn get_page() {
    let check: String = "apps/".to_owned() + &get_domain().unwrap() + ".html";
    if verify(check).unwrap() == true {
        let from_path: [String;2] = [("apps/".to_owned() + &get_domain().unwrap() + ".html").to_string(), ("apps/".to_owned() + &get_domain().unwrap() + ".js").to_string()];
        let to_path: [String;2] = ["templates/web.html.hbs".to_string(), "public/js.js".to_string()];
        for i in 0..2 {
            let file: File = OpenOptions::new().create(true).write(true).truncate(true).open(to_path[i].to_owned()).expect("Error");
            let mut write_file: File = file.try_clone().expect("Error");
            write_file.write_all(&fs::read(from_path[i].to_owned()).unwrap()).unwrap(); 
        }
    } else {
        let mut html: String = String::new();
        let mut js: String = String::new();
        for i in 0..get_len().unwrap().len_html {
            let program_id: Pubkey =
            Pubkey::from_str(&program_id::ID).unwrap();
            let client: Client = Client::new(cluster().unwrap(), Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")));
            let program: Program = client.program(program_id);
            html.push_str(
                &String::from_utf8_lossy(
                    &request("HTML".to_string(), i as usize, program).unwrap()
                )
                .to_string()
            );
            render("HTML".to_string(), html.clone()).unwrap();
        }
        for i in 0..get_len().unwrap().len_js {
            let program_id: Pubkey =
            Pubkey::from_str(&program_id::ID).unwrap();
            let client: Client = Client::new(cluster().unwrap(), Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")));
            let program: Program = client.program(program_id);
            js.push_str(
                &String::from_utf8_lossy(
                    &request("JS".to_string(), i as usize, program).unwrap()
                )
                .to_string()
            );
            render("JS".to_string(), js.clone()).unwrap();
        }
 }
}

pub fn request(html_js:String, iter: usize, program: Program) -> Result<Vec<u8>> {
    if html_js == "HTML" {
        let html_pda: Pdas = from_str(&read_to_string("src/functions/get_page/html_pdas.json").unwrap()).unwrap();
        let pda: Pubkey = Pubkey::from_str(&html_pda.pdas[iter]).unwrap();
        let html: StoreAccount = program.account(pda)?;
        store_iter(true).unwrap();
        return Ok(html.content)
    } else {
        let js_pda: Pdas = from_str(&read_to_string("src/functions/get_page/js_pdas.json").unwrap()).unwrap();
        let pda: Pubkey = Pubkey::from_str(&js_pda.pdas[iter]).unwrap();
        let js: StoreAccount = program.account(pda)?;
        store_iter(false).unwrap();
        return Ok(js.content)
    }
}

pub fn verify(path:String) -> Result<bool> {
    let file_path: &Path = Path::new(&path);
    if file_path.exists() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn render(html_js: String, content: String) -> Result<()> {
    if html_js == "HTML" {
        write("templates/web.html.hbs", content).unwrap();
    } else {
        write("public/js.js", content).unwrap();
    }
    Ok(())
}

#[post("/")]
pub async fn index() {
    get_page().await;
}