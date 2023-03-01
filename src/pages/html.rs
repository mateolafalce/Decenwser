use rocket_dyn_templates::{context, Template};

#[get("/html")]
pub fn index() -> Template {
    Template::render(
        "html",
        context! {
            title: "Upload Html Content",
        }
    )
}