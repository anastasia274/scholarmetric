import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/groups",
    },
    {
      path: "/groups",
      component: () => import("./views/GroupsView.vue"),
    },
    {
      path: "/subjects",
      component: () => import("./views/SubjectsView.vue"),
    },
    {
      path: "/students",
      component: () => import("./views/StudentsView.vue"),
    },
    {
      path: "/assign",
      component: () => import("./views/AssignView.vue"),
    },
    {
      path: "/grades",
      component: () => import("./views/GradesView.vue"),
    },
    {
      path: "/report-group",
      component: () => import("./views/GroupReportView.vue"),
    },
    {
      path: "/report-ranking",
      component: () => import("./views/RankingView.vue"),
    },
    {
      path: "/data",
      component: () => import("./views/DataView.vue"),
    },
  ],
});

export default router;
