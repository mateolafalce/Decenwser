# 🌐Decenwser🌐

A decentralized browser running locally as a server at http://127.0.0.1:2004. It uses pda storage, offered by Solana Blockchain, for HTML and JS content.

---

![Decenwser](/public/img/decenwser.PNG)

---

### Run the project

```
git clone https://github.com/mateolafalce/Decenwser-rust.git && cd Decenwser-rust && cargo run
```

---

### Blockchain Program

Available [`here`](https://github.com/mateolafalce/Decenwser-blockchain-program)

---

### GET HTTP Requests

```
http://127.0.0.1:2004/main          --Render the browser

http://127.0.0.1:2004/main/convert  --Render all tools and info to upload a web

http://127.0.0.1:2004/main/web      --Render the searched web
```

### POST HTTP Requests

```
http://127.0.0.1:2004/clear                 --Removes all variables initialized with the use of the browser, such as templates, scripts and coded files

http://127.0.0.1:2004/encode                --Convert the html and js to text sendable to the solana blockchain as String on a pda

http://127.0.0.1:2004/get_page              --Render a web

http://127.0.0.1:2004/create_app            --Send a request to the solana blockchain to create a domain

http://127.0.0.1:2004/send_app              --Send html and js to the solana blockchain to save the app decentrally

http://127.0.0.1:2004/store_domain          --Save the domain entered in the navigation bar to eliminate arguments when doing other functions that require the domain

http://127.0.0.1:2004/get_len               --Take the maximum size of the page

http://127.0.0.1:2004/store_app             --Save a rendered web locally

http://127.0.0.1:2004/delete_stored_app     --Delete the locally stored page

http://127.0.0.1:2004/delete_pda            --Remove html and js content from a domain

```

## License

[Apache 2.0](https://choosealicense.com/licenses/apache-2.0/)
