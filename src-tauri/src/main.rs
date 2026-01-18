#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextItem {
    id: u64,
    content: String,
    created_at: String,
}

const DATA_FILE: &str = "data.json";

fn get_data_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push(DATA_FILE);
    path
}

#[tauri::command]
async fn load_texts() -> Result<Vec<TextItem>, String> {
    let data_path = get_data_path();
    
    if !data_path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(data_path)
        .map_err(|e| e.to_string())?;

    serde_json::from_str(&content).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_text(content: String) -> Result<TextItem, String> {
    let data_path = get_data_path();
    let mut texts: Vec<TextItem> = if data_path.exists() {
        let content = fs::read_to_string(&data_path)
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())?
    } else {
        Vec::new()
    };
    
    let new_id = texts.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let new_item = TextItem {
        id: new_id,
        content,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    texts.push(new_item.clone());
    
    let json = serde_json::to_string(&texts)
        .map_err(|e| e.to_string())?;
    fs::write(&data_path, json).map_err(|e| e.to_string())?;

    Ok(new_item)
}

#[tauri::command]
async fn update_text(id: u64, content: String) -> Result<TextItem, String> {
    let data_path = get_data_path();
    let mut texts: Vec<TextItem> = if data_path.exists() {
        let content = fs::read_to_string(&data_path)
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())?
    } else {
        Vec::new()
    };
    
    let position = texts.iter()
        .position(|t| t.id == id)
        .ok_or("Text not found")?;

    texts[position].content = content;

    let json = serde_json::to_string(&texts)
        .map_err(|e| e.to_string())?;
    fs::write(&data_path, json).map_err(|e| e.to_string())?;

    Ok(texts[position].clone())
}

#[tauri::command]
async fn delete_text(id: u64) -> Result<(), String> {
    let data_path = get_data_path();
    let mut texts: Vec<TextItem> = if data_path.exists() {
        let content = fs::read_to_string(&data_path)
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())?
    } else {
        Vec::new()
    };
    
    texts.retain(|t| t.id != id);
    
    let json = serde_json::to_string(&texts)
        .map_err(|e| e.to_string())?;
    fs::write(&data_path, json).map_err(|e| e.to_string())?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_texts,
            add_text,
            update_text,
            delete_text
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
