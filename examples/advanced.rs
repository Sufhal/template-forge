use crate::common::utils::create_forge;
use crate::templates::Templates;
use serde::Serialize;
use std::str::FromStr;

mod components;
mod templates;
mod common;


fn main() {
    let forge = create_forge();

    let email = Templates::EmailValidation(Default::default());
    if let Templates::EmailValidation(data) = &email {
        dbg!(forge.render(&email, data));
    }
}