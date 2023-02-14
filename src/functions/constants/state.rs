use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct State {
    pub wallet: Vec<u8>,
    pub len_html: u16,
    pub len_js: u16,
    pub domain: String,
    pub network: bool, //true = mainnet     false = devnet
    pub html_iter: u16,
    pub js_iter: u16
}