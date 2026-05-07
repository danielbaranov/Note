mod database;

use tauri::Manager;

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let db = tauri::async_runtime::block_on(database::connect())?;
    app.manage(db);
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
            database::save_object_content
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
