use crate::templates::Templates;
use serde_json::Value;
use std::path::PathBuf;
use std::str::FromStr;
use template_forge::handlebars::DirectorySourceOptions;
use template_forge::{Forge, ForgeConfig, ForgeTemplate};

pub fn create_forge(preview: bool) -> Forge<Templates, Value> {
    let config = ForgeConfig {
        components_dir_path: PathBuf::from("examples/components/"),
        components_dir_options: DirectorySourceOptions::default(),
        preview,
    };

    let mut forge: Forge<Templates, Value> = Forge::new(config).unwrap();

    let template = Templates::EmailValidation(Default::default());
    if let Templates::EmailValidation(data) = &template {
        forge
            .add_template(ForgeTemplate::new(
                template.clone(),
                PathBuf::from_str("examples/templates/email_validation.hbs").unwrap(),
                serde_json::to_value(data.clone()).unwrap(),
            ))
            .unwrap();
    }

    let template = Templates::PasswordReset(Default::default());
    if let Templates::PasswordReset(data) = &template {
        forge
            .add_template(ForgeTemplate::new(
                template.clone(),
                PathBuf::from_str("examples/templates/password_reset.hbs").unwrap(),
                serde_json::to_value(data.clone()).unwrap(),
            ))
            .unwrap();
    }

    forge
}
