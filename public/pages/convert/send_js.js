var js_button = document.getElementById("js-button");

js_button.addEventListener("click", function () {
  fetch("/send_app", {
    method: "POST",
    body: "JS",
  }).catch((error) => console.log(error));
});
