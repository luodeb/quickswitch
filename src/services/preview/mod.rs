mod directory_generator;
mod image_generator;
mod pdf_generator;
mod preview_content;
mod preview_generator;
mod text_generator;

pub use directory_generator::DirectoryPreviewGenerator;
pub use image_generator::ImagePreviewGenerator;
use once_cell::sync::Lazy;
pub use pdf_generator::PdfPreviewGenerator;
pub use preview_content::PreviewContent;
pub use preview_generator::{
    BinaryPreviewGenerator, PreviewGenerator, PreviewGeneratorTrait, process_special_characters,
};
use ratatui_image::picker::Picker;
pub use text_generator::TextPreviewGenerator;

pub static GLOBAL_PICKER: Lazy<Picker> =
    Lazy::new(|| Picker::from_query_stdio().unwrap_or_else(|_| Picker::from_fontsize((8, 16))));
