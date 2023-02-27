document.addEventListener("DOMContentLoaded", function () {
  fetch("/clear", {
    method: "POST",
  }).catch((error) => {
    console.error("Error:", error);
  });
});
