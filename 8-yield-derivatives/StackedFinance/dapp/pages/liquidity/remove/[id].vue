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
    <liquidity-remove></liquidity-remove>
  </UContainer>
</template>

<style scoped></style>
