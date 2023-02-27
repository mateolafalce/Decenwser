var js_button = document.getElementById("js-button");

js_button.addEventListener("click", function () {
  const interval = setInterval(() => {
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
  })
    .then((res) => {
      fetch("/get_iter", {
        method: "POST",
      })
        .then((response) => response.json())
        .then((res) => {
          updateProgress(res.html_iter, res.max_html);
        });
    })
    .catch((error) => console.log(error))
    .finally(clearInterval(interval));
});
