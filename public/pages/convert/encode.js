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
    }).catch((error) => console.error("Error:", error));
  };
  reader.readAsText(files);
}
