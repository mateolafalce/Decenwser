use rocket_dyn_templates::{context, Template};

#[get("/upload_a_web")]
pub fn index() -> Template {
    Template::render(
        "upload_a_web",
        context! {
            title: "Upload a web",
        }
    )
}