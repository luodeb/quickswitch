use anyhow::Result;

use crate::{
    app_state::AppState,
    modes::ModeManager,
    services::{PreviewManager, create_data_provider},
    utils::AppMode,
};

pub struct App {
    pub state: AppState,
    pub mode_manager: ModeManager,
}

impl App {
    pub fn new(initial_mode: AppMode) -> Result<Self> {
        let mut state = AppState::new()?;

        // Load initial data using data provider
        let data_provider = create_data_provider(&initial_mode);
        data_provider.load_data(&mut state)?;

        let app = App {
            state,
            mode_manager: ModeManager::new(&initial_mode),
        };

        // Clear preview
        PreviewManager::clear_preview();

        Ok(app)
    }
}
