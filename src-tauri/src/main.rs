#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod collect;
mod ini;

use std::fs;
use std::path::{Path, PathBuf};
use tauri::Manager;

/// Rutas posibles del .ini: junto al .exe (portable) y, como respaldo, la carpeta de configuración.
fn ini_paths(app: &tauri::AppHandle) -> Vec<PathBuf> {
    let mut v = vec![];
    if let Ok(exe) = std::env::current_exe() { if let Some(d) = exe.parent() { v.push(d.join("bf2_visor.ini")); } }
    if let Ok(d) = app.path().app_config_dir() { let _ = fs::create_dir_all(&d); v.push(d.join("bf2_visor.ini")); }
    v
}

fn load_last(app: &tauri::AppHandle) -> (String, String) {
    for p in ini_paths(app) { if p.is_file() { return ini::read(&p); } }
    (String::new(), String::new())
}

/// Si la app se abrió con "Abrir con..." Windows pasa la ruta como primer argumento.
fn launch_target() -> Option<String> {
    std::env::args().skip(1).find(|a| Path::new(a).is_file())
}

#[tauri::command]
fn get_manifest(app: tauri::AppHandle, target: Option<String>) -> Result<serde_json::Value, String> {
    let t = match target.or_else(launch_target) {
        Some(t) => t,
        None => return Ok(serde_json::json!({ "target": "", "target_ext": "", "ske": "", "arms": "", "files": [] })),
    };
    let tp = PathBuf::from(&t);
    if !tp.is_file() { return Err(format!("No existe el archivo: {}", t)); }
    let (ske, arms) = load_last(&app);
    let f = collect::collect(&tp, &ske, &arms);
    let files: Vec<serde_json::Value> = f.files.iter().map(|(p, a)| serde_json::json!({ "path": p, "abs": a })).collect();
    let ext = tp.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
    Ok(serde_json::json!({ "target": f.target, "target_ext": ext, "ske": f.ske, "arms": f.arms, "files": files }))
}

#[tauri::command]
fn read_file(path: String) -> Result<tauri::ipc::Response, String> {
    fs::read(&path).map(tauri::ipc::Response::new).map_err(|e| format!("{}: {}", path, e))
}

/// Guarda el archivo; la primera vez deja una copia "<archivo>.bak" con el original.
#[tauri::command]
fn write_file(path: String, data: Vec<u8>) -> Result<(), String> {
    let p = PathBuf::from(&path);
    if p.is_file() {
        let mut bak = p.clone().into_os_string();
        bak.push(".bak");
        let bak = PathBuf::from(bak);
        if !bak.exists() { fs::copy(&p, &bak).map_err(|e| format!("No pude crear la copia .bak: {}", e))?; }
    }
    fs::write(&p, data).map_err(|e| format!("{}: {}", path, e))
}

#[tauri::command]
fn save_defaults(app: tauri::AppHandle, key: String, path: String) -> Result<(), String> {
    let (mut ske, mut arms) = load_last(&app);
    match key.as_str() { "ske" => ske = path, "arms" => arms = path, _ => return Ok(()) }
    let mut last_err = String::new();
    for p in ini_paths(&app) {
        match ini::write(&p, &ske, &arms) { Ok(_) => return Ok(()), Err(e) => last_err = e.to_string() }
    }
    Err(last_err)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![get_manifest, read_file, write_file, save_defaults])
        .run(tauri::generate_context!())
        .expect("error al iniciar BF2 Visor");
}
