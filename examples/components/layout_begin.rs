use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct LayoutBeginData {
    title: String,
    subject: String,
    lang: String,
}

impl Default for LayoutBeginData {
    fn default() -> Self {
        Self {
            title: "Default title".to_string(),
            subject: "Default subject".to_string(),
            lang: "en".to_string(),
        }
    }
}
