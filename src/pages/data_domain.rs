use rocket_dyn_templates::{context, Template};

#[get("/data_domain")]
pub fn index() -> Template {
    Template::render(
        "data_domain",
        context! {
            title: "Data domain",
        }
    )
}