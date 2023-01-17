use anchor_client::{
    anchor_lang::solana_program::hash::hash,
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
    Client, Cluster, Program,
};
use anyhow::{Ok, Result};
use rocket::serde::{Deserialize, Serialize, json::Json};
use std::rc::Rc;
use std::str::FromStr;
use decenwser::{MainAccount, HTML, CSS, JS};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct App {
    pub html: String,
    pub css: String,
    pub js: String
}

pub fn get_page(domain: String) -> Result<App> {
    let program_id: Pubkey =
        Pubkey::from_str("5t8N3VyXYMRYvuRQtmTZ3HmmRYK7En9ZHXVLLQZBC4gr").unwrap();
    let (pda, _bump) =
        Pubkey::find_program_address(&[&hash(domain.as_bytes()).to_bytes()], &program_id);
    let client: Client = Client::new(Cluster::Devnet, Rc::new(Keypair::new()));
    let program: Program = client.program(program_id);
    let app_data: MainAccount = program.account(pda)?;
    let mut html_content: String = "".to_string();
    let mut css_content: String = "".to_string();
    let mut js_content: String = "".to_string();
    let mut html_iter: usize = 0;
    let mut css_iter: usize = 0;
    let mut js_iter: usize = 0;
    let html_len: usize = app_data.html.len();
    let css_len: usize = app_data.css.len();
    let js_len: usize = app_data.js.len();
    while html_iter < html_len {
        let domain_html = domain.clone();
        html_iter += 1;
        html_content += &html_request(html_iter, domain_html)?.replace("#~", "\"").replace("°¬", "\\").replace("#!", ",");
    }
    while css_iter < css_len {
        let domain_css = domain.clone();
        css_iter += 1;
        css_content += &css_request(css_iter, domain_css)?.replace("#~", "\"").replace("°¬", "\\").replace("#!", ",");
    }
    while js_iter < js_len {
        let domain_js = domain.clone();
        js_iter += 1;
        js_content += &js_request(js_iter, domain_js)?.replace("#~", "\"").replace("°¬", "\\").replace("#!", ",");
    }
    let app: App = App {
        html: html_content,
        css: css_content,
        js: js_content
    };
    Ok(app)
}
pub fn html_request(html_iter: usize, domain: String) -> Result<String> {
    let program_id: Pubkey =
        Pubkey::from_str("5t8N3VyXYMRYvuRQtmTZ3HmmRYK7En9ZHXVLLQZBC4gr").unwrap();
    let client: Client = Client::new(Cluster::Devnet, Rc::new(Keypair::new()));
    let program: Program = client.program(program_id);
    let (pda, _bump) =
        Pubkey::find_program_address(&[&hash(domain.as_bytes()).to_bytes()], &program_id);
    let app_data: MainAccount = program.account(pda)?;
    let (html_pbk, _bump) =
    Pubkey::find_program_address(&[&app_data.html[html_iter - 1].to_le_bytes()], &program_id);
    let html: HTML = program.account(html_pbk)?;
    Ok(html.html)
}
pub fn css_request(css_iter: usize, domain: String) -> Result<String> {
    let program_id: Pubkey =
        Pubkey::from_str("5t8N3VyXYMRYvuRQtmTZ3HmmRYK7En9ZHXVLLQZBC4gr").unwrap();
    let client: Client = Client::new(Cluster::Devnet, Rc::new(Keypair::new()));
    let program: Program = client.program(program_id);
    let (pda, _bump) =
        Pubkey::find_program_address(&[&hash(domain.as_bytes()).to_bytes()], &program_id);
    let app_data: MainAccount = program.account(pda)?;
    let (css_pbk, _bump) =
    Pubkey::find_program_address(&[&app_data.css[css_iter - 1].to_le_bytes()], &program_id);
    let css: CSS = program.account(css_pbk)?;
    Ok(css.css)
}
pub fn js_request(js_iter: usize, domain: String) -> Result<String> {
    let program_id: Pubkey =
        Pubkey::from_str("5t8N3VyXYMRYvuRQtmTZ3HmmRYK7En9ZHXVLLQZBC4gr").unwrap();
    let client: Client = Client::new(Cluster::Devnet, Rc::new(Keypair::new()));
    let program: Program = client.program(program_id);
    let (pda, _bump) =
        Pubkey::find_program_address(&[&hash(domain.as_bytes()).to_bytes()], &program_id);
    let app_data: MainAccount = program.account(pda)?;
    let (js_pbk, _bump) =
    Pubkey::find_program_address(&[&app_data.js[js_iter - 1].to_le_bytes()], &program_id);
    let js: JS = program.account(js_pbk)?;
    Ok(js.js)
}
#[post("/", data = "<domain>")]
pub fn index(domain: String) -> Json<App> {
    Json(get_page(domain).unwrap())
}