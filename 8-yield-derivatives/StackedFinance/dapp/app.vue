import index from './pages/index.vue'; import admin from './pages/admin.vue';
<script lang="ts" setup>
const state = useState('global', () => ({
  maturity: 0,
  deposit: 0,
  balance: 0,
  lsuAddress:'',
  XRDBalance:0,
}))

</script>

<template>
  <div class="flex flex-col min-h-screen bg-slate-700">
    <app-header ></app-header>
    <NuxtPage></NuxtPage>
    <app-footer class="mt-auto p-4"></app-footer>
    <UNotifications />
  </div>
</template>
