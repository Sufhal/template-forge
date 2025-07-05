use strum::{Display, EnumString};

use crate::templates::email_validation::EmailValidationData;
use crate::templates::password_reset::PasswordResetData;

pub mod email_validation;
pub mod password_reset;

#[derive(EnumString, Display, Clone)]
pub enum Templates {
    EmailValidation(EmailValidationData),
    PasswordReset(PasswordResetData),
}
