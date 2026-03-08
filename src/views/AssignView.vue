<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Button from "primevue/button";
import Select from "primevue/select";
import InputNumber from "primevue/inputnumber";
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";
import { api, type Group, type Subject, type GroupSubject } from "../api";

const groups = ref<Group[]>([]);
const subjects = ref<Subject[]>([]);
const selectedGroupId = ref<number | null>(null);
const semester = ref(1);
const groupSubjects = ref<GroupSubject[]>([]);
const addSubjectId = ref<number | null>(null);
const addType = ref("exam");
const confirm = useConfirm();
const toast = useToast();

const typeOptions = [
  { label: "Экзамен", value: "exam" },
  { label: "Зачёт", value: "pass" },
];

onMounted(async () => {
  groups.value = await api.getGroups();
  subjects.value = await api.getSubjects();
});

watch([selectedGroupId, semester], loadAssignments);

async function loadAssignments() {
  if (!selectedGroupId.value) {
    groupSubjects.value = [];
    return;
  }
  groupSubjects.value = await api.getGroupSubjects(selectedGroupId.value, semester.value);
}

async function addSubject() {
  if (!selectedGroupId.value || !addSubjectId.value) return;
  try {
    await api.assignSubject(selectedGroupId.value, addSubjectId.value, semester.value, addType.value);
    await loadAssignments();
    addSubjectId.value = null;
    toast.add({ severity: "success", summary: "Предмет назначен", life: 2000 });
  } catch (e: any) {
    toast.add({ severity: "error", summary: "Ошибка", detail: e, life: 4000 });
  }
}

async function changeType(gs: GroupSubject) {
  const newType = gs.type === "exam" ? "pass" : "exam";
  try {
    await api.updateGroupSubjectType(gs.id, newType);
    await loadAssignments();
  } catch (e: any) {
    toast.add({ severity: "error", summary: "Ошибка", detail: e, life: 4000 });
  }
}

function confirmRemove(gs: GroupSubject) {
  confirm.require({
    message: `Убрать предмет "${gs.subject_name}" из этого семестра? Все оценки будут удалены.`,
    header: "Подтверждение",
    acceptLabel: "Убрать",
    rejectLabel: "Отмена",
    acceptClass: "p-button-danger",
    accept: async () => {
      await api.removeGroupSubject(gs.id);
      await loadAssignments();
      toast.add({ severity: "info", summary: "Удалено", life: 2000 });
    },
  });
}
</script>

<template>
  <h2>Учебный план</h2>

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

  <div v-if="selectedGroupId">
    <DataTable :value="groupSubjects" stripedRows>
      <Column header="Предмет">
        <template #body="{ data }">{{ data.subject_name }}</template>
      </Column>
      <Column header="Тип" style="width: 10rem">
        <template #body="{ data }">
          <Button
            :label="data.type === 'exam' ? 'Экзамен' : 'Зачёт'"
            :severity="data.type === 'exam' ? 'info' : 'success'"
            size="small"
            outlined
            @click="changeType(data)"
          />
        </template>
      </Column>
      <Column header="" style="width: 5rem">
        <template #body="{ data }">
          <Button icon="pi pi-trash" text rounded severity="danger" @click="confirmRemove(data)" />
        </template>
      </Column>
    </DataTable>

    <div class="add-row">
      <Select
        v-model="addSubjectId"
        :options="subjects"
        optionLabel="name"
        optionValue="id"
        placeholder="Выберите предмет"
        style="width: 250px"
      />
      <Select v-model="addType" :options="typeOptions" optionLabel="label" optionValue="value" style="width: 140px" />
      <Button label="Добавить" icon="pi pi-plus" @click="addSubject" :disabled="!addSubjectId" />
    </div>
  </div>
  <p v-else class="hint">Выберите группу для настройки учебного плана.</p>
</template>

<style scoped>
.toolbar {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
  align-items: center;
}
.add-row {
  display: flex;
  gap: 0.75rem;
  margin-top: 1rem;
  align-items: center;
}
.hint {
  margin-top: 2rem;
  color: #888;
}
</style>
