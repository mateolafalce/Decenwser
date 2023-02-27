use rocket_dyn_templates::{context, Template};

#[get("/convert")]
pub fn index() -> Template {
    Template::render(
        "convert",
        context! {
            foo: 123,
        }
    )
}