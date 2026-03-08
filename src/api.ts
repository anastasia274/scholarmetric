import { invoke } from "@tauri-apps/api/core";

// Types
export interface Group {
  id: number;
  name: string;
}

export interface Subject {
  id: number;
  name: string;
}

export interface Student {
  id: number;
  last_name: string;
  first_name: string;
  patronymic: string | null;
  group_id: number;
}

export interface GroupSubject {
  id: number;
  group_id: number;
  subject_id: number;
  semester: number;
  type: string;
  subject_name: string | null;
}

export interface Grade {
  id: number | null;
  student_id: number;
  group_subject_id: number;
  value: string | null;
}

export interface SubjectHeader {
  group_subject_id: number;
  name: string;
  type: string;
}

export interface GradeCell {
  group_subject_id: number;
  subject_name: string;
  subject_type: string;
  value: string | null;
}

export interface StudentRow {
  student_id: number;
  last_name: string;
  first_name: string;
  patronymic: string | null;
  grades: GradeCell[];
  category: string;
}

export interface ReportStats {
  excellent: number;
  good: number;
  satisfactory: number;
  failing: number;
  not_attested: number;
  total: number;
}

export interface GroupReport {
  group_name: string;
  semester: number;
  subjects: SubjectHeader[];
  students: StudentRow[];
  stats: ReportStats;
}

export interface RankingEntry {
  last_name: string;
  first_name: string;
  patronymic: string | null;
  group_name: string;
}

export interface OverallRanking {
  semester: number;
  excellent: RankingEntry[];
  good: RankingEntry[];
  satisfactory: RankingEntry[];
  failing: RankingEntry[];
  not_attested: RankingEntry[];
}

// API calls
export const api = {
  previewGroupName: (name: string) =>
    invoke<string>("preview_group_name", { name }),

  // Groups
  createGroup: (name: string) => invoke<Group>("create_group", { name }),
  getGroups: () => invoke<Group[]>("get_groups"),
  updateGroup: (id: number, name: string) =>
    invoke<Group>("update_group", { id, name }),
  deleteGroup: (id: number) => invoke<void>("delete_group", { id }),

  // Subjects
  createSubject: (name: string) => invoke<Subject>("create_subject", { name }),
  getSubjects: () => invoke<Subject[]>("get_subjects"),
  updateSubject: (id: number, name: string) =>
    invoke<Subject>("update_subject", { id, name }),
  deleteSubject: (id: number) => invoke<void>("delete_subject", { id }),

  // Students
  createStudent: (
    lastName: string,
    firstName: string,
    patronymic: string | null,
    groupId: number
  ) =>
    invoke<Student>("create_student", {
      lastName,
      firstName,
      patronymic,
      groupId,
    }),
  getStudents: (groupId?: number) =>
    invoke<Student[]>("get_students", { groupId: groupId ?? null }),
  updateStudent: (
    id: number,
    lastName: string,
    firstName: string,
    patronymic: string | null,
    groupId: number
  ) =>
    invoke<Student>("update_student", {
      id,
      lastName,
      firstName,
      patronymic,
      groupId,
    }),
  deleteStudent: (id: number) => invoke<void>("delete_student", { id }),

  // Group subjects
  assignSubject: (
    groupId: number,
    subjectId: number,
    semester: number,
    subjectType: string
  ) =>
    invoke<GroupSubject>("assign_subject", {
      groupId,
      subjectId,
      semester,
      subjectType,
    }),
  getGroupSubjects: (groupId: number, semester: number) =>
    invoke<GroupSubject[]>("get_group_subjects", { groupId, semester }),
  updateGroupSubjectType: (id: number, subjectType: string) =>
    invoke<void>("update_group_subject_type", { id, subjectType }),
  removeGroupSubject: (id: number) =>
    invoke<void>("remove_group_subject", { id }),

  // Grades
  setGrade: (
    studentId: number,
    groupSubjectId: number,
    value: string | null
  ) => invoke<void>("set_grade", { studentId, groupSubjectId, value }),
  getGradesForGroup: (groupId: number, semester: number) =>
    invoke<Grade[]>("get_grades_for_group", { groupId, semester }),

  // Reports
  getGroupReport: (groupId: number, semester: number) =>
    invoke<GroupReport>("get_group_report", { groupId, semester }),
  getOverallRanking: (semester: number) =>
    invoke<OverallRanking>("get_overall_ranking", { semester }),

  // Export/Import
  exportDb: (destPath: string) =>
    invoke<void>("export_db", { destPath }),
  importDb: (sourcePath: string) =>
    invoke<void>("import_db", { sourcePath }),
  exportGroupReportXlsx: (
    groupId: number,
    semester: number,
    destPath: string
  ) =>
    invoke<void>("export_group_report_xlsx", { groupId, semester, destPath }),
  exportRankingXlsx: (semester: number, destPath: string) =>
    invoke<void>("export_ranking_xlsx", { semester, destPath }),
};
