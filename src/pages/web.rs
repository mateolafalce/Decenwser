use rocket_dyn_templates::{context, Template};

#[get("/web")]
pub fn index() -> Template {
    Template::render(
        "web",
        context! {
            foo: 123,
        }
    )
}