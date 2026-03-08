<script setup lang="ts">
import { ref, onMounted } from "vue";
import Menubar from "primevue/menubar";
import Toast from "primevue/toast";
import ConfirmDialog from "primevue/confirmdialog";
import Button from "primevue/button";
import { useRouter } from "vue-router";

const router = useRouter();

const isDark = ref(false);

onMounted(() => {
  const saved = localStorage.getItem("theme");
  if (saved === "dark" || (!saved && window.matchMedia("(prefers-color-scheme: dark)").matches)) {
    isDark.value = true;
    document.documentElement.classList.add("dark");
  }
});

function toggleTheme() {
  isDark.value = !isDark.value;
  document.documentElement.classList.toggle("dark", isDark.value);
  localStorage.setItem("theme", isDark.value ? "dark" : "light");
}

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
      <template #end>
        <Button
          :icon="isDark ? 'pi pi-sun' : 'pi pi-moon'"
          text
          rounded
          @click="toggleTheme"
          v-tooltip.bottom="isDark ? 'Светлая тема' : 'Тёмная тема'"
        />
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
  background: var(--app-bg);
  color: var(--app-text);
  transition: background 0.2s, color 0.2s;
}

:root {
  --app-bg: #f8f9fa;
  --app-text: #1e1e1e;
  --app-text-secondary: #333;
  --app-border: #ddd;
  --app-card-bg: #ffffff;
  --app-hover: #f5f5f5;
  --app-cell-excellent: #e8f5e9;
  --app-cell-good: #e3f2fd;
  --app-cell-satisfactory: #fff3e0;
  --app-cell-failing: #ffebee;
  --app-cell-empty-bg: #fafafa;
  --app-cell-empty-text: #bbb;
  --app-stats-bg: #f9f9f9;
  --app-menu-bg: #ffffff;
  --app-menu-border: #ccc;
  --app-option-bg: #f5f5f5;
  --app-option-hover: #e0e0e0;
}

.dark {
  --app-bg: #1a1a2e;
  --app-text: #e0e0e0;
  --app-text-secondary: #cccccc;
  --app-border: #3a3a5c;
  --app-card-bg: #22223b;
  --app-hover: #2a2a4a;
  --app-cell-excellent: #1b3d2a;
  --app-cell-good: #1a2d4a;
  --app-cell-satisfactory: #3d3020;
  --app-cell-failing: #3d1a1a;
  --app-cell-empty-bg: #1e1e30;
  --app-cell-empty-text: #555;
  --app-stats-bg: #22223b;
  --app-menu-bg: #2a2a4a;
  --app-menu-border: #444466;
  --app-option-bg: #333355;
  --app-option-hover: #444477;
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
  color: var(--app-text-secondary);
}
</style>
