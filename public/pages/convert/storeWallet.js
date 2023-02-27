var fileInput = document.getElementById("fileInput");
fileInput.addEventListener("change", function (event) {
  var file = event.target.files[0];
  var reader = new FileReader();
  reader.onload = async function () {
    let result = reader.result;
    fetch("/store_wallet", {
      method: "POST",
      body: result,
    });
  };
  reader.readAsText(file);
});
