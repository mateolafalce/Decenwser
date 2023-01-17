use anchor_client::{
    anchor_lang::solana_program::hash::hash,
    anchor_lang::system_program,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signature},
    },
    Client, Cluster, Program,
};
use anyhow::{Ok, Result};
use decenwser::{DecenwserAccount, MainAccount};
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SendHtml {
    pub html: String,
}

pub fn send_html(html: String, web_name: String) -> Result<SendHtml> {
    let program_id: Pubkey =
        Pubkey::from_str("5t8N3VyXYMRYvuRQtmTZ3HmmRYK7En9ZHXVLLQZBC4gr").unwrap();
    let (pda, _bump) =
        Pubkey::find_program_address(&[&hash(web_name.as_bytes()).to_bytes()], &program_id);
    let client: Client = Client::new(Cluster::Devnet, Rc::new(Keypair::new()));
    let program: Program = client.program(program_id);
    let app_data: MainAccount = program.account(pda)?;
    let program = Client::new(
        Cluster::Devnet,
        Rc::new(
            read_keypair_file(&*shellexpand::tilde(
                "C:/Users/Mateo/.config/solana/id.json",
            ))
            .expect("Example requires a keypair file"),
        ),
    )
    .program(program_id);

    html_store(&program, html, web_name).unwrap();

    let html_tx: SendHtml = SendHtml {
        html: "foo".to_string(),
    };
    Ok(html_tx)
}

pub fn html_store(program: &Program, html: String, web_name: String) -> Result<String> {
    let (main_account, _bump) =
        Pubkey::find_program_address(&[&hash(web_name.as_bytes()).to_bytes()], &program.id());
    let (decenwser, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[b"Decenwser"], &program.id());
    let account: DecenwserAccount = program.account(decenwser)?;
    let (html_store, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[&account.total_updates.to_le_bytes()], &program.id());
    let tx: Signature = program
        .request()
        .accounts(decenwser::accounts::HtmlStore {
            main_account,
            decenwser,
            html_store,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::HtmlStore { html })
        .send()?;
    Ok(tx.to_string())
}

/*#[post("/", data = "<web>")]
pub fn index(html: String, web: String) -> Json<SendHtml> {
    Json(send_html(html, web).unwrap())
}*/
