use handlebars::{RenderError, TemplateError};

#[derive(Debug)]
pub enum ForgeError {
    TemplateUndefined,
    TemplateError(TemplateError),
    RenderError(RenderError)
}