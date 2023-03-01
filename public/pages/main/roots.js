function web() {
  window.location.href = "http://127.0.0.1:2005";
}
function config() {
  window.location.replace("/main/config");
}
function main() {
  window.location.replace("/main");
}
function uploadAWeb() {
  window.location.replace("/main/upload_a_web");
}

history.pushState({}, "", "/main");
