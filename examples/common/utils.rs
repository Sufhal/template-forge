use crate::templates::Templates;
use std::path::PathBuf;
use std::str::FromStr;
use template_forge::handlebars::DirectorySourceOptions;
use template_forge::{Forge, ForgeConfig, ForgeTemplate};

pub fn create_forge() -> Forge<Templates>
{
    let config = ForgeConfig {
        components_dir_path: PathBuf::from("examples/components/"),
        components_dir_options: DirectorySourceOptions::default(),
        preview: false,
    };

    let mut forge: Forge<Templates> = Forge::new(config).unwrap();

    forge
        .add_template(
            ForgeTemplate::new(
                Templates::EmailValidation(Default::default()),
                PathBuf::from_str("examples/templates/email_validation.hbs").unwrap(),
            )
        )
        .unwrap();

    forge
}