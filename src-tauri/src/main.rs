// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod category_service;
mod migration_service;
mod models;
mod report_service;
mod shared_service;
mod state;
mod transaction_service;

use rusqlite::Connection;
use state::{AppState, ServiceAccess};
use std::fs;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
#[cfg(not(tarpaulin_include))]
fn get_categories(handle: AppHandle) -> Result<Vec<models::Category>, String> {
    handle
        .db(|db| category_service::get_categories(db))
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
fn delete_category(handle: AppHandle, id: i32) -> Result<(), String> {
    handle
        .db(|db| category_service::delete_category(db, id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
fn insert_category(handle: AppHandle, label: &str) -> Result<(), String> {
    handle
        .db(|db| category_service::insert_category(db, label))
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
fn get_transactions(
    handle: AppHandle,
    page_size: i32,
    current_page: i32,
) -> Result<models::Page, String> {
    handle
        .db(|db| transaction_service::query_page(db, page_size, current_page))
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
fn delete_transaction(handle: AppHandle, id: i32) -> Result<(), String> {
    handle
        .db(|db| transaction_service::delete_transaction(db, id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
fn insert_transaction(
    handle: AppHandle,
    value: f64,
    name: &str,
    description: Option<&str>,
    date_created: &str,
    category_ids: Vec<i32>,
) -> Result<(), String> {
    handle
        .db(|db| {
            transaction_service::insert_transaction(
                db,
                value,
                name,
                description,
                date_created,
                category_ids,
            )
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
fn get_report_types() -> Result<Vec<models::ReportType>, ()> {
    report_service::get_supported_report_types()
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
fn get_basic_report(handle: AppHandle, report_type: models::ReportType, selected_date: &str) -> Result<models::BasicReport, String> {
    handle
        .db(|db| {
            report_service::get_basic_report(
                db,
                report_type,
                selected_date,
            )
        })
        .map_err(|e| e.to_string())
}

#[cfg(not(tarpaulin_include))]
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let app_dir = handle
                .path_resolver()
                .app_data_dir()
                .expect("The app data directory should exist.");

            println!("App data dir: {:?}", app_dir);

            fs::create_dir_all(&app_dir).expect("The app data directory should be created.");

            let sqlite_path = app_dir.join("finance-app.sqlite");

            let db = Connection::open(sqlite_path)?;
            migration_service::init_tables(&db).expect("Failed to initialize database");

            *app_state.db.lock().unwrap() = Some(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_categories,
            delete_category,
            insert_category,
            get_transactions,
            delete_transaction,
            insert_transaction,
            get_report_types,
            get_basic_report
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
