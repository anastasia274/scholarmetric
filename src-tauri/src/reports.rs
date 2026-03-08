use crate::db::DbState;
use crate::models::*;
use sqlx::SqlitePool;
use tauri::State;

type CmdResult<T> = Result<T, String>;

fn map_err(e: impl std::fmt::Display) -> String {
    e.to_string()
}

fn classify_student(grades: &[GradeCell]) -> String {
    if grades.is_empty() {
        return "not_attested".to_string();
    }

    let mut has_null = false;
    let mut has_failing = false;
    let mut has_satisfactory = false;
    let mut all_excellent = true;

    for g in grades {
        match &g.value {
            None => has_null = true,
            Some(v) => match v.as_str() {
                "2" | "-" | "absent" => has_failing = true,
                "3" => {
                    has_satisfactory = true;
                    all_excellent = false;
                }
                "4" => all_excellent = false,
                "5" | "+" => {}
                _ => {}
            },
        }
    }

    if has_failing {
        "failing".to_string()
    } else if has_null {
        "not_attested".to_string()
    } else if has_satisfactory {
        "satisfactory".to_string()
    } else if all_excellent {
        "excellent".to_string()
    } else {
        "good".to_string()
    }
}

pub async fn build_group_report(
    pool: &SqlitePool,
    group_id: i64,
    semester: i32,
) -> Result<GroupReport, String> {
    let group_name: String = sqlx::query_scalar("SELECT name FROM groups WHERE id = ?")
        .bind(group_id)
        .fetch_one(pool)
        .await
        .map_err(map_err)?;

    let subject_rows = sqlx::query_as::<_, SubjectHeaderRow>(
        "SELECT gs.id as group_subject_id, s.name, gs.type
         FROM group_subjects gs
         JOIN subjects s ON s.id = gs.subject_id
         WHERE gs.group_id = ? AND gs.semester = ?
         ORDER BY s.name",
    )
    .bind(group_id)
    .bind(semester)
    .fetch_all(pool)
    .await
    .map_err(map_err)?;

    let subjects: Vec<SubjectHeader> = subject_rows.into_iter().map(SubjectHeader::from).collect();

    let students_info = sqlx::query_as::<_, StudentInfo>(
        "SELECT id, last_name, first_name, patronymic
         FROM students WHERE group_id = ?
         ORDER BY last_name, first_name",
    )
    .bind(group_id)
    .fetch_all(pool)
    .await
    .map_err(map_err)?;

    let mut student_rows = Vec::new();
    let mut stats = ReportStats {
        excellent: 0,
        good: 0,
        satisfactory: 0,
        failing: 0,
        not_attested: 0,
        total: students_info.len(),
    };

    for si in &students_info {
        let mut grade_cells = Vec::new();
        for subj in &subjects {
            let value: Option<String> = sqlx::query_scalar(
                "SELECT value FROM grades WHERE student_id = ? AND group_subject_id = ?",
            )
            .bind(si.id)
            .bind(subj.group_subject_id)
            .fetch_optional(pool)
            .await
            .map_err(map_err)?
            .flatten();

            grade_cells.push(GradeCell {
                group_subject_id: subj.group_subject_id,
                subject_name: subj.name.clone(),
                subject_type: subj.subject_type.clone(),
                value,
            });
        }

        let category = classify_student(&grade_cells);
        match category.as_str() {
            "excellent" => stats.excellent += 1,
            "good" => stats.good += 1,
            "satisfactory" => stats.satisfactory += 1,
            "failing" => stats.failing += 1,
            "not_attested" => stats.not_attested += 1,
            _ => {}
        }

        student_rows.push(StudentRow {
            student_id: si.id,
            last_name: si.last_name.clone(),
            first_name: si.first_name.clone(),
            patronymic: si.patronymic.clone(),
            grades: grade_cells,
            category,
        });
    }

    Ok(GroupReport {
        group_name,
        semester,
        subjects,
        students: student_rows,
        stats,
    })
}

#[tauri::command]
pub async fn get_group_report(
    state: State<'_, DbState>,
    group_id: i64,
    semester: i32,
) -> CmdResult<GroupReport> {
    build_group_report(&state.0, group_id, semester).await
}

pub async fn build_overall_ranking(
    pool: &SqlitePool,
    semester: i32,
) -> Result<OverallRanking, String> {
    let all_students = sqlx::query_as::<_, StudentWithGroup>(
        "SELECT DISTINCT s.id, s.last_name, s.first_name, s.patronymic, g.name as group_name, s.group_id
         FROM students s
         JOIN groups g ON g.id = s.group_id
         JOIN group_subjects gs ON gs.group_id = s.group_id AND gs.semester = ?
         ORDER BY s.last_name, s.first_name",
    )
    .bind(semester)
    .fetch_all(pool)
    .await
    .map_err(map_err)?;

    let mut ranking = OverallRanking {
        semester,
        excellent: Vec::new(),
        good: Vec::new(),
        satisfactory: Vec::new(),
        failing: Vec::new(),
        not_attested: Vec::new(),
    };

    for student in &all_students {
        let subject_rows = sqlx::query_as::<_, SubjectHeaderRow>(
            "SELECT gs.id as group_subject_id, s.name, gs.type
             FROM group_subjects gs
             JOIN subjects s ON s.id = gs.subject_id
             WHERE gs.group_id = ? AND gs.semester = ?",
        )
        .bind(student.group_id)
        .bind(semester)
        .fetch_all(pool)
        .await
        .map_err(map_err)?;

        let subjects: Vec<SubjectHeader> = subject_rows.into_iter().map(SubjectHeader::from).collect();

        let mut grade_cells = Vec::new();
        for subj in &subjects {
            let value: Option<String> = sqlx::query_scalar(
                "SELECT value FROM grades WHERE student_id = ? AND group_subject_id = ?",
            )
            .bind(student.id)
            .bind(subj.group_subject_id)
            .fetch_optional(pool)
            .await
            .map_err(map_err)?
            .flatten();

            grade_cells.push(GradeCell {
                group_subject_id: subj.group_subject_id,
                subject_name: subj.name.clone(),
                subject_type: subj.subject_type.clone(),
                value,
            });
        }

        let category = classify_student(&grade_cells);
        let entry = RankingEntry {
            last_name: student.last_name.clone(),
            first_name: student.first_name.clone(),
            patronymic: student.patronymic.clone(),
            group_name: student.group_name.clone(),
        };

        match category.as_str() {
            "excellent" => ranking.excellent.push(entry),
            "good" => ranking.good.push(entry),
            "satisfactory" => ranking.satisfactory.push(entry),
            "failing" => ranking.failing.push(entry),
            "not_attested" => ranking.not_attested.push(entry),
            _ => {}
        }
    }

    Ok(ranking)
}

#[tauri::command]
pub async fn get_overall_ranking(
    state: State<'_, DbState>,
    semester: i32,
) -> CmdResult<OverallRanking> {
    build_overall_ranking(&state.0, semester).await
}
