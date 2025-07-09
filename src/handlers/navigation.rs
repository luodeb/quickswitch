use anyhow::Result;

use crate::app::App;

/// Common navigation operations for list-based interfaces
pub struct NavigationHelper;

impl NavigationHelper {


    /// Navigate to parent directory
    pub fn navigate_to_parent(app: &mut App) -> Result<bool> {
        if let Some(parent) = app.state.current_dir.parent() {
            app.change_directory(parent.to_path_buf())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Navigate into selected directory
    pub fn navigate_into_directory(app: &mut App) -> Result<bool> {
        if let Some(file) = app.get_selected_file() {
            if file.is_dir {
                app.change_directory(file.path.clone())?;
                return Ok(true);
            }
        }
        Ok(false)
    }




}
