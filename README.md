<div align="center">
  <img height="170" src="/public/img/Decenwser.ico" />
  <h1 id="title">Decenwser</h1>
  <p>
    <strong>Decentralized browser based on blockchain technology</strong>
  </p>

  <p>
    <a href="http://www.apache.org/licenses/LICENSE-2.0"><img alt="License" src="https://img.shields.io/badge/license-Apache%202.0-blue.svg" /></a>
  </p>
</div>

---

<br>

## Program Ids

<div align="center">

| Program                                                | Devnet                                                                                                                                            | Mainnet Beta |
| ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ |
| [Decenwser](https://github.com/mateolafalce/Decenwser) | [`A9KnHa1iaHtDfsZDNNnM8ZMSmBr6gBFqMHnqw7EyyPir`](https://explorer.solana.com/address/A9KnHa1iaHtDfsZDNNnM8ZMSmBr6gBFqMHnqw7EyyPir?cluster=devnet) | `...`        |

</div>

---

<br>

<p>A decentralized browser that runs locally as a server at http://127.0.0.1:2004. This will ensure that HTML and JavaScript content that has been previously sent to the Solana blockchain enjoy immutable pages that cannot be modified or removed by national or international government institutions. The browser provides an alternative to the modern server-client paradigm, becoming a blockchain-client or local-client in case you want to save the web on your PC.</p>

![Decenwser](/public/img/decenwser.PNG)

---

<br>

## Index

- <a href="#install">**Install the Project 📋**</a>
  - <a href="#install-rust">Install Rust</a>
  - <a href="#install-solana">Install Solana</a>
  - <a href="#install-node">Install Node Js</a>
  - <a href="#source">Download the source</a>
- <a href="#blockchain-program">**Blockchain Program ⛓️**</a>
  - <a href="#main-account"> main_account() </a>
  - <a href="#store">html_store() & js_store()</a>
  - <a href="#add">add_html() & add_js()</a>
- <a href="#decenwser-index">**Decenwser Browser 💻**</a>
  - <a href="#upload-web">How to upload a page to Decenwser?</a>
  - <a href="#important">Important things to keep in mind</a>
  - <a href="#change-network">Change network</a>
  - <a href="#save-web-locally">Save a webpage locally</a>

---

<br>

<h1 id="install"> Install the Project </h1>

<p> Follow the instructions below to be able to install the project and run it on your PC</p>

<h3 id="install-rust"> Install Rust </h3>

Install rust to be able to compile the code on your machine
[here](https://www.rust-lang.org/tools/install)

<h3 id="install-solana"> Install Solana </h3>

Install solana to be able to connect to the blockchain and sign transactions
[here](https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool).

<h3 id="install-node"> Install Node.js </h3>

Install the latest version of node.js to be able to mount decentralized servers
[here](https://nodejs.org/en/download/)

<h3 id="source"> Download the source </h3>

```bash
git clone https://github.com/mateolafalce/Decenwser.git && cd Decenwser && cargo run
```

---

<br>

<h1 id="blockchain-program"> Blockchain Program </h1>

Available [`here`](https://github.com/mateolafalce/Decenwser-blockchain-program)

<p>The application program that runs on the Solana blockchain consists of 7 functions. The first function is dedicated to creating the web domain, the second (html) and third (js) functions are dedicated to creating their respective PDAs for storing content, the fourth (html) and fifth (js) functions are dedicated to modifying the content of the PDA by sending additional data to it, and the sixth and seventh functions are dedicated to deleting content from the web by closing the PDAs and returning the corresponding SOL to the user for upload.</p>
<p>This section will provide more detailed information on each of these functions and their development details.</p>

---

<h3 id="main-account"> main_account() </h3>

<br>

```rust
use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
};
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn main_account(
    ctx: Context<MainAccountStruct>,
    web_name: String
) -> Result<()> {
    require!(web_name.len() <= 32, ErrorCode::TooLong);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    let (_pda, bump) = Pubkey::find_program_address(
          &[&anchor_lang::solana_program::hash::hash(web_name.as_bytes()).to_bytes()
        ],
        ctx.program_id
    );
    main_account.bump_original = bump;
    main_account.web_name = web_name;
    main_account.authority = ctx.accounts.signer.key();
    main_account.html = 0;
    main_account.js = 0;
    msg!(
        "{} is part of the international decentralized information interchange. Authority: {}",
        main_account.web_name,
        ctx.accounts.signer.key()
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(web_name: String)]
pub struct MainAccountStruct<'info> {
    #[account(init,
        seeds = [
              &anchor_lang::solana_program::hash::hash(web_name.as_bytes()).to_bytes()
            ],
            bump,
            payer = signer,
            space = MainAccount::SIZE + 8
        )
    ]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

<p>This function creates the basis of the user's domain on the network. Initially, any Solana network user can use this account without any special permission.</p>

<p>A PDA is created to store important data for each code update, data retrieval, or deletion. The transaction signer is saved as the account authority to ultimately emit a message in the transaction providing proof of what was saved.</p>

<p>Then two crucial values are created for content storage. The value corresponding to the size of the HTML or JS after each iteration will be saved there<u16>.</p>

> Previously, an alternative architecture was experimented with, in which the size of the HTML and JS was a u64 that increased in value based on the content increase. However, this model was abandoned as it stored an excessively large amount of data for the web, with a limit of 1.660207e+22 compared to the current model's 648,796,500 characters.

<p>It should be noted that the domain itself (the PDA) cannot be removed from the network, but all of its content stored within it can be. This decision is focused on limiting external interventions as much as possible in the websites rendered by Decenwser.</p>

![inputs-main](/public/img/inputs-main.PNG)
![create-account-main](/public/img/create-account-main.PNG)
![main-account-logs](/public/img/main-account-logs.PNG)

---

<h3 id="store">html_store() & js_store()</h3>

<br>

```rust
use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
};
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn html_store(
    ctx: Context<HtmlStore>,
    content: Vec<u8>
) -> Result<()> {
    require!(
        ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(),
        ErrorCode::AuthorityError
    );
    let (_pda, bump) = Pubkey::find_program_address(&[
        b"HTML",
        ctx.accounts.main_account.html.to_le_bytes().as_ref(),
        ctx.accounts.main_account.key().as_ref()
        ],
        ctx.program_id);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    let store: &mut Account<StoreAccount> = &mut ctx.accounts.store;
    store.content = content;
    store.bump_original = bump;
    msg!(
        "New account stores the HTML content with seed = [HTML, {}, {}]",
        main_account.html,
        main_account.key()
    );
    Ok(())
}

#[derive(Accounts)]
pub struct HtmlStore<'info> {
    #[account(
        mut,
        seeds = [
            &anchor_lang::solana_program::hash::hash(
                main_account.web_name.as_bytes()
            ).to_bytes()
            ],
        bump = main_account.bump_original
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(
        init,
        seeds =
            [
            b"HTML",
            main_account.html.to_le_bytes().as_ref(),
            main_account.key().as_ref()
            ],
        bump, payer = signer,
        space = StoreAccount::SIZE + 8
    )]
    pub store: Account<'info, StoreAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

<p>The storage of HTML and JavaScript content is carried out thanks to the PDAs offered by Solana. These are accounts derived from the program that allow us to make use of this decentralized architecture.</p>

<p>The function js_store() has the same logic as html_store(), but passes "JS" as bytes in the PDA.
An array of bytes is taken as an argument, which is a string converted to bytes. This is done in order to avoid Rust special characters (quotes, slashes, etc...) and Solana special characters (anything that is not UTF-8 text), which allows users to upload their applications using non-UTF-8 characters such as emojis or Asian and African alphabets.</p>

<p>Then, as a security measure, only the domain authority is able to update the website with new content. The PDA data (content and bump) is saved, and finally a message is emitted to announce the data used to create the account.</p>

> The functions are divided into two to use the available runtime data and avoid using external variables that could cause errors when initializing the account.

![store-input](/public/img/store-input.PNG)
![create-store](/public/img/create-store.PNG)
![logs-store](/public/img/logs-store.PNG)

---

<br>

<h3 id="add">add_html() & add_js()</h3>

<br>

```rust
use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
};
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn add_html(
    ctx: Context<AddHtml>,
    content: Vec<u8>
) -> Result<()> {
    require!(
        ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(),
        ErrorCode::AuthorityError
    );
    require!(
        ctx.accounts.store.content.len() < 9900,
        ErrorCode::Max9900
    );
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    let store: &mut Account<StoreAccount> = &mut ctx.accounts.store;
    store.content.extend(content);
    msg!("The content of the PDA was updated.");
    if store.content.len() == 9900 {
        main_account.html += 1;
    }
    Ok(())
}

#[derive(Accounts)]
pub struct AddHtml<'info> {
    #[account(
        mut,
        seeds = [
            &anchor_lang::solana_program::hash::hash(
                main_account.web_name.as_bytes()
            ).to_bytes()
            ],
        bump = main_account.bump_original
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(
        mut,
        seeds =
            [
            b"HTML",
            main_account.html.to_le_bytes().as_ref(),
            main_account.key().as_ref()
            ],
        bump = store.bump_original,
        realloc = 8 + 4 + 1 + store.content.len() + 900,
        realloc::payer = signer,
        realloc::zero = false,
    )]
    pub store: Account<'info, StoreAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

<p>These two functions are intended to update the content of the PDA by increasing the storage space. First, it is verified that the authority is the owner, and then the maximum allowed in the PDA is checked, which in this case has been determined to be 9900 bytes of html/js content. After the value is stored, a global message is emitted to verify the update, and after all of the above has been completed, it is checked if the data volume has reached its maximum, and if so, the .html or .js of the account is increased by one to create another PDA and continue with the data upload process.</p>

> In version 0.2.4, this type of architecture was implemented, which allows for great performance when rendering the webpage.

![add-input](/public/img/add-input.PNG)
![add-transfer](/public/img/add-transfer.PNG)
![add-logs](/public/img/add-logs.PNG)

---

<br>

<h3 id="delete"> delete_html() & delete_js()</h3>

```rust
use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
};
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn delete_html(
    ctx: Context<DeleteHtml>
) -> Result<()> {
    require!(ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(), ErrorCode::AuthorityError);
    let lamport: u64 = ctx.accounts.account.to_account_info().lamports() - 890880;
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    main_account.html -= 1;
    **ctx.accounts.account.to_account_info().try_borrow_mut_lamports()? -= lamport;
    **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? += lamport;
    msg!(
        "{} account removes HTML content and removes {} lamports from {} account",
        ctx.accounts.signer.key(),
        lamport,
        ctx.accounts.account.key()
    );
    Ok(())
}
#[derive(Accounts)]
pub struct DeleteHtml<'info> {
    #[account(
        mut,
        seeds = [
            &anchor_lang::solana_program::hash::hash(
                main_account.web_name.as_bytes()
            ).to_bytes()
        ],
        bump = main_account.bump_original
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(
        mut,
        seeds = [
            b"HTML",
            main_account.html.to_le_bytes().as_ref(),
            main_account.key().as_ref()
        ],
        bump = account.bump_original,
        close = signer
    )]
    pub account: Account<'info, StoreAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

