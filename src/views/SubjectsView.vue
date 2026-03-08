<script setup lang="ts">
import { ref, onMounted } from "vue";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Dialog from "primevue/dialog";
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";
import { api, type Subject } from "../api";

const subjects = ref<Subject[]>([]);
const showDialog = ref(false);
const editingSubject = ref<Subject | null>(null);
const formName = ref("");
const confirm = useConfirm();
const toast = useToast();

onMounted(loadSubjects);

async function loadSubjects() {
  subjects.value = await api.getSubjects();
}

function openNew() {
  editingSubject.value = null;
  formName.value = "";
  showDialog.value = true;
}

function openEdit(subject: Subject) {
  editingSubject.value = subject;
  formName.value = subject.name;
  showDialog.value = true;
}

async function save() {
  if (!formName.value.trim()) return;
  try {
    if (editingSubject.value) {
      await api.updateSubject(editingSubject.value.id, formName.value);
    } else {
      await api.createSubject(formName.value);
    }
    showDialog.value = false;
    await loadSubjects();
    toast.add({ severity: "success", summary: "Сохранено", life: 2000 });
  } catch (e: any) {
    toast.add({ severity: "error", summary: "Ошибка", detail: e, life: 4000 });
  }
}

function confirmDelete(subject: Subject) {
  confirm.require({
    message: `Удалить предмет "${subject.name}"? Все связанные оценки будут удалены.`,
    header: "Подтверждение",
    acceptLabel: "Удалить",
    rejectLabel: "Отмена",
    acceptClass: "p-button-danger",
    accept: async () => {
      await api.deleteSubject(subject.id);
      await loadSubjects();
      toast.add({ severity: "info", summary: "Удалено", life: 2000 });
    },
  });
}
</script>

<template>
  <h2>Предметы</h2>
  <Button label="Добавить предмет" icon="pi pi-plus" @click="openNew" class="mb-3" />

  <DataTable :value="subjects" stripedRows>
    <Column field="name" header="Название" sortable />
    <Column header="Действия" style="width: 10rem">
      <template #body="{ data }">
        <Button icon="pi pi-pencil" text rounded @click="openEdit(data)" />
        <Button icon="pi pi-trash" text rounded severity="danger" @click="confirmDelete(data)" />
      </template>
    </Column>
  </DataTable>

  <Dialog v-model:visible="showDialog" :header="editingSubject ? 'Редактировать предмет' : 'Новый предмет'" modal style="width: 400px">
    <div class="flex flex-column gap-3">
      <label>Название предмета</label>
      <InputText v-model="formName" placeholder="Например: Математика" autofocus />
    </div>
    <template #footer>
      <Button label="Отмена" text @click="showDialog = false" />
      <Button label="Сохранить" icon="pi pi-check" @click="save" />
    </template>
  </Dialog>
</template>

<style scoped>
.mb-3 { margin-bottom: 1rem; }
.flex { display: flex; }
.flex-column { flex-direction: column; }
.gap-3 { gap: 0.75rem; }
</style>
