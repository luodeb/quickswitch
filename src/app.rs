use anyhow::Result;
use crossterm::event::{KeyCode, MouseEvent};
use ratatui::{Frame, layout::Rect, style::Style};

use crate::{
    models::{AppMode, AppState, DisplayItem},
    modes::{ModeAction, ModeManager},
    services::{ActionDispatcher, PreviewManager, create_data_provider},
};

pub struct App {
    pub state: AppState,
    mode_manager: ModeManager,
}

impl App {
    pub fn new(initial_mode: AppMode) -> Result<Self> {
        let state = AppState::new()?;

        // Load initial data using data provider
        let data_provider = create_data_provider(&initial_mode);
        let mut temp_app = App {
            state,
            mode_manager: ModeManager::new(&initial_mode),
        };
        data_provider.load_data(&mut temp_app)?;

        // Clear preview
        PreviewManager::clear_preview(&mut temp_app);

        Ok(temp_app)
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

    pub fn switch_mode(&mut self, new_mode: AppMode) -> Result<()> {
        // Use unsafe to split the borrow temporarily
        let mode_manager_ptr = &mut self.mode_manager as *mut ModeManager;
        let app_ptr = self as *mut App;

        unsafe { (*mode_manager_ptr).switch_mode(&mut *app_ptr, &new_mode) }
    }

    pub fn handle_key(&mut self, key: KeyCode) -> Result<ModeAction> {
        let current_mode = self.mode_manager.get_current_mode().clone();
        ActionDispatcher::handle_key_event(self, key, &current_mode)
    }

    pub fn handle_mouse(
        &mut self,
        mouse: MouseEvent,
        left_area: Rect,
        right_area: Rect,
    ) -> Result<ModeAction> {
        let current_mode = self.mode_manager.get_current_mode().clone();
        ActionDispatcher::handle_mouse_event(self, mouse, left_area, right_area, &current_mode)
    }

    pub fn render_left_panel(&self, f: &mut Frame, area: Rect) {
        self.mode_manager.render_left_panel(f, area, self);
    }

    pub fn render_right_panel(&self, f: &mut Frame, area: Rect) {
        self.mode_manager.render_right_panel(f, area, self);
    }

    pub fn get_search_box_config(&self) -> (String, String, Style) {
        self.mode_manager.get_search_box_config(self)
    }

    pub fn get_current_mode(&self) -> &AppMode {
        self.mode_manager.get_current_mode()
    }

    pub fn is_mode(&self, mode: &AppMode) -> bool {
        self.mode_manager.is_mode(mode)
    }
}
