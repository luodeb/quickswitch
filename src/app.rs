use anyhow::Result;

use crate::
    models::{AppState, DisplayItem}
;

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new() -> Result<Self> {
        let state = AppState::new()?;
        let app = Self { state };
        Ok(app)
    }

    pub fn update_filter(&mut self) {
        self.state.apply_search_filter();
    }

    pub fn get_selected_item(&self) -> Option<&DisplayItem> {
        if let Some(selected) = self.state.file_list_state.selected() {
            if let Some(&file_index) = self.state.filtered_files.get(selected) {
                return self.state.files.get(file_index);
            }
        }
        None
    }


}
