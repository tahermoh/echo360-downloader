use echo360::{Echo360, courses::{Enrollments, Section}};
use std::sync::Mutex;

mod echo360;

type AppState = Mutex<Echo360>;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_courses(echo360: tauri::State<'_, AppState>) -> Vec<Section> {
    let echo360 = echo360.lock().unwrap();
    //let mut enrollments = echo360.enrollments.lock().unwrap();
    let mut enrollments = echo360.enrollments.lock().unwrap();
    *enrollments = Enrollments::get(&echo360.client, &echo360.domain).unwrap();

    dbg!(enrollments.user_sections.clone())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        //.plugin(tauri_plugin_opener::init())
        .manage(dbg!(Mutex::new(Echo360::login().unwrap())))
        .invoke_handler(tauri::generate_handler![greet, get_courses])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
