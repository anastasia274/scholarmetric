mod commands;
mod db;
mod export;
mod models;
mod normalize;
mod reports;

use db::{get_db_path, init_db, DbState};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let db_path = get_db_path(app.handle());
            let pool = tauri::async_runtime::block_on(init_db(&db_path))
                .expect("failed to initialize database");
            app.manage(DbState(pool));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Groups
            commands::preview_group_name,
            commands::create_group,
            commands::get_groups,
            commands::update_group,
            commands::delete_group,
            // Subjects
            commands::create_subject,
            commands::get_subjects,
            commands::update_subject,
            commands::delete_subject,
            // Students
            commands::create_student,
            commands::get_students,
            commands::update_student,
            commands::delete_student,
            // Group subjects
            commands::assign_subject,
            commands::get_group_subjects,
            commands::update_group_subject_type,
            commands::remove_group_subject,
            // Grades
            commands::set_grade,
            commands::get_grades_for_group,
            // Reports
            reports::get_group_report,
            reports::get_overall_ranking,
            // Export/Import
            export::export_db,
            export::import_db,
            export::export_group_report_xlsx,
            export::export_ranking_xlsx,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
