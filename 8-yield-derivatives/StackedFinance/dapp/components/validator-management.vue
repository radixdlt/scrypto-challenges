<script lang="ts" setup>
import liquidity from '../data/liquidity.json'
</script>

<template>
  <div class="flex flex-row gap-x-4">
    <card-management v-for="item in liquidity" 
    :yieldPackageAddress="item.yieldPackageAddress"
    :yieldComponentAddress="item.yieldComponentAddress"
    :ammPackageAddress="item.ammPackageAddress"
    :ammComponentAddress="item.ammComponentAddress"
    :validatorAddress="item.validatorAddress"
    :key="item.validatorAddress"> 
    </card-management>
  </div>
</template>

<style scoped></style>
