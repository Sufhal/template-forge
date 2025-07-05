use serde::Serialize;

use crate::components::{
    layout_begin::LayoutBeginData, main_button::MainButtonData, paragraph::ParagraphData,
    title::TitleData,
};

#[derive(Serialize, Default, Clone)]
pub struct PasswordResetData {
    layout_begin: LayoutBeginData,
    title: TitleData,
    paragraph_1: ParagraphData,
    main_button: MainButtonData,
}
