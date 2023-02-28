let inputElement = document.getElementsByName("search")[0];
const inputBar = document.getElementById("input-bar");

inputBar.addEventListener("keyup", function (event) {
  if (event.keyCode === 13) {
    getPage();
  }
});

async function getPage() {
  let input = document.getElementsByName("search")[0].value;
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
      setInterval(() => {
        fetch("/get_iter", {
          method: "POST",
        })
          .then((response) => response.json())
          .then((res) => {
            updateProgress(res.html_iter + res.js_iter, total_load_bar);
          });
      }, 1500);
      document.getElementById("phrase").style.display = "block";
      document.getElementById("autor").style.display = "block";
      document.getElementById("cite").style.display = "block";
      document.getElementById("background-bar").style.display = "block";
      document.getElementById("fill-bar").style.display = "block";
      document.getElementById("appCollector").innerHTML = "";
      document.getElementById("footer").innerHTML = "";
    });
}