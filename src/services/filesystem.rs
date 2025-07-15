use anyhow::Result;
use std::{fs, path::PathBuf};

use crate::utils::FileItem;

/// Service for filesystem operations
pub struct FilesystemService;

impl FilesystemService {
    /// Load directory contents and return sorted file list
    pub fn load_directory(current_dir: &PathBuf) -> Result<Vec<FileItem>> {
        let mut files = Vec::new();

        // Check if we're at Windows drives view and should show drives
        if Self::should_show_drives(current_dir) {
            return Self::load_drives();
        }

        // 添加当前目录
        files.push(FileItem {
            name: ".".to_string(),
            path: current_dir.clone(),
            is_dir: true,
        });

        let entries = fs::read_dir(current_dir)?;
        let mut items: Vec<FileItem> = entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = path.is_dir();

                Some(FileItem { name, path, is_dir })
            })
            .collect();

        items.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });

        files.extend(items);
        Ok(files)
    }

    /// Check if we should show drives instead of directory contents
    fn should_show_drives(current_dir: &PathBuf) -> bool {
        #[cfg(windows)]
        {
            // On Windows, only show drives when we're at the special "DRIVES:" path
            current_dir.to_string_lossy() == "DRIVES:"
        }
        #[cfg(not(windows))]
        {
            let _ = current_dir; // Suppress unused variable warning
            false
        }
    }

    /// Load available drives on Windows
    pub fn load_drives() -> Result<Vec<FileItem>> {
        #[cfg(windows)]
        {
            let mut drives = Vec::new();

            // Try common drive letters and check if they exist
            for letter in 'A'..='Z' {
                let drive_path = format!("{}:\\", letter);
                let path = PathBuf::from(&drive_path);

                // Check if the drive is accessible by trying to read its metadata
                if path.exists() && path.is_dir() {
                    drives.push(FileItem {
                        name: drive_path.clone(),
                        path,
                        is_dir: true,
                    });
                }
            }

            Ok(drives)
        }
        #[cfg(not(windows))]
        {
            Ok(Vec::new())
        }
    }
}
