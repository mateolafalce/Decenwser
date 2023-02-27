const pubkey = document.getElementById("pubkey");
const domain = document.getElementById("domain");
const authority = document.getElementById("authority");
const bump_original = document.getElementById("bump_original");
const html = document.getElementById("html");
const js = document.getElementById("js");
let input = document.getElementById("input-domain");

function get_data_domain() {
  fetch("/get_data_domain", {
    method: "POST",
    body: input.value,
  })
    .then((res) => res.json())
    .then((result) => {
      pubkey.innerText = result.pubkey;
      domain.innerText = result.web_name;
      authority.innerText = result.authority;
      bump_original.innerText = result.bump_original;
      html.innerText = result.html;
      js.innerText = result.js;
    });
}
