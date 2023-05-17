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

<h3 align="center">Program Ids</h3>

<div align="center">

| Program                                                | Devnet                                                                                                                                            | Mainnet Beta |
| ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ |
| [Decenwser](https://github.com/mateolafalce/Decenwser) | [`A9KnHa1iaHtDfsZDNNnM8ZMSmBr6gBFqMHnqw7EyyPir`](https://explorer.solana.com/address/A9KnHa1iaHtDfsZDNNnM8ZMSmBr6gBFqMHnqw7EyyPir?cluster=devnet) | `...`        |

</div>

---


Web 3.0 has arrived to revolutionize the way we navigate the Internet and interact online. With decentralization as its main feature, this new stage of the web promises to offer a more secure, transparent and equitable experience for all users. In this context, Decenwser is a browser that emerges as an innovative alternative to the traditional server-client model.

Decenwser is a decentralized browser that runs locally as a server at http://127.0.0.1:2004. Its main purpose is to guarantee the immutability of HTML and JavaScript content that has been previously stored on the Solana blockchain. This means that any web page that has been registered on the blockchain cannot be modified or removed by national or international government institutions. In this way, the privacy and security of the information we share online is ensured.

In addition, Decenwser is presented as an alternative to the current server-client model. Instead of relying on remote servers, the browser acts as a blockchain client or local client, which means that users can save web information on their own computers. This guarantees the independence of the user and the reduction of the power of the big technological corporations.

Among the benefits of Decenwser is the ability to regain control over our online data and the ability to browse the web without restrictions or censorship. In addition, by using blockchain technology, the browser offers greater transparency and security in data management, avoiding any type of tampering or manipulation.

Decenwser is a tool that promises to change the way we interact with the web and interact online. With its decentralized approach and its ability to store information securely and immutably, it allows us to regain trust in technology and ensure privacy and independence online.

![Decenwser](/public/img/decenwser.PNG)

---

<details>
<summary>Index</summary>

<br>

- <a href="#install">**Install the Project 📋**</a>
  - <a href="#install-rust">Install Rust </a>
  - <a href="#install-solana">Install Solana</a>
  - <a href="#install-node">Install Node Js</a>
  - <a href="#source">Download the source </a>
- <a href="#blockchain-program">**Blockchain Program ⛓️**</a>
  - <a href="#main-account"> Create a domain</a>
  - <a href="#store"> Store the content of the web page</a>
  - <a href="#add">Add more content to the web </a>
  - <a href="#delete">Remove content from the web</a>
- <a href="#decenwser-index">**Decenwser Browser 💻**</a>
  - <a href="#upload-web">How to upload a page to Decenwser?</a>
  - <a href="#important">Important things to keep in mind</a>
  - <a href="#change-network">Change network</a>
  - <a href="#save-web-locally">Save a webpage locally</a>
  - <a href="#get_page">get_page()</a>

 </details>

---

<details>
<summary>Install the project📋</summary>

<br>

If you're looking to install and run a project on your PC, it's important to follow a set of instructions to ensure everything runs smoothly. Below are some guidelines to help you get started


<h2> Installation Guide </h2>
To be able to install and run the project on your PC, you'll need to follow a set of instructions. Below are some guidelines to help you get started.

<h3 id="install-rust"> Install Rust 🦀 </h3>
Rust is required to compile the code on your machine. To install Rust, follow the steps below:

Go to the [official Rust website](https://www.rust-lang.org/tools/install)and select your operating system.

Follow the instructions provided to download and install Rust.

Verify that Rust has been installed correctly by running the following command in your terminal:

```bash
rustc --version
```
This should print out the version of Rust that you just installed.

<h3 id="install-solana"> Install Solana 🌞 </h3>
Solana is required to connect to the blockchain and sign transactions. To install Solana, follow the steps below:

Go to the [Solana CLI installation page](https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool)..

Follow the instructions provided to download and install the Solana CLI.

Verify that Solana has been installed correctly by running the following command in your terminal:

```bash
solana --version
```
This should print out the version of Solana that you just installed.

<h3 id="install-node"> Install Node.js 🚀 </h3>
Node.js is required to mount decentralized servers. To install the latest version of Node.js, follow the steps below:

Go to the [official Node.js website](https://nodejs.org/en/download/) and select your operating system.

Follow the instructions provided to download and install Node.js.

Verify that Node.js has been installed correctly by running the following command in your terminal:

```bash
node --version
```
This should print out the version of Node.js that you just installed.

<h3 id="source"> Download the source 🗂️ </h3>
To download the source code and run the project, follow the steps below:

Clone the Decenwser repository by running the following command in your terminal:

```bash
git clone https://github.com/mateolafalce/Decenwser.git
```
Change into the Decenwser directory by running the following command in your terminal:

```bash
cd Decenwser
```
Build and run the project by running the following command in your terminal:

```bash
cargo run
```
This should build and run the project, allowing you to interact with the decentralized servers.

 </details>

---

<details>
<summary>Blockchain program⛓️</summary>

<br>

Available [`here`](https://github.com/mateolafalce/Decenwser-blockchain-program)

The constant evolution of technology has given rise to a new paradigm that is revolutionizing the way we interact with the web. The web3 and decentralization are two concepts that have become the basis of a new digital age, where privacy and security are the priority.

In this context, the Decenwser browser has become a key tool to take advantage of the benefits of web3 and decentralization. This browser is designed to interact directly with the Solana blockchain network and its programmatic application, which consists of seven fundamental functions.

The first function is dedicated to the creation of the web domain, which allows establishing a unique and decentralized identity for the site. Functions two and three (html and js) focus on creating the respective PDAs (Permanent Data Files) that store the page content securely and reliably on the blockchain network.

Functions four and five (html and js) allow the modification of the PDA content by adding new data, which guarantees the constant updating of the site. Functions six and seven are dedicated to removing content from the web, allowing users to close their PDAs and receive the corresponding SOL (Solana's cryptocurrency) as a reward for their contribution to the network.

The Decenwser browser has become an essential tool for those looking to reap the benefits of web3 and decentralization. By allowing direct access to the Solana blockchain network and its programmatic application, users can enjoy a more secure and private online experience. In addition, the use of decentralized PDAs ensures the immutability of the page content and resistance to censorship, which guarantees that the content is available forever.

In summary, web3 and decentralization represent a new era of the web, where security, privacy and freedom are the main values. Decenwser has positioned itself as a key browser to reap these benefits, making it a must-have for users looking for a safer and more reliable online experience.

---

<h3 id="main-account" align="center"> Create a domain 💻</h3>

```rust
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

This function is designed to create the foundation for a user's domain on the network. The process begins by creating a PDA (Program Derived Address) which is used to store important data related to each code update, data retrieval, or deletion. This PDA is created using the web_name parameter passed into the function, which represents the name of the web domain being created.

The function checks that the length of the web_name parameter is no greater than 32 characters, to ensure compatibility with the Solana blockchain. Once the PDA is created, the transaction signer is saved as the account authority to ultimately emit a message in the transaction providing proof of what was saved.

Two crucial values are then created for content storage. The value corresponding to the size of the HTML or JS after each iteration will be saved there as a u16. This ensures that the amount of data stored is manageable and optimized for performance.

> It's worth noting that an alternative architecture was previously experimented with, where the size of the HTML and JS was a u64 that increased in value based on the content increase. However, this model was abandoned as it stored an excessively large amount of data for the web, with a limit of 1.660207e+22 compared to the current model's 648,796,500 characters.

The user's main account is then created and initialized with the PDA, and the values for HTML and JS are both set to 0. Finally, a message is emitted in the transaction indicating that the domain has been successfully created and that it is now part of the international decentralized information interchange.

It's important to note that while the domain itself (the PDA) cannot be removed from the network, all of its content stored within it can be. This decision was made to limit external interventions as much as possible in the websites rendered by Decenwser.

Overall, this smart contract provides a secure and efficient way for users to create and manage their own domains on the Solana blockchain. By leveraging the power of decentralization, users can enjoy increased control and autonomy over their online presence, while also benefiting from improved security and performance.

<div align="center">

![inputs-main](/public/img/inputs-main.PNG)

![create-account-main](/public/img/create-account-main.PNG)

![main-account-logs](/public/img/main-account-logs.PNG)

</div>

</details>

---

<details>
<summary>Store the content of the web page📝</summary>

<br>

```rust
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

The HTML and JavaScript content storage in the Solana blockchain is made possible through the use of Programmable Data Accounts (PDAs), which offer a decentralized architecture for secure data storage. The html_store() function within the smart contract code is responsible for storing HTML content on the blockchain. It takes in a vector of bytes as an argument, which is a string converted to bytes in order to prevent issues with Rust special characters and non-UTF-8 characters such as emojis or non-Latin alphabets.

To ensure the security of the data, only the domain authority is able to update the website with new content. The PDA data, which includes the content and bump, is then saved in the account. The bump is a value generated by the program that is used to prevent replay attacks.

Similarly, the js_store() function is responsible for storing JavaScript content on the blockchain, and it follows the same logic as html_store(), but passes "JS" as bytes in the PDA.

It is worth noting that the functions are divided into two parts to use the available runtime data and avoid using external variables that could cause errors when initializing the account. This helps to ensure that the account initialization process is smooth and secure.

> Overall, the use of Solana's PDAs for content storage provides a number of benefits, including decentralization, security, and flexibility in terms of data formats. By leveraging this architecture, users can trust that their content is safe and accessible, while also enjoying the benefits of a decentralized network.

<div align="center">

![store-input](/public/img/store-input.PNG)

![create-store](/public/img/create-store.PNG)

![logs-store](/public/img/logs-store.PNG)

</div>

</details>

---

<details>
<summary>Add more content to the web📤</summary>

<br>


```rust
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

The function add_html is a crucial part of the program that is responsible for updating the content of the PDA on the Solana blockchain. This function provides an efficient and effective way of increasing the storage space of the PDA by adding HTML/JS content to it. The function is implemented in Rust and consists of several steps that ensure the security and integrity of the data being stored.

Firstly, the function verifies that the authority is the owner of the PDA by checking the signer key. Once the ownership is verified, the function checks the maximum allowed size of the PDA, which has been set at 9900 bytes of HTML/JS content. If the content to be added exceeds the maximum size, an error is returned.

After verifying the ownership and maximum size of the PDA, the content is added to the PDA, and a global message is emitted to confirm the update. Furthermore, the function checks if the data volume has reached its maximum. If so, the .html or .js of the account is increased by one, which creates another PDA, and the data upload process continues seamlessly.

> This type of architecture was implemented in version 0.2.4, which allows for great performance when rendering the webpage. The function is part of a larger program that runs on the Solana blockchain and has several other functions that work together to create and manage the web domain. The program's main goal is to provide an efficient and decentralized way of hosting web content on the blockchain, which brings numerous benefits such as increased security, accessibility, and censorship resistance.

<div align="center">

![add-input](/public/img/add-input.PNG)

![add-transfer](/public/img/add-transfer.PNG)

![add-logs](/public/img/add-logs.PNG)

</div>

</details>

---

<details>
<summary>Remove content from the web❌</summary>

<br>

```rust
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

The function described in this code focuses on deleting the contents of a PDA (Permanent Data Files) and returning SOLs (Solana's cryptocurrency) to the creator. This process is carried out through a series of verifications and actions that ensure the security and integrity of the Solana blockchain network.

The first check performed in this function is the authority of the signer, which ensures that the person performing the action has the necessary permissions to do so. Once the authority is verified, we proceed to obtain the SOLs from the PDA and subtract them from the corresponding account.

To carry out this operation, the information stored in the main account and in the PDA account is used, including the seed, the bump and the address of the accounts. In addition, a SOL transaction takes place between the signer's account and the PDA's account, ensuring that SOLs are returned to the creator safely and reliably.

Finally, a message is issued informing users about the details of the operation performed. This includes the address of the signer, the number of SOLs removed, and the address of the corresponding PDA account.

This function is a critical part of the programmatic application that runs on the Solana blockchain network. Its goal is to ensure the security and privacy of users by removing content from PDAs and reliably returning SOLs to their creator. By following a series of carefully designed checks and actions, this feature ensures that content removal is done effectively and securely, making it an essential part of the programmatic application on the Solana blockchain network.

<div align="center">

![delete-input](/public/img/delete-input.PNG)

![delete-accounts](/public/img/delete-accounts.PNG)

![delete-logs](/public/img/delete-logs.PNG)

</div>

</details>

---

<details>
<summary>Decenwser browser concept💻</summary>

<br>

In this section, all aspects related to the operation of the browser will be analyzed. The rationale for the general development framework used and its performance will be addressed. The key features of the browser will be examined and its advantages and disadvantages in relation to other browsers will be discussed. The different components of the browser will also be explored, including its rendering engine and its Node.js and Rocket.rs engine, as well as the underlying technologies that make them possible. In general, this section will provide a detailed insight into the inner workings of the browser and help users better understand its operation and performance.

</details>

---

<details>
<summary>How to upload a page to Decenwser?👀</summary>

<br>

To upload a web page using Decenwser, you will first need to download the browser and compile the HTML and JavaScript files into two separate files. Once you have completed this, you can go to the "Upload a web" option located in the top bar of the Decenwser browser.

The upload process involves the following steps:

- The code is converted into bytes, enabling it to be sent to the Solana blockchain.
- The user is then prompted to create a unique domain to host the uploaded HTML and JavaScript files. During this process, the wallet will be required to sign the transaction and any subsequent ones.
- Next, the HTML file is sent to the blockchain.
- Finally, the JavaScript file is sent to the blockchain.

Overall, the upload process is simple and intuitive to execute. However, it's important to note that uploading data to the blockchain can sometimes take a while. For example, a page created with React and compiled with webpack, containing approximately 400,000 lines of code, may take between 3 to 4.5 hours to upload, depending on the user's internet connectivity. Despite this, the benefits of using a decentralized platform like Decenwser far outweigh the temporary inconvenience of a longer upload time, as it provides a secure and trustless way to host your content without relying on centralized intermediaries.

</details>

---

<details>
<summary>Important things to keep in mind❗</summary>

<br>

When you are developing an app for Decenwser, the .js file should be called from the HTML as a

```html
<script src="../js.js"></script>
```
This line of code is crucial for your application to run correctly on Decenwser's platform. By calling the .js file from the HTML, you ensure that all elements of your application load correctly and communicate effectively.

It is important to note that the .js file must be located in the same folder as the corresponding HTML file. Otherwise, the Decenwser platform will not be able to find and load the .js file, which could lead to errors and crashes in the application.

In conclusion, by calling the .js file from the HTML properly, you are ensuring that your application works effectively on Decenwser's platform. Therefore, it is important to follow these instructions to ensure success in developing your app on this decentralized platform.

You have to add a CDN link depending on the framework you are working with, like React for example.

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

To compile the code and produce two outputs in production mode, it is necessary to configure Webpack and Babel appropriately. Here we present a template so that you only have to write the corresponding web logic.

Webpack is a tool used to package and compile the source code of a web application into a set of smaller, browser-optimized files. To do this, it is necessary to correctly configure the Webpack configuration file, so that the inputs, outputs, and different modules to be used are specified.

For its part, Babel is a code compiler used to convert modern JavaScript code to a version compatible with older browsers. To do this, you must install the necessary plugins and presets and configure the Babel configuration file according to the needs of the application.

Once Webpack and Babel have been properly configured, you must ensure that two outputs are produced in production mode. This can be achieved by configuring Webpack to generate two different output files, one for the JavaScript code and one for the CSS styles. These files can be minified and optimized for faster page load.

[link here](https://github.com/mateolafalce/template-decenwser-app)

It includes a React CDN link, but you can use any.

> The wallet is only stored in the application signing process and in case of modifying or deleting content. After that, it is cleaned of the data by the system program.

The images in Decenwser must be imported from either traditional servers or decentralized servers. Currently, storing images passed to base64 on the blockchain is very expensive. If a user chooses to do so, they do it at their own risk. From banners to the favicon, they must be externally imported with links into the application's source code running on Decenwser.

</details>

---
<details>
<summary>Change Network♻️</summary>

If you want to switch between Devnet and Mainnet in the app, you can do it through the main panel. Just select the network you want to navigate with a single click. When you switch networks, all of the app's state will automatically adjust to your needs, whether it's for development or browsing.

This process is very simple and easy to perform, and it allows you to switch between both networks without problems. In addition, by doing so, you will be able to access all the functionalities and tools available in each one of them.

By changing networks, you will also be able to test your application in different environments, allowing you to ensure that it works correctly in all possible scenarios. This is especially important if you are developing an application for use on the blockchain network, as you will need to ensure its compatibility and functionality in all possible environments.


<div align="center">

  ![change-network](/public/img/change-network.PNG)

</div>

</details>

---

<details>
<summary>Save a webpage locally⛺</summary>

If you want to avoid having to wait for content to load directly from the blockchain every time you access the page, there is a simple solution: you can store the app locally in your browser. To do this, simply press Ctrl + d and you will be able to access the page immediately on your next searches.

It is important to note that this function stores the content of the page at the time of saving it. If the page's source code is updated (but not necessarily the content), you'll need to delete the app from the browser and save it again to access the latest version.

To remove the application from the browser, just go to the applications tab on the main page of the browser and select the application you want to remove. Once you've deleted the app, you can save it again to access the updated version.

Storing the app locally in the browser can be a great way to save time by accessing content faster. However, it is important to note that this function stores the current version of the page, so if the source code is updated, you need to delete the app from the browser and save it again to access the most recent version.

</details>

---

<h3 id="save-web-locallyget_page">get_page()📬 </h3>

The get_page() function is responsible for fetching the content of the PDAs from the Solana blockchain and rendering them in the app. The speed at which this process occurs varies depending on the version being used, as shown in the table below:

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

> The current version (0.2.4) of the architecture is the most efficient, having solved the problem of iterative volume with the PDAs during rendering. Additionally, it is worth noting that the PDAs now pass bytes instead of strings, further improving the efficiency of the process. Overall, the get_page() function plays a crucial role in enabling users to access and view the content stored on the decentralized web3 platform.
