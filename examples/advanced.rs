use std::{path::PathBuf, str::FromStr};
use serde::Serialize;
use strum::{Display, EnumIter, EnumString};
use template_forge::{Forge, ForgeConfig, ForgeTemplate,handlebars::DirectorySourceOptions};

use crate::templates::Templates;

mod components;
mod templates;

fn main() {
    let config = ForgeConfig {
        components_dir_path: PathBuf::from_str("examples/components/").unwrap(),
        components_dir_options: DirectorySourceOptions::default(),
        preview: false
    };

    let mut forge: Forge<_, Templates> = Forge::new(config).unwrap();

    forge
        .add_template(
            ForgeTemplate::new(
                Templates::EmailValidation(Default::default()), 
                PathBuf::from_str("examples/templates/email_validation.hbs").unwrap(),
            )
        )   
        .unwrap();

    let email = Templates::EmailValidation(Default::default());
    if let Templates::EmailValidation(data) = &email {
        dbg!(forge.render(&email, data));
    }
}