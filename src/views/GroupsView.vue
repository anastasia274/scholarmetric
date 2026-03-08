<script setup lang="ts">
import { ref, onMounted } from "vue";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Dialog from "primevue/dialog";
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";
import { api, type Group } from "../api";

const groups = ref<Group[]>([]);
const showDialog = ref(false);
const editingGroup = ref<Group | null>(null);
const formName = ref("");
const previewName = ref("");
const confirm = useConfirm();
const toast = useToast();

onMounted(loadGroups);

async function loadGroups() {
  groups.value = await api.getGroups();
}

function openNew() {
  editingGroup.value = null;
  formName.value = "";
  previewName.value = "";
  showDialog.value = true;
}

function openEdit(group: Group) {
  editingGroup.value = group;
  formName.value = group.name;
  previewName.value = group.name;
  showDialog.value = true;
}

async function onInput() {
  if (formName.value.trim()) {
    previewName.value = await api.previewGroupName(formName.value);
  } else {
    previewName.value = "";
  }
}

async function save() {
  if (!formName.value.trim()) return;
  try {
    if (editingGroup.value) {
      await api.updateGroup(editingGroup.value.id, formName.value);
    } else {
      await api.createGroup(formName.value);
    }
    showDialog.value = false;
    await loadGroups();
    toast.add({ severity: "success", summary: "Сохранено", life: 2000 });
  } catch (e: any) {
    toast.add({ severity: "error", summary: "Ошибка", detail: e, life: 4000 });
  }
}

function confirmDelete(group: Group) {
  confirm.require({
    message: `Удалить группу "${group.name}"? Все студенты и оценки группы будут удалены.`,
    header: "Подтверждение",
    acceptLabel: "Удалить",
    rejectLabel: "Отмена",
    acceptClass: "p-button-danger",
    accept: async () => {
      await api.deleteGroup(group.id);
      await loadGroups();
      toast.add({ severity: "info", summary: "Удалено", life: 2000 });
    },
  });
}
</script>

<template>
  <h2>Группы</h2>
  <Button label="Добавить группу" icon="pi pi-plus" @click="openNew" class="mb-3" />

  <DataTable :value="groups" stripedRows>
    <Column field="name" header="Название" sortable />
    <Column header="Действия" style="width: 10rem">
      <template #body="{ data }">
        <Button icon="pi pi-pencil" text rounded @click="openEdit(data)" />
        <Button icon="pi pi-trash" text rounded severity="danger" @click="confirmDelete(data)" />
      </template>
    </Column>
  </DataTable>

  <Dialog v-model:visible="showDialog" :header="editingGroup ? 'Редактировать группу' : 'Новая группа'" modal style="width: 400px">
    <div class="flex flex-column gap-3">
      <label>Название группы</label>
      <InputText v-model="formName" @input="onInput" placeholder="Например: П 601А" autofocus />
      <small v-if="previewName" class="text-color-secondary">
        Будет сохранено как: <strong>{{ previewName }}</strong>
      </small>
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
