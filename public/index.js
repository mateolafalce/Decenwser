function get_page() {
  input = document.getElementsByName("search")[0].value;
  fetch("/get_app", {
    method: "POST",
    body: input,
  })
    .then((res) => res.json())
    .then((result) => {
      let htmlContent = [];
      let iter = 1;
      let len = result.html.length;

      request(iter, len);
      async function request(iter, len) {
        var xhr = new XMLHttpRequest();
        xhr.open("POST", "https://api.devnet.solana.com");
        xhr.setRequestHeader("Content-Type", "application/json");
        xhr.onreadystatechange = async function () {
          if (xhr.readyState === 4) {
            var html = await JSON.parse(
              xhr.response
            ).result.meta.logMessages[2].slice(13);
            console.log(html);
            htmlContent += html;
            if (len > iter) {
              iter++;
              await request(iter, len);
            } else {
              let x = htmlContent
                .replace(/#~/g, '"')
                .replace(/°¬/g, "\\")
                .replace(/#!/g, ",");
              console.log(x);
              document.write(
                htmlContent
                  .replace(/#~/g, '"')
                  .replace(/°¬/g, "\\")
                  .replace(/#!/g, ",")
              );
            }
          }
        };
        var data1 = `
        {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTransaction",
            "params": [
            "${result.html[iter - 1]}",
            "json"
            ]
        }
        `;
        xhr.send(data1);
      }
    })
    .catch((error) => console.error("Error:", error));
}
