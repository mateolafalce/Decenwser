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

input.addEventListener("input", () => {
  fetch("/store_domain", {
    method: "POST",
    body: input.value,
  }).catch((error) => {
    console.error("There was a problem:", error);
  });
});