<p>This function deletes the content of the PDA and returns the SOL to the creator. The function verifies that the signer is the authority, then obtains the SOL from the account and proceeds to remove the lamports from the PDA. Finally, a message is issued disclosing the data used to delete that account
</p>

![delete-input](/public/img/delete-input.PNG)
![delete-accounts](/public/img/delete-accounts.PNG)
![delete-logs](/public/img/delete-logs.PNG)

---

<br>

<h1 id="decenwser-index">Decenwser Browser</h1>

<p>In this section they will analyze all aspects related to the operation of the browser. We will talk about the justification of the general development framework and its performance.</p>

<br>

---

<h3 id="upload-web"> How to upload a page to Decenwser?</h3>
<p>To upload a page, all you need to do is download the browser, compile the HTML and JavaScript into two different files, and go to "Upload a web" in the top bar.

- First, the code is converted to bytes so that it can be sent to the Solana blockchain.
- Second, the user is asked to create a domain in which to upload the HTML and JavaScript. In this process, the wallet will be requested to sign this transaction and subsequent ones.
- Third, the HTML is sent to the blockchain.
- Fourth, the JavaScript is sent to the blockchain.

<p>The process is very straightforward and intuitive to execute. That said, it's worth noting that the data upload process often takes too long. A page made with React and compiled with webpack, with an output of about 400,000 lines of code in total, would take between 3:00 to 4:30 hours to complete the upload, and this could be affected by the user's internet connectivity.</p>

