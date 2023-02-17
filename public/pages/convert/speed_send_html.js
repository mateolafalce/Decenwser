var html_button = document.getElementById("speed-html-button");

html_button.addEventListener("click", function () {
  fetch("/create_signers", {
    method: "POST",
  }).then((res) => {
    fetch("/send_sol_to_signers", {
      method: "POST",
      body: "HTML",
    }).then((res) => {
      fetch("/get_iter", {
        method: "POST",
      })
        .then((response) => response.json())
        .then((res) => {
          let iter = 0;
          while (iter < res.max_html) {
            var data = {
              html_js: "HTML",
              iter: iter,
            };
            fetch("/speed_send_app", {
              method: "POST",
              body: JSON.stringify(data),
            });
            iter++;
          }
        });
    });
  });
});
