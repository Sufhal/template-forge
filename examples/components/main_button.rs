use serde::Serialize;

#[derive(Serialize)]
pub struct MainButtonData {
    href: String,
    text: String
}

impl Default for MainButtonData {
    fn default() -> Self {
        Self {
            href: "#".to_string(),
            text: "Default button".to_string(), 
        }
    }
}