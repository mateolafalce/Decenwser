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
}
