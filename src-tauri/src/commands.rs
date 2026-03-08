use crate::db::DbState;
use crate::models::*;
use crate::normalize::{normalize_group_name, normalize_name};
use tauri::State;

type CmdResult<T> = Result<T, String>;

fn map_err(e: impl std::fmt::Display) -> String {
    e.to_string()
}

// ==================== Groups ====================

#[tauri::command]
pub fn preview_group_name(name: String) -> String {
    normalize_group_name(&name)
}

#[tauri::command]
pub async fn create_group(state: State<'_, DbState>, name: String) -> CmdResult<Group> {
    let normalized = normalize_group_name(&name);
    let id = sqlx::query_scalar::<_, i64>("INSERT INTO groups (name) VALUES (?) RETURNING id")
        .bind(&normalized)
        .fetch_one(&state.0)
        .await
        .map_err(map_err)?;
    Ok(Group {
        id,
        name: normalized,
    })
}

#[tauri::command]
pub async fn get_groups(state: State<'_, DbState>) -> CmdResult<Vec<Group>> {
    sqlx::query_as::<_, Group>("SELECT id, name FROM groups ORDER BY name")
        .fetch_all(&state.0)
        .await
        .map_err(map_err)
}

#[tauri::command]
pub async fn update_group(state: State<'_, DbState>, id: i64, name: String) -> CmdResult<Group> {
    let normalized = normalize_group_name(&name);
    sqlx::query("UPDATE groups SET name = ? WHERE id = ?")
        .bind(&normalized)
        .bind(id)
        .execute(&state.0)
        .await
        .map_err(map_err)?;
    Ok(Group {
        id,
        name: normalized,
    })
}

#[tauri::command]
pub async fn delete_group(state: State<'_, DbState>, id: i64) -> CmdResult<()> {
    sqlx::query("DELETE FROM groups WHERE id = ?")
        .bind(id)
        .execute(&state.0)
        .await
        .map_err(map_err)?;
    Ok(())
}

// ==================== Subjects ====================

#[tauri::command]
pub async fn create_subject(state: State<'_, DbState>, name: String) -> CmdResult<Subject> {
    let trimmed = name.trim().to_string();
    let id = sqlx::query_scalar::<_, i64>("INSERT INTO subjects (name) VALUES (?) RETURNING id")
        .bind(&trimmed)
        .fetch_one(&state.0)
        .await
        .map_err(map_err)?;
    Ok(Subject { id, name: trimmed })
}

#[tauri::command]
pub async fn get_subjects(state: State<'_, DbState>) -> CmdResult<Vec<Subject>> {
    sqlx::query_as::<_, Subject>("SELECT id, name FROM subjects ORDER BY name")
        .fetch_all(&state.0)
        .await
        .map_err(map_err)
}

#[tauri::command]
pub async fn update_subject(
    state: State<'_, DbState>,
    id: i64,
    name: String,
) -> CmdResult<Subject> {
    let trimmed = name.trim().to_string();
    sqlx::query("UPDATE subjects SET name = ? WHERE id = ?")
        .bind(&trimmed)
        .bind(id)
        .execute(&state.0)
        .await
        .map_err(map_err)?;
    Ok(Subject { id, name: trimmed })
}

#[tauri::command]
pub async fn delete_subject(state: State<'_, DbState>, id: i64) -> CmdResult<()> {
    sqlx::query("DELETE FROM subjects WHERE id = ?")
        .bind(id)
        .execute(&state.0)
        .await
        .map_err(map_err)?;
    Ok(())
}

// ==================== Students ====================

#[tauri::command]
pub async fn create_student(
    state: State<'_, DbState>,
    last_name: String,
    first_name: String,
    patronymic: Option<String>,
    group_id: i64,
) -> CmdResult<Student> {
    let ln = normalize_name(&last_name);
    let fn_ = normalize_name(&first_name);
    let pat = patronymic.map(|p| normalize_name(&p));
    let id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO students (last_name, first_name, patronymic, group_id) VALUES (?, ?, ?, ?) RETURNING id",
    )
    .bind(&ln)
    .bind(&fn_)
    .bind(&pat)
    .bind(group_id)
    .fetch_one(&state.0)
    .await
    .map_err(map_err)?;
    Ok(Student {
        id,
        last_name: ln,
        first_name: fn_,
        patronymic: pat,
        group_id,
    })
}

#[tauri::command]
pub async fn get_students(
    state: State<'_, DbState>,
    group_id: Option<i64>,
) -> CmdResult<Vec<Student>> {
    match group_id {
        Some(gid) => {
            sqlx::query_as::<_, Student>(
                "SELECT id, last_name, first_name, patronymic, group_id FROM students WHERE group_id = ? ORDER BY last_name, first_name",
            )
            .bind(gid)
            .fetch_all(&state.0)
            .await
            .map_err(map_err)
        }
        None => {
            sqlx::query_as::<_, Student>(
                "SELECT id, last_name, first_name, patronymic, group_id FROM students ORDER BY last_name, first_name",
            )
            .fetch_all(&state.0)
            .await
            .map_err(map_err)
        }
    }
}

