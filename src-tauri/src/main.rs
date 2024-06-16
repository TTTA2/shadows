// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

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
fn get_templates_file_data() -> Result<Vec<FileInfo>, String> {

    // 読み込むフォルダのパスを指定
    let folder_path = "Books";

    // フォルダ内のテキストファイルをすべて読み込む
    let file_infos = read_all_text_files(folder_path).unwrap();
    // JSONとして出力
    // let json_output = serde_json::to_string_pretty(&file_infos).unwrap();

    Ok(file_infos)
}


#[tauri::command]
fn write_template(template: TemplateInfo, isUpdate: bool) -> Result<String, String> {

    let connection = sqlc::connect("./database.db").unwrap();
    let a = sqlc::write_template(connection, template, isUpdate);
    Ok("test".to_string())
}

fn test() {
    
}


#[derive(Serialize)]
struct FileInfo {
    name: String,
    path: String,
    contents: String,
    parent: Option<String>,
}

fn read_all_text_files<P: AsRef<Path>>(path: P) -> io::Result<Vec<FileInfo>> {

    let mut dirs_to_visit = vec![PathBuf::from(path.as_ref())];
    let mut visited_paths = HashSet::new();
    let mut file_infos = Vec::new();

    while let Some(current_dir) = dirs_to_visit.pop() {
        if visited_paths.contains(&current_dir) {
            continue; // Avoid processing the same directory multiple times
        }
        visited_paths.insert(current_dir.clone());

        match fs::read_dir(&current_dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_dir() {
                                dirs_to_visit.push(path);
                            } else if let Some(extension) = path.extension() {
                                if extension == "txt" {
                                    match read_text_file(&path) {
                                        Ok(file_info) => file_infos.push(file_info),
                                        Err(e) => eprintln!("Failed to read file {:?}: {}", path, e),
                                    }
                                }
                            }
                        }
                        Err(e) => eprintln!("Failed to read entry in {:?}: {}", current_dir, e),
                    }
                }
            }
            Err(e) => eprintln!("Failed to read directory {:?}: {}", current_dir, e),
        }
    }

    Ok(file_infos)
}

fn read_text_file<P: AsRef<Path>>(path: P) -> io::Result<FileInfo> {
    let path = path.as_ref();
    let mut file = fs::File::open(&path)?;
    let mut contents = String::new();

    let parent_path = path.parent().map(|parent| parent.to_string_lossy().to_string());

    file.read_to_string(&mut contents)?;
    
    let file_info = FileInfo {
        name: path.file_name().unwrap().to_string_lossy().to_string(),
        path: path.to_string_lossy().to_string(),
        parent: parent_path,
        contents,
    };
    Ok(file_info)
}


fn main() {



    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_templates_info,
            write_template,
            get_templates_file_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
