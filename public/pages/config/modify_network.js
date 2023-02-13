function modify_network() {
  fetch("/modify_network", {
    method: "POST",
  }).catch((error) => {
    console.error("Error:", error);
  });
  state();
}
