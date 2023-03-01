function main() {
  window.location.replace("/main");
}

function createADomain() {
  window.location.replace("/main/upload_a_web/create_a_domain");
}

history.pushState({}, "", "/main/upload_a_web");
