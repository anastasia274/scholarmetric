<script setup lang="ts">
import { ref, watch } from "vue";
import InputNumber from "primevue/inputnumber";
import Button from "primevue/button";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Tabs from "primevue/tabs";
import TabList from "primevue/tablist";
import Tab from "primevue/tab";
import TabPanels from "primevue/tabpanels";
import TabPanel from "primevue/tabpanel";
import { useToast } from "primevue/usetoast";
import { save as saveDialog } from "@tauri-apps/plugin-dialog";
import { api, type OverallRanking, type RankingEntry } from "../api";

const semester = ref(1);
const ranking = ref<OverallRanking | null>(null);
const activeTab = ref("excellent");
const toast = useToast();

watch(semester, loadRanking);

async function loadRanking() {
  try {
    ranking.value = await api.getOverallRanking(semester.value);
  } catch (e: any) {
    toast.add({ severity: "error", summary: "Ошибка", detail: e, life: 4000 });
  }
}

function fio(e: RankingEntry): string {
  const parts = [e.last_name, e.first_name];
  if (e.patronymic) parts.push(e.patronymic);
  return parts.join(" ");
}

async function exportXlsx() {
  const path = await saveDialog({
    defaultPath: `рейтинг_сем${semester.value}.xlsx`,
    filters: [{ name: "Excel", extensions: ["xlsx"] }],
  });
  if (!path) return;
  try {
    await api.exportRankingXlsx(semester.value, path);
    toast.add({ severity: "success", summary: "Экспортировано", detail: path, life: 3000 });
  } catch (e: any) {
    toast.add({ severity: "error", summary: "Ошибка", detail: e, life: 4000 });
  }
}

loadRanking();

const tabs = [
  { key: "excellent", label: "Отличники" },
  { key: "good", label: "Хорошисты" },
  { key: "satisfactory", label: "Троечники" },
  { key: "failing", label: "Неуспевающие" },
  { key: "not_attested", label: "Не аттестованы" },
];

function getEntries(key: string): RankingEntry[] {
  if (!ranking.value) return [];
  return (ranking.value as any)[key] ?? [];
}
</script>

<template>
  <h2>Общий рейтинг</h2>

  <div class="toolbar">
    <label>Семестр:</label>
    <InputNumber v-model="semester" :min="1" :max="12" showButtons style="width: 120px" />
    <Button
      v-if="ranking"
      label="Экспорт в Excel"
      icon="pi pi-file-excel"
      severity="success"
      outlined
      @click="exportXlsx"
    />
  </div>

  <div v-if="ranking">
    <Tabs v-model:value="activeTab">
      <TabList>
        <Tab v-for="tab in tabs" :key="tab.key" :value="tab.key">
          {{ tab.label }} ({{ getEntries(tab.key).length }})
        </Tab>
      </TabList>
      <TabPanels>
        <TabPanel v-for="tab in tabs" :key="tab.key" :value="tab.key">
          <DataTable :value="getEntries(tab.key)" stripedRows>
            <Column header="№">
              <template #body="{ index }">{{ index + 1 }}</template>
            </Column>
            <Column header="ФИО">
              <template #body="{ data }">{{ fio(data) }}</template>
            </Column>
            <Column field="group_name" header="Группа" />
          </DataTable>
        </TabPanel>
      </TabPanels>
    </Tabs>
  </div>
</template>

<style scoped>
.toolbar {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
  align-items: center;
}
</style>
