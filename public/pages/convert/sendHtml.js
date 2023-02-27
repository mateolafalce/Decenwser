var html_button = document.getElementById("html-button");

html_button.addEventListener("click", function () {
  const intervalId = setInterval(() => {
    fetch("/get_iter", {
      method: "POST",
    })
      .then((response) => response.json())
      .then((res) => {
        updateProgress(res.html_iter, res.max_html);
      });
  }, 10000);
  fetch("/send_app", {
    method: "POST",
    body: "HTML",
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
    .finally(clearInterval(intervalId));
});
