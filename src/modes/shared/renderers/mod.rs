pub mod preview;

use crate::app::App;
pub use preview::PreviewRenderer;
use ratatui::{Frame, layout::Rect};

pub trait Renderer {
    /// Render the component in the given area
    fn render(&self, f: &mut Frame, area: Rect, app: &App);
}
