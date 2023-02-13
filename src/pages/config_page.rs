use rocket_dyn_templates::{context, Template};

#[get("/config")]
pub fn index() -> Template {
    Template::render(
        "config",
        context! {
            foo: 123,
        }
    )
}