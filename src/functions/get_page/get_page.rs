/*
    -This function interprets the data from the blockchain and renders the web. 
    First, it checks if the page is saved locally. If it is, it is rendered, if it is not, 
    the app is built. The iteration from the frontend demonstrated higher performance 
    than a native for loop in rust, along with the increase in Rocket.rs workers, 
    rendering time is optimized.
*/

use anchor_client::{
    anchor_lang::{solana_program::hash::hash, Key},
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
    Client, Program,
};
use anyhow::{Result, Ok};
use rocket::serde::{Deserialize, Serialize, json::Json};
use std::{ 
    rc::Rc, str::FromStr, 
    fs, 
    io::Write, 
    fs::{OpenOptions, File}, 
    path::Path
};
use decenwser::state::StoreAccount;
use crate::functions::{
    config_settings::cluster::cluster,
    get_page::{
        get_len::get_len,
        get_domain::get_domain
    },
    constants::program_id
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Get {
    pub html_js: String,
    pub iter: u16
}

pub async fn get_page(html_js: String, iter: u16) {
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
        let program_id: Pubkey =
            Pubkey::from_str(&program_id::ID).unwrap();
        let (pda, _bump) =
            Pubkey::find_program_address(&[&hash(&get_domain().unwrap().as_bytes()).to_bytes()], &program_id);
        let client: Client = Client::new(cluster().unwrap(), Rc::new(Keypair::new()));
        let program: Program = client.program(program_id);
        let content: String = request(html_js.clone(), iter as usize, program_id, program, pda.clone()).unwrap().replace("#~", "\"").replace("#&", "\\").replace("#!", ",");
        slice_app(html_js,content,iter)
 }
}

pub fn slice_app(html_js: String, content:String, iter:u16){
    if html_js == "HTML" {
        let html_path: String = "src/functions/build_app/".to_owned() + &iter.to_string() + ".html";
        let create_html_web: File = OpenOptions::new().create(true).write(true).truncate(true).open(html_path).expect("Error");
        let mut html_web_file: File = create_html_web.try_clone().expect("Error");
        html_web_file.write_all(&content.as_bytes()).unwrap();
        if iter == (get_len().unwrap().len_html - 1_u16) {
            render(html_js.clone()).unwrap();
        }
    }  else if html_js == "JS" {
        let js_path: String = "src/functions/build_app/".to_owned() + &iter.to_string() + ".js";
        let create_js_web: File = OpenOptions::new().create(true).write(true).truncate(true).open(js_path).expect("Error");
        let mut js_web_file: File = create_js_web.try_clone().expect("Error");
        js_web_file.write_all(&content.as_bytes()).unwrap();
        if iter == (get_len().unwrap().len_js - 1_u16) {
            render(html_js.clone()).unwrap();
        }
    }
}

pub fn request(html_js:String, iter: usize, program_id: Pubkey, program: Program, pda:Pubkey) -> Result<String> {
    if html_js == "HTML" {
        let (pbk, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[
            b"HTML",
            iter.to_le_bytes().as_ref(),
            pda.key().as_ref(),
            ], &program_id);
        let html: StoreAccount = program.account(pbk)?;
        return Ok(html.content)
    } else {
        let (pbk, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[
            b"JS",
            iter.to_le_bytes().as_ref(),
            pda.key().as_ref(),
            ], &program_id);
        let js: StoreAccount = program.account(pbk)?;
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

pub fn render(html_js: String) -> Result<()> {
    if html_js == "HTML" {
        let html = OpenOptions::new().create(true).write(true).append(true).open("templates/web.html.hbs").expect("Error");
        for i in 0..(get_len().unwrap().len_html) {
            let path: String = "src/functions/build_app/".to_owned() + &i.to_string() + ".html";
            let mut file: File = html.try_clone().expect("Error");
            loop {
                let c = Path::new(&path);
                if c.exists() {
                    file.write_all(&fs::read(path)?)?;
                    break
                }
            }
        }
    } 
    if html_js == "JS" {
        let js: File = OpenOptions::new().create(true).write(true).append(true).open("public/js.js").expect("Error");
        for i in 0..(get_len().unwrap().len_js) {
            let path: String = "src/functions/build_app/".to_owned() + &i.to_string() + ".js";
            let mut file: File = js.try_clone().expect("Error");
            loop {
                let c = Path::new(&path);
                if c.exists() {
                    file.write_all(&fs::read(path)?)?;
                    break
                }
            }
        }
    }
    Ok(())
}

#[post("/", data = "<domain>")]
pub async fn index(domain: Json<Get>) {
    get_page(domain.html_js.clone(), domain.iter.clone()).await;
}