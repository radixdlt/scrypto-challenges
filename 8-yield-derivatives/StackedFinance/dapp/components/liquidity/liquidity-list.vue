<script lang="ts" setup>
import liquidity from '../../data/liquidity.json'
</script>

<template>
  <div class="flex flex-row gap-x-4">
    <card-liquidity v-for="item in liquidity" 
    :yieldPackageAddress="item.yieldPackageAddress"
    :yieldComponentAddress="item.yieldComponentAddress"
    :ammPackageAddress="item.ammPackageAddress"
    :ammComponentAddress="item.ammComponentAddress"
    :validatorAddress="item.validatorAddress"
    :key="item.validatorAddress"> 
    </card-liquidity>
  </div>
</template>

<style scoped></style>
