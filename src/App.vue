<script setup lang="ts">
import Menubar from "primevue/menubar";
import Toast from "primevue/toast";
import ConfirmDialog from "primevue/confirmdialog";
import { useRouter } from "vue-router";

const router = useRouter();

const menuItems = [
  {
    label: "Справочники",
    icon: "pi pi-database",
    items: [
      { label: "Группы", icon: "pi pi-users", command: () => router.push("/groups") },
      { label: "Предметы", icon: "pi pi-book", command: () => router.push("/subjects") },
      { label: "Студенты", icon: "pi pi-user", command: () => router.push("/students") },
    ],
  },
  {
    label: "Учебный план",
    icon: "pi pi-calendar",
    command: () => router.push("/assign"),
  },
  {
    label: "Оценки",
    icon: "pi pi-pencil",
    command: () => router.push("/grades"),
  },
  {
    label: "Отчёты",
    icon: "pi pi-chart-bar",
    items: [
      { label: "По группе", icon: "pi pi-list", command: () => router.push("/report-group") },
      { label: "Общий рейтинг", icon: "pi pi-sort-alpha-down", command: () => router.push("/report-ranking") },
    ],
  },
  {
    label: "Данные",
    icon: "pi pi-download",
    command: () => router.push("/data"),
  },
];
</script>

<template>
  <Toast />
  <ConfirmDialog />
  <div class="app-layout">
    <Menubar :model="menuItems" class="app-menubar">
      <template #start>
        <span class="app-title">ScholarMetric</span>
      </template>
    </Menubar>
    <div class="app-content">
      <router-view />
    </div>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: Inter, system-ui, -apple-system, sans-serif;
  background: #f8f9fa;
  color: #1e1e1e;
}

.app-layout {
  min-height: 100vh;
}

.app-menubar {
  border-radius: 0 !important;
}

.app-title {
  font-weight: 700;
  font-size: 1.1rem;
  margin-right: 1.5rem;
  color: var(--p-primary-color);
}

.app-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 1.5rem;
}

h2 {
  margin-bottom: 1rem;
  color: #333;
}
</style>
