use crate::common::utils::create_forge;
use crate::templates::Templates;

mod common;
mod components;
mod templates;

fn main() {
    let forge = create_forge(false);

    let email = Templates::EmailValidation(Default::default());
    if let Templates::EmailValidation(data) = &email {
        dbg!(forge.render(&email, &serde_json::to_value(data).unwrap()));
    }
}
