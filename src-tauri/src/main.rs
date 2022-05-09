#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
async fn write_report(app: tauri::AppHandle) -> Result<(), String> {
  let app_dir = app.path_resolver().app_dir().expect("failed to get app dir");
  let report_path = app_dir.join("report.txt");
  std::fs::write(&report_path, "the file content")
    .map_err(|e| e.to_string());
  Ok(())
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![write_report])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
