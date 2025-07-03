use strum::{Display, EnumString};

use crate::templates::email_validation::EmailValidationData;

pub mod email_validation;

#[derive(EnumString, Display)]
pub enum Templates {
    EmailValidation(EmailValidationData) 
}