<h3 id="important">Important things to keep in mind</h3>

<p>When developing an app for Decenwser, the .js file should be called from the HTML as a </p>

```html
<script src="../js.js"></script>
```

<p>before the root div.</p>

<p>You have to add a CDN link depending on the framework you are working with, like React for example.</p>

```html
<script
  crossorigin
  src="https://unpkg.com/react@18/umd/react.production.min.js"
></script>
<script
  crossorigin
  src="https://unpkg.com/react-dom@18/umd/react-dom.production.min.js"
></script>
```

<p>Webpack and Babel should be configured to compile the code and produce 2 outputs in production mode. Here's a template so you only need to write the web logic

[link here](https://github.com/mateolafalce/template-decenwser-app)

It includes a React CDN link, but you can use any.</p>

> The wallet is only stored in the application signing process and in case of modifying or deleting content. After that, it is cleaned of the data by the system program.

<p>The images in Decenwser must be imported from either traditional servers or decentralized servers. Currently, storing images passed to base64 on the blockchain is very expensive. If a user chooses to do so, they do it at their own risk. From banners to the favicon, they must be externally imported with links into the application's source code running on Decenwser.</p>

---

<h3 id="change-network">Change Networks</h3>

<p>To change between Devnet and Mainnet, you can access the main dashboard and change the network you want to navigate in just one click. By changing networks, the entire application state will adjust to your needs for both development and navigation</p>
<div align="center">
  
  ![change-network](/public/img/change-network.PNG)
  
</div>

---

<h3 id="save-web-locally">Save a webpage locally</h3>

<p>To avoid having to wait for the content to load directly from the blockchain, you can store the app locally in your browser. Simply press Ctrl + d to access the page immediately on your next search.</p>
<p>This function stores the content of the page at the time of saving. If the source code of the page is updated (not necessarily the content), you will need to remove the app from the browser by going to the apps tab on the homepage.</p>

---

<br>

## get_page() function performance.

This takes the speed for each pda fetched from the blockchain and rendered in the app.

<div align="center">

| Version | Speed      |
| ------- | ---------- |
| 0.1.0   | 2278.01 ms |
| 0.1.1   | 1087.94 ms |
| 0.2.0   | 724.38 ms  |
| 0.2.1   | 463.14 ms  |
| 0.2.2   | 283.75 ms  |
| 0.2.3   | 343.09 ms  |
| 0.2.4   | 108.26 ms  |

</div>

> The current architecture is the one implemented in version 0.2.4, which came to solve a problem of iterational volume with the PDAs at the time of rendering, together with the fact that they stopped passing strings, instead passing bytes.
