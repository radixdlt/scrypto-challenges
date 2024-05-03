<script lang="ts" setup>
const { $getRDT, $getWallet, $getEntityDetails } = useNuxtApp();
const walletAddress = (await $getWallet()).accounts[0].address;
const props = defineProps([
  'validatorAddress',
  'yieldPackageAddress',
  'yieldComponentAddress',
  'ammPackageAddress',
  'ammComponentAddress'
]);
const rdt = await $getRDT();

const entityData = await $getEntityDetails(props.yieldComponentAddress);

const lsuVaultAddress = useFindByFieldName(entityData.details.state.fields, 'lsu_vault').findByFieldName;
const lsuValidatorAddress = useFindByFieldName(entityData.details.state.fields, 'lsu_validator_component').findByFieldName;
const lsuVaultDetails = await $getEntityDetails(lsuVaultAddress);

//Getting the entitiy details so that we can get the validator name
const validatorDetails = await $getEntityDetails(lsuValidatorAddress);
//This parses the meta data to try and get the validator name
const validatorName = useFindbyMetaData(validatorDetails.metadata, 'name').typed.value;

</script>



<template>
  <UCard class="flex flex-col">
    <template #header>
      {{ validatorName }}
    </template>

    <p>Up to 50% p.a</p>
    <p>$10,000,000 TVL</p>

    <template #footer>
      <div class="flex flex-col gap-y-4">
        <UButtonGroup class="gap-x-2 "orientation="horizontal">
          <UButton :to="`/liquidity/add/${ammComponentAddress}`" target="_self">Add Liquidity</UButton>
          <UButton :to="`/liquidity/remove/${ammComponentAddress}`" target="_self">Remove Liquidity</UButton>
        </UButtonGroup>
        <UButton :to="`/liquidity/swap/${ammComponentAddress}`" target="_self" block>Swap</UButton>
      </div>
      
    </template>
  </UCard>
</template>

<style scoped></style>
