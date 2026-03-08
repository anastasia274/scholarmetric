<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import Select from "primevue/select";
import InputNumber from "primevue/inputnumber";
import { useToast } from "primevue/usetoast";
import { api, type Group, type Student, type GroupSubject } from "../api";

const groups = ref<Group[]>([]);
const selectedGroupId = ref<number | null>(null);
const semester = ref(1);
const students = ref<Student[]>([]);
const groupSubjects = ref<GroupSubject[]>([]);
const gradesMap = ref<Record<string, string | null>>({});
const toast = useToast();

onMounted(async () => {
  groups.value = await api.getGroups();
});

watch([selectedGroupId, semester], loadData);

async function loadData() {
  if (!selectedGroupId.value) return;
  students.value = await api.getStudents(selectedGroupId.value);
  groupSubjects.value = await api.getGroupSubjects(selectedGroupId.value, semester.value);
  const grades = await api.getGradesForGroup(selectedGroupId.value, semester.value);
  const map: Record<string, string | null> = {};
  for (const g of grades) {
    map[`${g.student_id}_${g.group_subject_id}`] = g.value;
  }
  gradesMap.value = map;
}

function getGrade(studentId: number, gsId: number): string | null {
  return gradesMap.value[`${studentId}_${gsId}`] ?? null;
}

function getDisplayValue(val: string | null): string {
  if (val === null) return "—";
  if (val === "absent") return "н/я";
  return val;
}

function getOptions(type: string): { label: string; value: string | null }[] {
  if (type === "exam") {
    return [
      { label: "5", value: "5" },
      { label: "4", value: "4" },
      { label: "3", value: "3" },
      { label: "2", value: "2" },
      { label: "н/я", value: "absent" },
      { label: "—", value: null },
    ];
  }
  return [
    { label: "+", value: "+" },
    { label: "−", value: "-" },
    { label: "н/я", value: "absent" },
    { label: "—", value: null },
  ];
}

const activeCell = ref<{ studentId: number; gsId: number } | null>(null);

function toggleMenu(studentId: number, gsId: number) {
  if (activeCell.value?.studentId === studentId && activeCell.value?.gsId === gsId) {
    activeCell.value = null;
  } else {
    activeCell.value = { studentId, gsId };
  }
}

async function selectGrade(studentId: number, gsId: number, value: string | null) {
  try {
    await api.setGrade(studentId, gsId, value);
    gradesMap.value[`${studentId}_${gsId}`] = value;
    activeCell.value = null;
  } catch (e: any) {
    toast.add({ severity: "error", summary: "Ошибка", detail: e, life: 4000 });
  }
}

function getCellClass(val: string | null): string {
  if (val === null) return "cell-empty";
  if (val === "5" || val === "+") return "cell-excellent";
  if (val === "4") return "cell-good";
  if (val === "3") return "cell-satisfactory";
  if (val === "2" || val === "-" || val === "absent") return "cell-failing";
  return "";
}

function isLastRows(studentId: number): boolean {
  const idx = students.value.findIndex((s) => s.id === studentId);
  return idx >= students.value.length - 2;
}

function fio(s: Student): string {
  const parts = [s.last_name, s.first_name];
  if (s.patronymic) parts.push(s.patronymic);
  return parts.join(" ");
}
</script>

<template>
  <h2>Ввод оценок</h2>

  <div class="toolbar">
    <Select
      v-model="selectedGroupId"
      :options="groups"
      optionLabel="name"
      optionValue="id"
      placeholder="Выберите группу"
      style="width: 200px"
    />
    <label>Семестр:</label>
    <InputNumber v-model="semester" :min="1" :max="12" showButtons style="width: 120px" />
  </div>

  <div v-if="selectedGroupId && groupSubjects.length" class="table-wrapper">
    <table class="grades-table">
      <thead>
        <tr>
          <th class="th-name">Студент</th>
          <th v-for="gs in groupSubjects" :key="gs.id" class="th-subject">
            {{ gs.subject_name }}
            <br /><small>{{ gs.type === 'exam' ? 'экз.' : 'зач.' }}</small>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="student in students" :key="student.id">
          <td class="td-name">{{ fio(student) }}</td>
          <td
            v-for="gs in groupSubjects"
            :key="gs.id"
            class="td-grade"
            :class="getCellClass(getGrade(student.id, gs.id))"
          >
            <div class="grade-cell" @click="toggleMenu(student.id, gs.id)">
              {{ getDisplayValue(getGrade(student.id, gs.id)) }}
            </div>
            <div
              v-if="activeCell?.studentId === student.id && activeCell?.gsId === gs.id"
              class="grade-menu"
              :class="{ 'grade-menu-up': isLastRows(student.id) }"
            >
              <button
                v-for="opt in getOptions(gs.type)"
                :key="String(opt.value)"
                class="grade-option"
                :class="{ active: getGrade(student.id, gs.id) === opt.value }"
                @click="selectGrade(student.id, gs.id, opt.value)"
              >
                {{ opt.label }}
              </button>
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
  <p v-else-if="selectedGroupId" class="hint">
    Нет предметов для этой группы в выбранном семестре. Назначьте предметы в разделе «Учебный план».
  </p>
  <p v-else class="hint">Выберите группу для ввода оценок.</p>
</template>

<style scoped>
.toolbar {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
  align-items: center;
}
.hint {
  margin-top: 2rem;
  color: #888;
}
.table-wrapper {
  overflow-x: auto;
  margin-top: 1rem;
  padding-bottom: 3rem;
}
.grades-table {
  border-collapse: collapse;
  width: 100%;
  font-size: 0.9rem;
}
.grades-table th,
.grades-table td {
  border: 1px solid #ddd;
  padding: 0.5rem;
  text-align: center;
}
.th-name,
.td-name {
  text-align: left;
  white-space: nowrap;
  min-width: 200px;
}
.th-subject {
  min-width: 100px;
}
.th-subject small {
  color: #888;
  font-weight: normal;
}
.td-grade {
  position: relative;
  cursor: pointer;
  min-width: 60px;
}
.grade-cell {
  padding: 0.25rem;
  font-weight: 600;
}
.cell-excellent { background: #e8f5e9; }
.cell-good { background: #e3f2fd; }
.cell-satisfactory { background: #fff3e0; }
.cell-failing { background: #ffebee; }
.cell-empty { background: #fafafa; color: #bbb; }

.grade-menu {
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  background: white;
  border: 1px solid #ccc;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  display: flex;
  gap: 2px;
  padding: 4px;
  z-index: 100;
}
.grade-menu-up {
  top: auto;
  bottom: 100%;
}
.grade-option {
  border: none;
  background: #f5f5f5;
  padding: 0.35rem 0.6rem;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 600;
  font-size: 0.9rem;
}
.grade-option:hover {
  background: #e0e0e0;
}
.grade-option.active {
  background: var(--p-primary-color, #3b82f6);
  color: white;
}
</style>
