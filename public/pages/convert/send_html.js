var html_button = document.getElementById("html-button");

html_button.addEventListener("click", function () {
  var data = {
    web_name: document.getElementById("init-domain").value,
    html_js: "HTML",
  };
  fetch("/send_app", {
    method: "POST",
    body: JSON.stringify(data),
  }).catch((error) => console.log(error));
});
