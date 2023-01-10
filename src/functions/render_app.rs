use std::fs::File;
use std::io::Write;

pub fn render_app() {
    let mut file = File::create("../../templates/web/index.html.hbs").unwrap();
    let tx = "Te voy a metel a pija por la nariz ricotera".to_string();
    file.write_all(tx.as_bytes()).unwrap();
}
