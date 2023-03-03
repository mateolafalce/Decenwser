let inputElement = document.getElementsByName("search")[0];
const inputBar = document.getElementById("input-bar");

inputBar.addEventListener("keyup", function (event) {
  if (event.keyCode === 13) {
    getPage();
  }
});

async function getPage() {
  fetch("/get_len", {
    method: "POST",
    body: inputElement.value,
  })
    .then(
      fetch("/store_pdas", {
        method: "POST",
      })
    )
    .then(async (res) => {
      if (res.status === 500) {
        main();
      }
      fetch("/get_page", {
        method: "POST",
      }).finally((res) => {
        web();
      });
      let result = await res.json();
      let total_load_bar = result.len_js + result.len_html;
      const intervalID = setInterval(() => {
        fetch("/get_iter", {
          method: "POST",
        })
          .then((response) => response.json())
          .then((res) => {
            if (res.html_iter + res.js_iter === total_load_bar) {
              clearInterval(intervalID);
            }
            updateProgress(res.html_iter + res.js_iter, total_load_bar);
          });
      }, 800);
      document.getElementById("load-container").style.display = "block";
      document.getElementById("appCollector").innerHTML = "";
      document.getElementById("footer").innerHTML = "";
    });
}
