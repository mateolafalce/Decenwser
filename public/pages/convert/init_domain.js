var input = document.getElementById("init-domain");

function init_domain() {
  fetch("/create_app", {
    method: "POST",
    body: input.value,
  })
    .then((res) => res.json())
    .then((result) => {
      console.log(result);
    })
    .catch((error) => {
      console.error("Error:", error);
    });
}
