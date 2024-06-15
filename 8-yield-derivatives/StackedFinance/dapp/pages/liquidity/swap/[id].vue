<script lang="ts" setup>
const state = useState('liquidity', () => ({
  maturity: 0,
  deposit: 0,
  balance: 0,
  lsuAddress:'',
  XRDBalance:0,
}))

</script>

<template>
  <UContainer class="flex flex-col gap-y-6">
    <liquidity-swap></liquidity-swap>
  </UContainer>
</template>

<style scoped></style>
