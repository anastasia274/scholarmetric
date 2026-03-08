use crate::db::{get_db_path, DbState};
use crate::reports::{build_group_report, build_overall_ranking};
use rust_xlsxwriter::*;
use tauri::{AppHandle, Manager, State};

type CmdResult<T> = Result<T, String>;

fn map_err(e: impl std::fmt::Display) -> String {
    e.to_string()
}

#[tauri::command]
pub async fn export_db(app: AppHandle, dest_path: String) -> CmdResult<()> {
    let db_path = get_db_path(&app);
    std::fs::copy(&db_path, &dest_path).map_err(map_err)?;
    Ok(())
}

#[tauri::command]
pub async fn import_db(app: AppHandle, source_path: String) -> CmdResult<()> {
    let db_path = get_db_path(&app);

    // Validate the source is a valid SQLite DB with our tables
    let test_url = format!("sqlite:{}", source_path);
    let test_pool = sqlx::SqlitePool::connect(&test_url).await.map_err(map_err)?;
    sqlx::query("SELECT 1 FROM groups LIMIT 1")
        .execute(&test_pool)
        .await
        .map_err(|_| "Invalid database file: missing required tables".to_string())?;
    test_pool.close().await;

    // Close current pool, copy file, reopen
    let state: State<DbState> = app.state();
    state.0.close().await;

    std::fs::copy(&source_path, &db_path).map_err(map_err)?;

    // We can't replace the pool in State, so the app needs a restart after import.
    // For now, just signal success — the user should restart.
    Ok(())
}

#[tauri::command]
pub async fn export_group_report_xlsx(
    state: State<'_, DbState>,
    group_id: i64,
    semester: i32,
    dest_path: String,
) -> CmdResult<()> {
    let report = build_group_report(&state.0, group_id, semester).await?;

    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet
        .set_name(format!("{} сем.{}", report.group_name, report.semester))
        .map_err(map_err)?;

    let header_fmt = Format::new().set_bold();
    let center_fmt = Format::new().set_align(FormatAlign::Center);

    sheet.write_with_format(0, 0, "№", &header_fmt).map_err(map_err)?;
    sheet.write_with_format(0, 1, "ФИО", &header_fmt).map_err(map_err)?;

    for (i, subj) in report.subjects.iter().enumerate() {
        let col = (i + 2) as u16;
        let label = format!(
            "{} ({})",
            subj.name,
            if subj.subject_type == "exam" { "экз." } else { "зач." }
        );
        sheet.write_with_format(0, col, &label, &header_fmt).map_err(map_err)?;
    }

    for (row_idx, student) in report.students.iter().enumerate() {
        let row = (row_idx + 1) as u32;
        sheet.write(row, 0, (row_idx + 1) as u32).map_err(map_err)?;

        let fio = match &student.patronymic {
            Some(p) => format!("{} {} {}", student.last_name, student.first_name, p),
            None => format!("{} {}", student.last_name, student.first_name),
        };
        sheet.write(row, 1, &fio).map_err(map_err)?;

        for (i, grade) in student.grades.iter().enumerate() {
            let col = (i + 2) as u16;
            let display = match &grade.value {
                Some(v) => match v.as_str() {
                    "absent" => "н/я".to_string(),
                    other => other.to_string(),
                },
                None => "—".to_string(),
            };
            sheet.write_with_format(row, col, &display, &center_fmt).map_err(map_err)?;
        }
    }

    let stats_row = (report.students.len() + 2) as u32;
    sheet.write(stats_row, 0, "Статистика:").map_err(map_err)?;
    sheet.write(stats_row + 1, 0, format!("Отличники: {}", report.stats.excellent)).map_err(map_err)?;
    sheet.write(stats_row + 2, 0, format!("Хорошисты: {}", report.stats.good)).map_err(map_err)?;
    sheet.write(stats_row + 3, 0, format!("Троечники: {}", report.stats.satisfactory)).map_err(map_err)?;
    sheet.write(stats_row + 4, 0, format!("Неуспевающие: {}", report.stats.failing)).map_err(map_err)?;
    sheet.write(stats_row + 5, 0, format!("Не аттестованы: {}", report.stats.not_attested)).map_err(map_err)?;

    sheet.autofit();
    workbook.save(&dest_path).map_err(map_err)?;
    Ok(())
}

#[tauri::command]
pub async fn export_ranking_xlsx(
    state: State<'_, DbState>,
    semester: i32,
    dest_path: String,
) -> CmdResult<()> {
    let ranking = build_overall_ranking(&state.0, semester).await?;

    let mut workbook = Workbook::new();
    let header_fmt = Format::new().set_bold();

    let categories: [(&str, &Vec<_>); 5] = [
        ("Отличники", &ranking.excellent),
        ("Хорошисты", &ranking.good),
        ("Троечники", &ranking.satisfactory),
        ("Неуспевающие", &ranking.failing),
        ("Не аттестованы", &ranking.not_attested),
    ];

    for (sheet_name, entries) in &categories {
        let sheet = workbook.add_worksheet();
        sheet.set_name(*sheet_name).map_err(map_err)?;

        sheet.write_with_format(0, 0, "№", &header_fmt).map_err(map_err)?;
        sheet.write_with_format(0, 1, "ФИО", &header_fmt).map_err(map_err)?;
        sheet.write_with_format(0, 2, "Группа", &header_fmt).map_err(map_err)?;

        for (i, entry) in entries.iter().enumerate() {
            let row = (i + 1) as u32;
            sheet.write(row, 0, (i + 1) as u32).map_err(map_err)?;

            let fio = match &entry.patronymic {
                Some(p) => format!("{} {} {}", entry.last_name, entry.first_name, p),
                None => format!("{} {}", entry.last_name, entry.first_name),
            };
            sheet.write(row, 1, &fio).map_err(map_err)?;
            sheet.write(row, 2, &entry.group_name).map_err(map_err)?;
        }

        sheet.autofit();
    }

    workbook.save(&dest_path).map_err(map_err)?;
    Ok(())
}
