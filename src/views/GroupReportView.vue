<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import Select from "primevue/select";
import InputNumber from "primevue/inputnumber";
import Button from "primevue/button";
import Tag from "primevue/tag";
import { useToast } from "primevue/usetoast";
import { save as saveDialog } from "@tauri-apps/plugin-dialog";
import { api, type Group, type GroupReport } from "../api";

const groups = ref<Group[]>([]);
const selectedGroupId = ref<number | null>(null);
const semester = ref(1);
const report = ref<GroupReport | null>(null);
const toast = useToast();

onMounted(async () => {
  groups.value = await api.getGroups();
});

watch([selectedGroupId, semester], loadReport);

async function loadReport() {
  if (!selectedGroupId.value) {
    report.value = null;
    return;
  }
  try {
    report.value = await api.getGroupReport(selectedGroupId.value, semester.value);
  } catch (e: any) {
    toast.add({ severity: "error", summary: "Ошибка", detail: e, life: 4000 });
  }
}

function getDisplayValue(val: string | null): string {
  if (val === null) return "—";
  if (val === "absent") return "н/я";
  return val;
}

function getCellClass(val: string | null): string {
  if (val === null) return "cell-empty";
  if (val === "5" || val === "+") return "cell-excellent";
  if (val === "4") return "cell-good";
  if (val === "3") return "cell-satisfactory";
  if (val === "2" || val === "-" || val === "absent") return "cell-failing";
  return "";
}

function categoryLabel(cat: string): string {
  const m: Record<string, string> = {
    excellent: "Отличник",
    good: "Хорошист",
    satisfactory: "Троечник",
    failing: "Неуспевающий",
    not_attested: "Не аттестован",
  };
  return m[cat] || cat;
}

function categorySeverity(cat: string): string {
  const m: Record<string, string> = {
    excellent: "success",
    good: "info",
    satisfactory: "warn",
    failing: "danger",
    not_attested: "secondary",
  };
  return m[cat] || "secondary";
}

function fio(s: { last_name: string; first_name: string; patronymic: string | null }): string {
  const parts = [s.last_name, s.first_name];
  if (s.patronymic) parts.push(s.patronymic);
  return parts.join(" ");
}

function pct(n: number, total: number): string {
  if (total === 0) return "0";
  return ((n / total) * 100).toFixed(0);
}

async function exportXlsx() {
  if (!selectedGroupId.value || !report.value) return;
  const path = await saveDialog({
    defaultPath: `${report.value.group_name}_сем${semester.value}.xlsx`,
    filters: [{ name: "Excel", extensions: ["xlsx"] }],
  });
  if (!path) return;
  try {
    await api.exportGroupReportXlsx(selectedGroupId.value, semester.value, path);
    toast.add({ severity: "success", summary: "Экспортировано", detail: path, life: 3000 });
  } catch (e: any) {
    toast.add({ severity: "error", summary: "Ошибка", detail: e, life: 4000 });
  }
}
</script>

<template>
  <h2>Отчёт по группе</h2>

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
    <Button
      v-if="report"
      label="Экспорт в Excel"
      icon="pi pi-file-excel"
      severity="success"
      outlined
      @click="exportXlsx"
    />
  </div>

  <div v-if="report && report.subjects.length" class="table-wrapper">
    <table class="report-table">
      <thead>
        <tr>
          <th>№</th>
          <th class="th-name">ФИО</th>
          <th v-for="subj in report.subjects" :key="subj.group_subject_id" class="th-subject">
            {{ subj.name }}
            <br /><small>{{ subj.type === 'exam' ? 'экз.' : 'зач.' }}</small>
          </th>
          <th>Статус</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(student, idx) in report.students" :key="student.student_id">
          <td>{{ idx + 1 }}</td>
          <td class="td-name">{{ fio(student) }}</td>
          <td
            v-for="grade in student.grades"
            :key="grade.group_subject_id"
            class="td-grade"
            :class="getCellClass(grade.value)"
          >
            {{ getDisplayValue(grade.value) }}
          </td>
          <td>
            <Tag :value="categoryLabel(student.category)" :severity="categorySeverity(student.category) as any" />
          </td>
        </tr>
      </tbody>
    </table>

    <div class="stats-block">
      <h3>Статистика</h3>
      <div class="stats-grid">
        <span>Отличники:</span><strong>{{ report.stats.excellent }} ({{ pct(report.stats.excellent, report.stats.total) }}%)</strong>
        <span>Хорошисты:</span><strong>{{ report.stats.good }} ({{ pct(report.stats.good, report.stats.total) }}%)</strong>
        <span>Троечники:</span><strong>{{ report.stats.satisfactory }} ({{ pct(report.stats.satisfactory, report.stats.total) }}%)</strong>
        <span>Неуспевающие:</span><strong>{{ report.stats.failing }} ({{ pct(report.stats.failing, report.stats.total) }}%)</strong>
        <span>Не аттестованы:</span><strong>{{ report.stats.not_attested }} ({{ pct(report.stats.not_attested, report.stats.total) }}%)</strong>
        <span>Всего:</span><strong>{{ report.stats.total }}</strong>
      </div>
    </div>
  </div>
  <p v-else-if="selectedGroupId" class="hint">Нет данных для отображения.</p>
  <p v-else class="hint">Выберите группу.</p>
</template>

<style scoped>
.toolbar {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
  align-items: center;
}
.hint { margin-top: 2rem; color: var(--app-cell-empty-text); }
.table-wrapper { overflow-x: auto; margin-top: 1rem; }
.report-table { border-collapse: collapse; width: 100%; font-size: 0.9rem; }
.report-table th, .report-table td { border: 1px solid var(--app-border); padding: 0.5rem; text-align: center; }
.th-name, .td-name { text-align: left; white-space: nowrap; }
.th-subject { min-width: 90px; }
.th-subject small { color: var(--app-cell-empty-text); font-weight: normal; }
.td-grade { font-weight: 600; }
.cell-excellent { background: var(--app-cell-excellent); }
.cell-good { background: var(--app-cell-good); }
.cell-satisfactory { background: var(--app-cell-satisfactory); }
.cell-failing { background: var(--app-cell-failing); }
.cell-empty { color: var(--app-cell-empty-text); }
.stats-block { margin-top: 1.5rem; padding: 1rem; background: var(--app-stats-bg); border-radius: 8px; }
.stats-block h3 { margin-bottom: 0.75rem; }
.stats-grid { display: grid; grid-template-columns: auto auto; gap: 0.4rem 1rem; max-width: 300px; }
</style>
