var html_output = document.getElementById("html-output");
var css_output = document.getElementById("css-output");
var js_output = document.getElementById("js-output");

function convertInput() {
  let input = document.getElementsByName("input")[0].value;
  fetch("/encode", {
    method: "POST",
    body: input,
  })
    .then((res) => res.json())
    .then((result) => {
      console.log(result.output);
    })
    .catch((error) => console.error("Error:", error));
}
function deleteInput() {
  document.getElementsByName("input")[0].value = "";
}
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
          let parts = [];
          for (let i = 0; i < result.output.length; i += 932) {
            parts.push(result.output.substring(i, i + 932));
          }
          console.log(parts);
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
          let parts = [];
          for (let i = 0; i < result.output.length; i += 932) {
            parts.push(result.output.substring(i, i + 932));
          }
          console.log(parts);
        })
        .catch((error) => console.error("Error:", error));
    };
    reader.readAsText(files);
  }
  if (files.name.endsWith(".js")) {
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
          //var button = document.createElement("button-js");
          //button.innerHTML = "<button id='button-js'>Send</button>";
          //newElement.innerHTML = parts[0];
          //js_output.appendChild(button);
          //send_js(await parts[0]);
          console.log(parts);
        })
        .catch((error) => console.error("Error:", error));
    };
    reader.readAsText(files);
  }
}
function send_js(js) {
  var data = {
    web_name: "mateo",
    js: js,
  };
  fetch("/send_js", {
    method: "POST",
    body: JSON.stringify(data),
  })
    .then((res) => res.json())
    .then((result) => {
      console.log(result);
    })
    .catch((error) => console.error("Error:", error));
}
