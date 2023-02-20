let inputElement = document.getElementsByName("search")[0];
async function get_page() {
  let input = document.getElementsByName("search")[0].value;
  fetch("/get_len", {
    method: "POST",
    body: inputElement.value,
  })
    .then(
      fetch("/store_pdas", {
        method: "POST",
      })
    )
    .then(async (res) => {
      if (res.status === 500) {
        main();
      }
      let result = await res.json();
      let iter_html = result.len_html;
      let iter_js = result.len_js;
      let i_html = 0;
      let i_js = 0;
      let promises = [];
      while (i_html < iter_html) {
        var data = {
          domain: input,
          html_js: "HTML",
          iter: i_html,
        };
        promises.push(
          fetch("/get_page", {
            method: "POST",
            body: JSON.stringify(data),
          })
        );
        i_html++;
      }
      while (i_js < iter_js) {
        var data = {
          domain: input,
          html_js: "JS",
          iter: i_js,
        };
        promises.push(
          fetch("/get_page", {
            method: "POST",
            body: JSON.stringify(data),
          })
        );
        i_js++;
      }
      document.write(
        '<!DOCTYPE html><html lang="en"><link rel="icon" href="./Decenwser.ico"><title>Loading</title><div><h1 id="loading">Loading...</h1></div><style>#loading {position: absolute; top: 50%;  left: 50%;  transform: translate(-50%, -50%);font-family: "Times New Roman", Times, serif;font-style: italic;}</style></html>'
      );
      await Promise.all(promises);
      web();
    });
}
