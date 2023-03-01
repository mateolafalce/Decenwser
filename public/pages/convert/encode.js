document.addEventListener("DOMContentLoaded", function () {
  fetch("/clear", {
    method: "POST",
  }).catch((error) => {
    console.error("Error:", error);
  });
});

function encodeApi(files, html_js) {
  const reader = new FileReader();
  reader.onload = function () {
    var data = {
      input: reader.result,
      html_js: html_js,
    };
    fetch("/encode", {
      method: "POST",
      body: JSON.stringify(data),
    })
      .then((response) => response.json())
      .then((res) => {
        if (res.html_js == "html") {
          document.getElementById("html-price").innerText =
            res.price / 1000000000 + " SOL";
          document.getElementById("total-price").innerText =
            res.price / 1000000000 +
            0.00145964 +
            parseFloat(document.getElementById("js-price").innerHTML) +
            " SOL";
        } else if (res.html_js == "js") {
          document.getElementById("js-price").innerText =
            res.price / 1000000000 + " SOL";
          document.getElementById("total-price").innerText =
            res.price / 1000000000 +
            0.00145964 +
            parseFloat(document.getElementById("html-price").innerHTML);
          +" SOL";
        }
      })
      .catch((error) => console.error("Error:", error));
  };
  reader.readAsText(files);
}
