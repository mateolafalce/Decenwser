inputElement.addEventListener("input", () => {
  fetch("/store_domain", {
    method: "POST",
    body: inputElement.value,
  }).catch((error) => {
    console.error("There was a problem:", error);
  });
});
