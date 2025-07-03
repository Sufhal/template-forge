use serde::Serialize;

#[derive(Serialize)]
pub struct TitleData {
    text: String
}

impl Default for TitleData {
    fn default() -> Self {
        Self {
            text: "Default title".to_string(), 
        }
    }
}