#[tauri::command]
pub async fn update_student(
    state: State<'_, DbState>,
    id: i64,
    last_name: String,
    first_name: String,
    patronymic: Option<String>,
    group_id: i64,
) -> CmdResult<Student> {
    let ln = normalize_name(&last_name);
    let fn_ = normalize_name(&first_name);
    let pat = patronymic.map(|p| normalize_name(&p));
    sqlx::query(
        "UPDATE students SET last_name=?, first_name=?, patronymic=?, group_id=? WHERE id=?",
    )
    .bind(&ln)
    .bind(&fn_)
    .bind(&pat)
    .bind(group_id)
    .bind(id)
    .execute(&state.0)
    .await
    .map_err(map_err)?;
    Ok(Student {
        id,
        last_name: ln,
        first_name: fn_,
        patronymic: pat,
        group_id,
    })
}

#[tauri::command]
pub async fn delete_student(state: State<'_, DbState>, id: i64) -> CmdResult<()> {
    sqlx::query("DELETE FROM students WHERE id = ?")
        .bind(id)
        .execute(&state.0)
        .await
        .map_err(map_err)?;
    Ok(())
}

// ==================== Group Subjects ====================

#[tauri::command]
pub async fn assign_subject(
    state: State<'_, DbState>,
    group_id: i64,
    subject_id: i64,
    semester: i32,
    subject_type: String,
) -> CmdResult<GroupSubject> {
    let id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO group_subjects (group_id, subject_id, semester, type) VALUES (?, ?, ?, ?) RETURNING id",
    )
    .bind(group_id)
    .bind(subject_id)
    .bind(semester)
    .bind(&subject_type)
    .fetch_one(&state.0)
    .await
    .map_err(map_err)?;
    Ok(GroupSubject {
        id,
        group_id,
        subject_id,
        semester,
        subject_type,
        subject_name: None,
    })
}

#[tauri::command]
pub async fn get_group_subjects(
    state: State<'_, DbState>,
    group_id: i64,
    semester: i32,
) -> CmdResult<Vec<GroupSubject>> {
    let rows = sqlx::query_as::<_, GroupSubjectRow>(
        "SELECT gs.id, gs.group_id, gs.subject_id, gs.semester, gs.type, s.name as subject_name
         FROM group_subjects gs
         JOIN subjects s ON s.id = gs.subject_id
         WHERE gs.group_id = ? AND gs.semester = ?
         ORDER BY s.name",
    )
    .bind(group_id)
    .bind(semester)
    .fetch_all(&state.0)
    .await
    .map_err(map_err)?;
    Ok(rows.into_iter().map(GroupSubject::from).collect())
}

#[tauri::command]
pub async fn update_group_subject_type(
    state: State<'_, DbState>,
    id: i64,
    subject_type: String,
) -> CmdResult<()> {
    sqlx::query("UPDATE group_subjects SET type = ? WHERE id = ?")
        .bind(&subject_type)
        .bind(id)
        .execute(&state.0)
        .await
        .map_err(map_err)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_group_subject(state: State<'_, DbState>, id: i64) -> CmdResult<()> {
    sqlx::query("DELETE FROM group_subjects WHERE id = ?")
        .bind(id)
        .execute(&state.0)
        .await
        .map_err(map_err)?;
    Ok(())
}

// ==================== Grades ====================

#[tauri::command]
pub async fn set_grade(
    state: State<'_, DbState>,
    student_id: i64,
    group_subject_id: i64,
    value: Option<String>,
) -> CmdResult<()> {
    match value {
        Some(v) => {
            sqlx::query(
                "INSERT INTO grades (student_id, group_subject_id, value)
                 VALUES (?, ?, ?)
                 ON CONFLICT(student_id, group_subject_id) DO UPDATE SET value = excluded.value",
            )
            .bind(student_id)
            .bind(group_subject_id)
            .bind(&v)
            .execute(&state.0)
            .await
            .map_err(map_err)?;
        }
        None => {
            sqlx::query("DELETE FROM grades WHERE student_id = ? AND group_subject_id = ?")
                .bind(student_id)
                .bind(group_subject_id)
                .execute(&state.0)
                .await
                .map_err(map_err)?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn get_grades_for_group(
    state: State<'_, DbState>,
    group_id: i64,
    semester: i32,
) -> CmdResult<Vec<Grade>> {
    sqlx::query_as::<_, Grade>(
        "SELECT g.id, g.student_id, g.group_subject_id, g.value
         FROM grades g
         JOIN group_subjects gs ON gs.id = g.group_subject_id
         JOIN students s ON s.id = g.student_id
         WHERE s.group_id = ? AND gs.semester = ?",
    )
    .bind(group_id)
    .bind(semester)
    .fetch_all(&state.0)
    .await
    .map_err(map_err)
}
