use serde::Serialize;

use crate::components::{layout_begin::LayoutBeginData, main_button::MainButtonData, paragraph::ParagraphData, title::TitleData};

#[derive(Serialize, Default)]
pub struct EmailValidationData {
    layout_begin: LayoutBeginData,
    title: TitleData,
    paragraph_1: ParagraphData,
    paragraph_2: ParagraphData,
    paragraph_3: ParagraphData,
    main_button: MainButtonData,
}