var js_button = document.getElementById("js-button");

js_button.addEventListener("click", function () {
  setInterval(() => {
    fetch("/get_iter", {
      method: "POST",
    })
      .then((response) => response.json())
      .then((res) => {
        updateProgress(res.js_iter, res.max_js);
      });
  }, 10000);
  fetch("/send_app", {
    method: "POST",
    body: "JS",
  }).catch((error) => console.log(error));
});
