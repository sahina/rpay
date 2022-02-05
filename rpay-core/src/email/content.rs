use tera::{Context, Tera};

use crate::vars::templates_glob;

pub fn build_tera() -> Tera {
    let templates = templates_glob();
    Tera::new(&templates).unwrap()
}

pub fn confirm_email_address(context: &Context) -> String {
    build_tera()
        .render("confirmation_email.html", &context)
        .unwrap()
}
