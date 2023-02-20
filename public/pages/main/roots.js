var convert = document.getElementById("convert");

function web() {
  window.location.replace("/main/web");
}
function config() {
  window.location.replace("/main/config");
}
function main() {
  window.location.replace("/main");
}

convert.addEventListener("click", function () {
  window.location.href = "/main/convert";
});

history.pushState({}, "", "/main");
