mod database;

use tauri::{Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

#[cfg(target_os = "macos")]
#[allow(deprecated)]
fn set_macos_window_background(window: &WebviewWindow) -> tauri::Result<()> {
    use cocoa::appkit::{NSColor, NSWindow};
    use cocoa::base::{id, nil};

    let ns_window = window.ns_window()? as id;

    unsafe {
        let bg_color = NSColor::colorWithRed_green_blue_alpha_(
            nil,
            251.0 / 255.0,
            250.0 / 255.0,
            248.0 / 255.0,
            1.0,
        );
        ns_window.setBackgroundColor_(bg_color);
    }

    Ok(())
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let db = tauri::async_runtime::block_on(database::connect())?;
    app.manage(db);

    let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("Note")
        .inner_size(800.0, 600.0);

    #[cfg(target_os = "macos")]
    let win_builder = win_builder
        .title_bar_style(TitleBarStyle::Overlay)
        .hidden_title(true);

    let window = win_builder.build()?;

    #[cfg(target_os = "macos")]
    set_macos_window_background(&window)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            database::search_objects,
            database::create_markdown_object,
            database::load_object,
            database::update_object,
            database::save_object_content,
            database::delete_object,
            database::list_all_connections,
            database::list_object_connections,
            database::create_object_connection,
            database::update_object_connection_content,
            database::delete_object_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
