document.addEventListener("keydown", function (event) {
  if (event.ctrlKey && event.key === "d") {
    event.preventDefault();
    fetch("http://127.0.0.1:2004/store_app", {
      method: "POST",
    });
    alert("Account successfully saved");
  }
});
