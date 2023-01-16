function convertInput() {
  let input = document.getElementsByName("input")[0].value;
  fetch("/encode", {
    method: "POST",
    body: input,
  })
    .then((res) => res.json())
    .then((result) => {
      console.log(result.output);
    })
    .catch((error) => console.error("Error:", error));
}
function deleteInput() {
  document.getElementsByName("input")[0].value = "";
}
