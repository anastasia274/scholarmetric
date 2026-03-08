<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Select from "primevue/select";
import Dialog from "primevue/dialog";
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";
import { api, type Student, type Group } from "../api";

const students = ref<Student[]>([]);
const groups = ref<Group[]>([]);
const filterGroupId = ref<number | null>(null);
const showDialog = ref(false);
const editingStudent = ref<Student | null>(null);
const formLastName = ref("");
const formFirstName = ref("");
const formPatronymic = ref("");
const formGroupId = ref<number | null>(null);
const confirm = useConfirm();
const toast = useToast();

const groupMap = computed(() => {
  const m: Record<number, string> = {};
  for (const g of groups.value) m[g.id] = g.name;
  return m;
});

onMounted(async () => {
  groups.value = await api.getGroups();
  await loadStudents();
});

async function loadStudents() {
  students.value = await api.getStudents(filterGroupId.value ?? undefined);
}

function openNew() {
  editingStudent.value = null;
  formLastName.value = "";
  formFirstName.value = "";
  formPatronymic.value = "";
  formGroupId.value = groups.value.length ? groups.value[0].id : null;
  showDialog.value = true;
}

function openEdit(s: Student) {
  editingStudent.value = s;
  formLastName.value = s.last_name;
  formFirstName.value = s.first_name;
  formPatronymic.value = s.patronymic ?? "";
  formGroupId.value = s.group_id;
  showDialog.value = true;
}

async function save() {
  if (!formLastName.value.trim() || !formFirstName.value.trim() || !formGroupId.value) return;
  const pat = formPatronymic.value.trim() || null;
  try {
    if (editingStudent.value) {
      await api.updateStudent(
        editingStudent.value.id,
        formLastName.value,
        formFirstName.value,
        pat,
        formGroupId.value
      );
    } else {
      await api.createStudent(
        formLastName.value,
        formFirstName.value,
        pat,
        formGroupId.value
      );
    }
    showDialog.value = false;
    await loadStudents();
    toast.add({ severity: "success", summary: "Сохранено", life: 2000 });
  } catch (e: any) {
    toast.add({ severity: "error", summary: "Ошибка", detail: e, life: 4000 });
  }
}

function confirmDelete(s: Student) {
  confirm.require({
    message: `Удалить студента "${s.last_name} ${s.first_name}"?`,
    header: "Подтверждение",
    acceptLabel: "Удалить",
    rejectLabel: "Отмена",
    acceptClass: "p-button-danger",
    accept: async () => {
      await api.deleteStudent(s.id);
      await loadStudents();
      toast.add({ severity: "info", summary: "Удалено", life: 2000 });
    },
  });
}
</script>

<template>
  <h2>Студенты</h2>

  <div class="toolbar">
    <Select
      v-model="filterGroupId"
      :options="[{ id: null, name: 'Все группы' }, ...groups]"
      optionLabel="name"
      optionValue="id"
      placeholder="Фильтр по группе"
      @change="loadStudents"
      style="width: 200px"
    />
    <Button label="Добавить студента" icon="pi pi-plus" @click="openNew" />
  </div>

  <DataTable :value="students" stripedRows>
    <Column field="last_name" header="Фамилия" sortable />
    <Column field="first_name" header="Имя" sortable />
    <Column field="patronymic" header="Отчество" />
    <Column header="Группа" sortable>
      <template #body="{ data }">{{ groupMap[data.group_id] || '—' }}</template>
    </Column>
    <Column header="Действия" style="width: 10rem">
      <template #body="{ data }">
        <Button icon="pi pi-pencil" text rounded @click="openEdit(data)" />
        <Button icon="pi pi-trash" text rounded severity="danger" @click="confirmDelete(data)" />
      </template>
    </Column>
  </DataTable>

  <Dialog v-model:visible="showDialog" :header="editingStudent ? 'Редактировать студента' : 'Новый студент'" modal style="width: 450px">
    <div class="form-grid">
      <label>Фамилия *</label>
      <InputText v-model="formLastName" autofocus />
      <label>Имя *</label>
      <InputText v-model="formFirstName" />
      <label>Отчество</label>
      <InputText v-model="formPatronymic" />
      <label>Группа *</label>
      <Select
        v-model="formGroupId"
        :options="groups"
        optionLabel="name"
        optionValue="id"
        placeholder="Выберите группу"
      />
    </div>
    <template #footer>
      <Button label="Отмена" text @click="showDialog = false" />
      <Button label="Сохранить" icon="pi pi-check" @click="save" />
    </template>
  </Dialog>
</template>

<style scoped>
.toolbar {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
  align-items: center;
}
.form-grid {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 0.75rem;
  align-items: center;
}
</style>
