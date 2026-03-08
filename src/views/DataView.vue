<script setup lang="ts">
import Button from "primevue/button";
import { useToast } from "primevue/usetoast";
import { save as saveDialog, open as openDialog } from "@tauri-apps/plugin-dialog";
import { api } from "../api";

const toast = useToast();

async function exportDatabase() {
  const path = await saveDialog({
    defaultPath: "scholarmetric.db",
    filters: [{ name: "SQLite Database", extensions: ["db"] }],
  });
  if (!path) return;
  try {
    await api.exportDb(path);
    toast.add({ severity: "success", summary: "Экспортировано", detail: path, life: 3000 });
  } catch (e: any) {
    toast.add({ severity: "error", summary: "Ошибка", detail: e, life: 4000 });
  }
}

async function importDatabase() {
  const path = await openDialog({
    filters: [{ name: "SQLite Database", extensions: ["db"] }],
    multiple: false,
  });
  if (!path) return;
  try {
    await api.importDb(path as string);
    toast.add({
      severity: "success",
      summary: "Импортировано",
      detail: "База данных импортирована. Перезапустите приложение для применения изменений.",
      life: 5000,
    });
  } catch (e: any) {
    toast.add({ severity: "error", summary: "Ошибка", detail: e, life: 4000 });
  }
}
</script>

<template>
  <h2>Импорт / Экспорт данных</h2>

  <div class="data-actions">
    <div class="action-card">
      <h3>Экспорт базы данных</h3>
      <p>Сохранить копию всей базы данных в файл .db</p>
      <Button label="Экспорт" icon="pi pi-upload" @click="exportDatabase" />
    </div>

    <div class="action-card">
      <h3>Импорт базы данных</h3>
      <p>Загрузить базу данных из файла .db. Текущие данные будут заменены.</p>
      <Button label="Импорт" icon="pi pi-download" severity="warn" @click="importDatabase" />
    </div>
  </div>
</template>

<style scoped>
.data-actions {
  display: flex;
  gap: 2rem;
  margin-top: 1rem;
}
.action-card {
  flex: 1;
  padding: 1.5rem;
  background: white;
  border-radius: 8px;
  border: 1px solid #e0e0e0;
}
.action-card h3 {
  margin-bottom: 0.5rem;
}
.action-card p {
  margin-bottom: 1rem;
  color: #666;
  font-size: 0.9rem;
}
</style>
