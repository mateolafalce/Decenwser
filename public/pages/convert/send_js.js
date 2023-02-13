var js_button = document.getElementById("js-button");

js_button.addEventListener("click", function () {
  var data = {
    web_name: document.getElementById("init-domain").value,
    html_js: "JS",
  };
  fetch("/send_app", {
    method: "POST",
    body: JSON.stringify(data),
  }).catch((error) => console.log(error));
});
