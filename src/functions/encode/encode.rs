use anyhow::{Ok, Result};
use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Encode {
    pub output: String
}

pub fn encode(data: String) -> Result<Encode> {
    let output: String = data.replace('"', "#~").replace('\\', "°¬").replace(',', "#!");
    let encode: Encode = Encode {
        output: output,
    };
    Ok(encode)
}

#[post("/", data = "<string>")]
pub fn index(string: String) -> Json<Encode>  {
    Json(encode(string).unwrap())
}