<script lang="ts" setup>
import liquidity from '../../data/liquidity.json'
</script>

<template>
  <div class="flex flex-row gap-x-4">
    <card-default v-for="item in liquidity" 
    :name="item.name" 
    :validatorAddress="item.validatorAddress"
    :componentAddress="item.yieldComponentAddress"
    :maturities="item.maturities"
    :key="item.validatorAddress"> 
    </card-default>
  </div>
</template>

<style scoped></style>
