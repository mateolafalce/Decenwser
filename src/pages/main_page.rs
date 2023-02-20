use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "main",
        context! {
            title: "Decenwser",
        }
    )
}