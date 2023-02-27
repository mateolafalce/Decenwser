const http = require("http");
const fs = require("fs");
const path = require("path");

const hostname = "127.0.0.1";
const port = 2005;

const server = http.createServer((req, res) => {
  if (req.url === "/") {
    const indexFilePath = path.join(__dirname, "./templates/web.html.hbs");
    fs.readFile(indexFilePath, (err, data) => {
      if (err) {
        res.writeHead(500);
        res.end(`Error loading index.html: ${err}`);
        return;
      }
      res.writeHead(200, { "Content-Type": "text/html" });
      res.end(data);
    });
  } else if (req.url === "/js.js") {
    const jsFilePath = path.join(__dirname, "./public/js.js");
    fs.readFile(jsFilePath, (err, data) => {
      if (err) {
        res.writeHead(500);
        res.end(`Error loading js.js: ${err}`);
        return;
      }
      res.writeHead(200, { "Content-Type": "application/javascript" });
      res.end(data);
    });
  } else {
    res.writeHead(404);
    res.end("Not found");
  }
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
