use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Student {
    pub id: i64,
    pub last_name: String,
    pub first_name: String,
    pub patronymic: Option<String>,
    pub group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Subject {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct GroupSubjectRow {
    pub id: i64,
    pub group_id: i64,
    pub subject_id: i64,
    pub semester: i32,
    #[sqlx(rename = "type")]
    pub subject_type: String,
    pub subject_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupSubject {
    pub id: i64,
    pub group_id: i64,
    pub subject_id: i64,
    pub semester: i32,
    #[serde(rename = "type")]
    pub subject_type: String,
    pub subject_name: Option<String>,
}

impl From<GroupSubjectRow> for GroupSubject {
    fn from(r: GroupSubjectRow) -> Self {
        GroupSubject {
            id: r.id,
            group_id: r.group_id,
            subject_id: r.subject_id,
            semester: r.semester,
            subject_type: r.subject_type,
            subject_name: r.subject_name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Grade {
    pub id: Option<i64>,
    pub student_id: i64,
    pub group_subject_id: i64,
    pub value: Option<String>,
}

// --- Report structures ---

#[derive(Debug, Serialize, Clone)]
pub struct GradeCell {
    pub group_subject_id: i64,
    pub subject_name: String,
    pub subject_type: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct StudentRow {
    pub student_id: i64,
    pub last_name: String,
    pub first_name: String,
    pub patronymic: Option<String>,
    pub grades: Vec<GradeCell>,
    pub category: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct GroupReport {
    pub group_name: String,
    pub semester: i32,
    pub subjects: Vec<SubjectHeader>,
    pub students: Vec<StudentRow>,
    pub stats: ReportStats,
}

#[derive(Debug, Serialize, Clone)]
pub struct SubjectHeader {
    pub group_subject_id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub subject_type: String,
}

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct SubjectHeaderRow {
    pub group_subject_id: i64,
    pub name: String,
    #[sqlx(rename = "type")]
    pub subject_type: String,
}

impl From<SubjectHeaderRow> for SubjectHeader {
    fn from(r: SubjectHeaderRow) -> Self {
        SubjectHeader {
            group_subject_id: r.group_subject_id,
            name: r.name,
            subject_type: r.subject_type,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ReportStats {
    pub excellent: usize,
    pub good: usize,
    pub satisfactory: usize,
    pub failing: usize,
    pub not_attested: usize,
    pub total: usize,
}

#[derive(Debug, Serialize, Clone)]
pub struct RankingEntry {
    pub last_name: String,
    pub first_name: String,
    pub patronymic: Option<String>,
    pub group_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct OverallRanking {
    pub semester: i32,
    pub excellent: Vec<RankingEntry>,
    pub good: Vec<RankingEntry>,
    pub satisfactory: Vec<RankingEntry>,
    pub failing: Vec<RankingEntry>,
    pub not_attested: Vec<RankingEntry>,
}
