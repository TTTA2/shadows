// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlc::TemplateInfo;
mod sqlc;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet() -> Result<Vec<TemplateInfo>, String> {

    let connection = sqlc::connect("./database.db").unwrap();
    // sqlc::init(connection);

    let a = sqlc::get_templates(connection);
    Ok(a)
}

#[tauri::command]
fn get_templates_info() -> Result<Vec<TemplateInfo>, String> {

    let connection = sqlc::connect("./database.db").unwrap();
    let a = sqlc::get_templates(connection);
    Ok(a)
}


#[tauri::command]
fn write_template(template: TemplateInfo, isUpdate: bool) -> Result<String, String> {

    let connection = sqlc::connect("./database.db").unwrap();
    let a = sqlc::write_template(connection, template, isUpdate);
    Ok("test".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_templates_info,
            write_template,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
