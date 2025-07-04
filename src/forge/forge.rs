use handlebars::{DirectorySourceOptions, Handlebars};
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

use crate::{errors::ForgeError, forge::template::ForgeTemplate};

pub struct Forge<S: ToString> {
    hbs: Handlebars<'static>,
    templates: HashMap<String, ForgeTemplate<S>>,
}

pub struct ForgeConfig {
    /// Path used to register Handlebars templates
    pub components_dir_path: PathBuf,
    pub components_dir_options: DirectorySourceOptions,
    /// Enables `dev_mode` option of Handlebars
    pub preview: bool,
}

impl<S: ToString> Forge<S> {
    pub fn new(config: ForgeConfig) -> Result<Self, ForgeError> {
        let mut hbs = Handlebars::new();

        if config.preview {
            hbs.set_dev_mode(true);
        }

        hbs
            .register_templates_directory(
                config.components_dir_path,
                config.components_dir_options,
            )
            .map_err(|e| ForgeError::TemplateError(e))?;

        Ok(Self {
            hbs,
            templates: HashMap::new(),
        })
    }

    pub fn add_template(&mut self, template: ForgeTemplate<S>) -> Result<(), ForgeError> {
        self.hbs
            .register_template_file(template.get_name(), template.get_path())
            .map_err(|e| ForgeError::TemplateError(e))?;
        self.templates.insert(template.get_name().to_string(), template);
        Ok(())
    }

    pub fn get_templates(&self) -> Vec<String> {
        self.templates.iter().map(|(key, _)| key.to_string()).collect()
    }

    pub fn get_handlebars(&mut self) -> &mut Handlebars<'static> {
        &mut self.hbs
    }

    pub fn render<T>(&self, template_name: &S, data: &T) -> Result<String, ForgeError>
    where
        T: Serialize,
    {
        let template_name = template_name.to_string();
        let name = template_name.as_str();
        let output = self.hbs
            .render(name, data)
            .map_err(|e| ForgeError::RenderError(e))?;

        Ok(output)
    }
}