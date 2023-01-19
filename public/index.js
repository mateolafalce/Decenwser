function convert() {
  window.location.replace("/convert");
}

async function get_page() {
  fetch("/get_page", {
    method: "POST",
    body: document.getElementsByName("search")[0].value,
  })
    .then((res) => res.json())
    .then((result) => {
      document.close();
      document.write(result.html);
      document.getElementById("css").innerHTML = result.css;
      document.getElementById("js").innerHTML = result.js;
    })
    .catch((error) => {
      //window.location.replace("http://127.0.0.1:8000");
      console.error("Error:", error);
    });
  document.write(
    '<!DOCTYPE html><html lang="en"><link rel="icon" href="./Decenwser.ico"><div><h1 id="loading">Loading...</h1></div><style>#loading {position: absolute; top: 50%;  left: 50%;  transform: translate(-50%, -50%);font-family: "Times New Roman", Times, serif;font-style: italic;}</style></html>'
  );
}
