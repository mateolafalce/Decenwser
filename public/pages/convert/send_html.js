var html_button = document.getElementById("html-button");

html_button.addEventListener("click", function () {
  setInterval(() => {
    fetch("/get_iter", {
      method: "POST",
    })
      .then((response) => response.json())
      .then((res) => {
        updateProgress(res.html_iter, res.max_html);
      });
  }, 25000);
  fetch("/send_app", {
    method: "POST",
    body: "HTML",
  }).catch((error) => console.log(error));
});
