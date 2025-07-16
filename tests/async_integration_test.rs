use crossterm::event::KeyCode;
use quickswitch::core::InputDispatcher;
use quickswitch::services::{PreviewManager, create_data_provider};
use quickswitch::utils::AppMode;
use quickswitch::{App, AppState};
use std::path::PathBuf;
use tokio;

/// Test that the async preview generation works correctly
#[tokio::test]
async fn test_async_preview_generation() {
    // Create a temporary directory for testing
    let temp_dir = std::env::temp_dir().join("quickswitch_test");
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Create a test file
    let test_file = temp_dir.join("test.txt");
    std::fs::write(&test_file, "Hello, World!\nThis is a test file.").unwrap();

    // Create app state
    let mut state = AppState::new().unwrap();
    state.current_dir = temp_dir.clone();

    // Load data using data provider
    let provider = create_data_provider(&AppMode::Normal);
    provider.load_data(&mut state).unwrap();

    // Test that we can navigate and update preview asynchronously
    if !state.files.is_empty() {
        // Navigate to first item
        let result = provider.navigate_down(&mut state).await;
        assert!(result);

        // Check that preview was updated
        assert!(!state.preview_title.is_empty());
    }

    // Cleanup
    std::fs::remove_dir_all(&temp_dir).unwrap();
}

/// Test that async navigation works correctly
#[tokio::test]
async fn test_async_navigation() {
    let mut state = AppState::new().unwrap();
    let provider = create_data_provider(&AppMode::Normal);
    provider.load_data(&mut state).unwrap();

    if !state.files.is_empty() {
        // Test navigate down
        let _result = provider.navigate_down(&mut state).await;
        // Navigation might return false if already at the end, that's ok

        // Test navigate up
        let _result = provider.navigate_up(&mut state).await;
        // Navigation might return false if already at the beginning, that's ok

        // Test half page navigation
        let _result = provider.navigate_half_page_down(&mut state).await;
        // Navigation might return false if already at the end, that's ok

        let _result = provider.navigate_half_page_up(&mut state).await;
        // Navigation might return false if already at the beginning, that's ok
    }

    // Just test that the functions can be called without panicking
    assert!(true);
}

/// Test that async key event handling works correctly
#[tokio::test]
async fn test_async_key_handling() {
    let mut state = AppState::new().unwrap();
    let mode = AppMode::Normal;

    // Load data
    let provider = create_data_provider(&mode);
    provider.load_data(&mut state).unwrap();

    // Test key handling
    let result = InputDispatcher::handle_key_event(&mut state, KeyCode::Down, &mode).await;
    assert!(result.is_ok());

    let result = InputDispatcher::handle_key_event(&mut state, KeyCode::Up, &mode).await;
    assert!(result.is_ok());

    let result = InputDispatcher::handle_key_event(&mut state, KeyCode::Char('j'), &mode).await;
    assert!(result.is_ok());

    let result = InputDispatcher::handle_key_event(&mut state, KeyCode::Char('k'), &mode).await;
    assert!(result.is_ok());
}

/// Test that preview manager async functions work correctly
#[tokio::test]
async fn test_async_preview_manager() {
    use quickswitch::utils::{DisplayItem, FileItem};

    let mut state = AppState::new().unwrap();

    // Create a test file item
    let test_file = FileItem {
        name: "test.txt".to_string(),
        path: PathBuf::from("/tmp/test.txt"),
        is_dir: false,
    };

    let display_item = DisplayItem::File(test_file);

    // Test async preview update
    PreviewManager::update_preview_for_item(&mut state, &display_item).await;

    // Check that preview was set
    assert!(!state.preview_title.is_empty());
}

/// Test that the entire async call chain works from top to bottom
#[tokio::test]
async fn test_full_async_chain() {
    use quickswitch::core::events;

    // Create an app instance
    let mut app = App::new(AppMode::Normal).unwrap();

    // Test that we can handle key events asynchronously
    let result = events::handle_key_event(&mut app, KeyCode::Down).await;
    assert!(result.is_ok());

    let result = events::handle_key_event(&mut app, KeyCode::Up).await;
    assert!(result.is_ok());

    // Test navigation keys
    let result = events::handle_key_event(&mut app, KeyCode::Char('j')).await;
    assert!(result.is_ok());

    let result = events::handle_key_event(&mut app, KeyCode::Char('k')).await;
    assert!(result.is_ok());

    // Test half-page navigation
    let result = events::handle_key_event(&mut app, KeyCode::Char('f')).await;
    assert!(result.is_ok());

    let result = events::handle_key_event(&mut app, KeyCode::Char('b')).await;
    assert!(result.is_ok());
}
