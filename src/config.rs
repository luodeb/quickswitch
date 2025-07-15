use anyhow::Result;
use std::{fs, path::PathBuf};

/// Get the data directory for quickswitch
///
/// This function reads the `_QUICKSWITCH_DATA_DIR` environment variable.
/// If the environment variable is not set or empty, it returns a suitable default directory:
/// - On Unix-like systems: `~/.local/share/quickswitch`
/// - On Windows: `%APPDATA%\quickswitch`
///
/// The function will create the directory if it doesn't exist.
pub fn get_data_dir() -> Result<PathBuf> {
    // First, try to read from environment variable
    if let Ok(env_dir) = std::env::var("_QUICKSWITCH_DATA_DIR") {
        if !env_dir.trim().is_empty() {
            let data_dir = PathBuf::from(env_dir);
            // Create directory if it doesn't exist
            if !data_dir.exists() {
                fs::create_dir_all(&data_dir)?;
            }
            return Ok(data_dir);
        }
    }

    // If environment variable is not set or empty, use default directory
    let data_dir = get_default_data_dir()?;

    // Create directory if it doesn't exist
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }

    Ok(data_dir)
}

/// Get the default data directory based on the operating system
fn get_default_data_dir() -> Result<PathBuf> {
    #[cfg(windows)]
    {
        // On Windows, use %APPDATA%\quickswitch
        if let Ok(appdata) = std::env::var("APPDATA") {
            Ok(PathBuf::from(appdata).join("quickswitch"))
        } else {
            // Fallback to temp directory if APPDATA is not available
            Ok(std::env::temp_dir().join("quickswitch"))
        }
    }

    #[cfg(not(windows))]
    {
        // On Unix-like systems, use ~/.local/share/quickswitch
        if let Ok(home) = std::env::var("HOME") {
            Ok(PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("quickswitch"))
        } else if let Ok(xdg_data_home) = std::env::var("XDG_DATA_HOME") {
            // Follow XDG Base Directory Specification
            Ok(PathBuf::from(xdg_data_home).join("quickswitch"))
        } else {
            // Fallback to temp directory if HOME is not available
            Ok(std::env::temp_dir().join("quickswitch"))
        }
    }
}
