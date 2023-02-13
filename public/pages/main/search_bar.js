const bar = document.getElementById("input-bar");

document.addEventListener("DOMContentLoaded", state());

function state() {
  fetch("/get_config", {
    method: "POST",
  })
    .then(async (result) => {
      let res = await result.json();
      if (res.network == true) {
        bar.setAttribute("placeholder", "Currently on Mainnet");
      } else if (res.network == false) {
        bar.setAttribute("placeholder", "Currently on Devnet");
      }
    })
    .catch((error) => {
      console.error("Error:", error);
    });
}
