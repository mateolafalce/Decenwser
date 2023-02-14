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

  var data = {
    web_name: document.getElementById("init-domain").value,
    html_js: "HTML",
  };
  fetch("/send_app", {
    method: "POST",
    body: JSON.stringify(data),
  }).catch((error) => console.log(error));
});
