extern crate serde;

use self::serde::Serialize;
use rocket_contrib::Template;

#[derive(Serialize)]
struct Context<'a, T: Serialize> {
    template_name: &'a str,
    context: T,
}

impl<'a, T: Serialize> Context<'a, T> {
    pub fn new(template_name: &'a str, context: T) -> Self {
        Self {
            template_name,
            context,
        }
    }
}

pub fn render_with_layout<C: Serialize>(template_name: &str, context: C) -> Template {
    let context = Context::new(template_name, context);
    Template::render("layouts/index", context)
}
