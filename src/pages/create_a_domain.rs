use rocket_dyn_templates::{context, Template};

#[get("/create_a_domain")]
pub fn index() -> Template {
    Template::render(
        "create_a_domain",
        context! {
            title: "Create a New Domain",
        }
    )
}