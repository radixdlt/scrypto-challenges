<script lang="ts" setup>
const links = [
{
  label: 'Home',
  icon: 'i-heroicons-home',
  to: '/'
}, {
  label: 'Fixed Income',
  icon: 'i-heroicons-circle-stack',
  to: '/fixed-income'
}, {
  label: 'Liquidity',
  icon: 'i-heroicons-chart-bar',
  to: '/liquidity'
}, {
  label: 'Admin Tools',
  icon: 'i-heroicons-command-line',
  to: '/admin'
}]
</script>

<template>
  <div class="flex p-4">
    <UHorizontalNavigation :links="links" />
    <ClientOnly>
      <radix-connect-button />
    </ClientOnly>
  </div>
</template>

<style scoped></style>
