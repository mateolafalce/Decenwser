function web() {
  window.location.href = "http://127.0.0.1:2005";
}
function config() {
  window.location.replace("/main/config");
}
function main() {
  window.location.replace("/main");
}
function convert() {
  window.location.replace("/main/convert");
}

history.pushState({}, "", "/main");
