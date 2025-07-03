use std::{collections::HashMap, marker::PhantomData, path::PathBuf};
use handlebars::{DirectorySourceOptions, Handlebars};
use serde::Serialize;

use crate::{errors::ForgeError, forge::template::ForgeTemplate};

pub struct Forge<'a, T: Serialize, S: ToString> {
    hbs: Handlebars<'a>,
    templates: HashMap<String, ForgeTemplate<S>>,
    _phantom_t: PhantomData<T>
}

pub struct ForgeConfig {
    /// Path used to register Handlebars templates
    pub components_dir_path: PathBuf,
    pub components_dir_options: DirectorySourceOptions,
    /// Enables `dev_mode` option of Handlebars
    pub preview: bool,
}

impl<'a, T: Serialize, S: ToString> Forge<'a, T, S> {

    pub fn new(config: ForgeConfig) -> Result<Self, ForgeError> {
        let mut hbs = Handlebars::new();
        
        if config.preview {
            hbs.set_dev_mode(true);
        }

        hbs
            .register_templates_directory(
                config.components_dir_path, 
                config.components_dir_options
            )
            .map_err(|e| ForgeError::TemplateError(e))?;

        Ok(Self { 
            hbs,
            templates: HashMap::new(),
            _phantom_t: PhantomData
        })
    }

    pub fn add_template(&mut self, template: ForgeTemplate<S>) -> Result<(), ForgeError> {
        self.hbs
            .register_template_file(template.get_name(), template.get_path())
            .map_err(|e| ForgeError::TemplateError(e))?;
        self.templates.insert(template.get_name().to_string(), template);
        Ok(())
    }

    pub fn get_handlebars(&'a mut self) -> &'a mut Handlebars<'a> {
        &mut self.hbs
    }

    pub fn render(&self, template_name: &S, data: &T) -> Result<String, ForgeError> 
        where T: Serialize
    {
        let template_name = template_name.to_string();
        let name = template_name.as_str();
        let output = self.hbs
            .render(name, data)
            .map_err(|e| ForgeError::RenderError(e))?;

        Ok(output)
    }

}