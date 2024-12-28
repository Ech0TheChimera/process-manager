// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use::std::env;
use sysinfo::{System};
use serde::{Serialize, Deserialize};

#[tauri::command]
fn os_name() -> &'static str {
    env::consts::OS
}

#[derive(Serialize, Deserialize)]
struct ProcessInfo {
    id: String,
    name: String
}

#[tauri::command]
fn list_process() -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.processes()
        .iter()
        .map(|(id, process)| {
            ProcessInfo {
                id: id.to_string(),
                name: process.name().to_string_lossy().into_owned()
            }
        }).collect()
}

#[tauri::command]
fn kill_by_id(id: &str) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.processes()
        .iter()
        .find(|(pid, _)| pid.to_string().eq_ignore_ascii_case(id))
        .map_or(false, |(_,process)| process.kill())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![os_name, list_process, kill_by_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
