let inputElement = document.getElementById("network");

document.addEventListener("DOMContentLoaded", state());

function state() {
  fetch("/get_config", {
    method: "POST",
  })
    .then(async (result) => {
      let res = await result.json();
      if (res.network == false) {
        inputElement.innerText = "Change to Mainnet";
      } else if (res.network == true) {
        inputElement.innerText = "Change to Devnet";
      }
    })
    .catch((error) => {
      console.error("Error:", error);
    });
}
