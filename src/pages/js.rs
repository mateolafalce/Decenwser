use rocket_dyn_templates::{context, Template};

#[get("/js")]
pub fn index() -> Template {
    Template::render(
        "js",
        context! {
            title: "Upload Javascript Content",
        }
    )
}