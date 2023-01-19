var html_output = document.getElementById("html-output");
var css_output = document.getElementById("css-output");
var js_output = document.getElementById("js-output");

let web_name = "re";

const dropArea = document.getElementById("drop-area");
["dragenter", "dragover", "dragleave", "drop"].forEach((eventName) => {
  dropArea.addEventListener(eventName, preventDefaults, false);
  document.body.addEventListener(eventName, preventDefaults, false);
});
["dragenter", "dragover"].forEach((eventName) => {
  dropArea.addEventListener(eventName, highlight, false);
});
["dragleave", "drop"].forEach((eventName) => {
  dropArea.addEventListener(eventName, unhighlight, false);
});
dropArea.addEventListener("drop", handleDrop, false);
function preventDefaults(e) {
  e.preventDefault();
  e.stopPropagation();
}
function highlight(e) {
  dropArea.classList.add("highlight");
}
function unhighlight(e) {
  dropArea.classList.remove("highlight");
}
function handleDrop(e) {
  var dt = e.dataTransfer;
  var files = dt.files;

  handleFiles(files);
}
function handleFiles(files) {
  [...files].forEach(uploadFile);
}

/*var counter = document.getElementById("counter");
counter.addEventListener("click", function () {
  let count = counter.innerHTML.;
  counter.innerHTML = count += 1;
});*/

function uploadFile(files) {
  if (files.name.endsWith(".html")) {
    const reader = new FileReader();
    reader.onload = function () {
      fetch("/encode", {
        method: "POST",
        body: reader.result,
      })
        .then((res) => res.json())
        .then(async (result) => {
          let parts = [""];
          for (let i = 0; i < result.output.length; i += 932) {
            parts.push(result.output.substring(i, i + 932));
          }
          for (var i = 1; i < parts.length; i++) {
            var newDiv = document.createElement("style");
            var button = document.createElement("button");
            var br = document.createElement("br");
            newDiv.innerHTML = parts[i];
            newDiv.className = "html-" + (i + 1);
            button.id = "html-" + (i + 1);
            newDiv.hidden = true;
            button.innerHTML = "Send HTML";
            html_output.appendChild(newDiv);
            html_output.appendChild(br);
            button.addEventListener("click", function () {
              send_html(
                document.getElementsByClassName(button.id)[0].innerHTML
              );
            });
            html_output.appendChild(button);
          }
        })
        .catch((error) => console.error("Error:", error));
    };
    reader.readAsText(files);
  }
  if (files.name.endsWith(".css")) {
    const reader = new FileReader();
    reader.onload = function () {
      fetch("/encode", {
        method: "POST",
        body: reader.result,
      })
        .then((res) => res.json())
        .then(async (result) => {
          let parts = [""];
          for (let i = 0; i < result.output.length; i += 932) {
            parts.push(result.output.substring(i, i + 932));
          }
          for (var i = 1; i < parts.length; i++) {
            var newDiv = document.createElement("div");
            var button = document.createElement("button");
            var br = document.createElement("br");
            newDiv.innerHTML = parts[i];
            newDiv.className = "css-" + (i + 1);
            button.id = "css-" + (i + 1);
            newDiv.hidden = true;
            button.innerHTML = "Send Css";
            css_output.appendChild(newDiv);
            css_output.appendChild(br);
            button.addEventListener("click", function () {
              send_css(document.getElementsByClassName(button.id)[0].innerText);
            });
            css_output.appendChild(button);
          }
        })
        .catch((error) => console.error("Error:", error));
    };
    reader.readAsText(files);
  }
  if (files.name.endsWith(".js")) {
    const reader = new FileReader();
    reader.onload = function () {
      let iter = 0;
      let chunks = [];
      let chunkSize = 8000;
      for (let i = 0; i < reader.result.length; i += chunkSize) {
        chunks.push(reader.result.slice(i, i + chunkSize));
      }
      for (let i = 0; i < chunks.length; i += 1) {
        fetch("/encode", {
          method: "POST",
          body: chunks[i],
        })
          .then((res) => res.json())
          .then(async (result) => {
            let parts = [""];
            for (let i = 0; i < result.output.length; i += 932) {
              parts.push(result.output.substring(i, i + 932));
            }
            for (var foo = 1; foo < parts.length; foo++) {
              var newDiv = document.createElement("style");
              var button = document.createElement("button");
              var br = document.createElement("br");
              newDiv.innerHTML = parts[foo];
              newDiv.className = iter;
              button.id = iter;
              newDiv.hidden = true;
              button.innerHTML = "Send Js";
              js_output.appendChild(newDiv);
              js_output.appendChild(br);
              button.addEventListener("click", function () {
                send_js(
                  document.getElementsByClassName(button.id)[0].innerText,
                  button.id
                );
              });
              js_output.appendChild(button);
              iter++;
            }
          })
          .catch((error) => console.error("Error:", error));
      }
    };
    reader.readAsText(files);
  }
}
async function send_js(js, id) {
  var data = {
    web_name: web_name,
    content: js,
  };
  fetch("/send_js", {
    method: "POST",
    body: JSON.stringify(data),
  })
    .then((res) => res.json())
    .then((result) => {
      console.log(result);
      console.log(id);
      document.getElementById(id).style.color = "white";
      document.getElementById(id).style.backgroundColor = "green";
    })
    .catch((error) => {
      console.error("Error:", error);
      document.getElementById(id).style.color = "white";
      document.getElementById(id).style.backgroundColor = "red";
    });
}
async function send_css(css) {
  var data = {
    web_name: web_name,
    content: css,
  };
  fetch("/send_css", {
    method: "POST",
    body: JSON.stringify(data),
  })
    .then((res) => res.json())
    .then((result) => {
      console.log(result);
    })
    .catch((error) => console.error("Error:", error));
}
async function send_html(html) {
  var data = {
    web_name: web_name,
    content: html,
  };
  fetch("/send_html", {
    method: "POST",
    body: JSON.stringify(data),
  })
    .then((res) => res.json())
    .then((result) => {
      console.log(result);
    })
    .catch((error) => console.error("Error:", error));
}
