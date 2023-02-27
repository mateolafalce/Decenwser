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
    encodeApi(files, "html");
  }
  if (files.name.endsWith(".js")) {
    encodeApi(files, "js");
  }
}
