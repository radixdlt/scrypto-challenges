<script lang="ts" setup>
const { $getRDT, $getWallet, $getEntityDetails } = useNuxtApp();
const toast = useToast();
try{
  const walletAddress = (await $getWallet()).accounts[0].address;
}catch(error){
  toast.add({ 
    title: 'Please Connect your wallet this site to function', 
    color : 'red',
    timeout: 0 });
}
const props = defineProps([
  'validatorAddress',
  'yieldPackageAddress',
  'yieldComponentAddress',
  'ammPackageAddress',
  'ammComponentAddress'
]);
const rdt = await $getRDT();

const impliedRate = ref(0);

const entityData = await $getEntityDetails(props.yieldComponentAddress);

const lsuVaultAddress = useFindByFieldName(entityData.details.state.fields, 'lsu_vault').findByFieldName;
const lsuValidatorAddress = useFindByFieldName(entityData.details.state.fields, 'lsu_validator_component').findByFieldName;
const lsuVaultDetails = await $getEntityDetails(lsuVaultAddress);

//Getting the entitiy details so that we can get the validator name
const validatorDetails = await $getEntityDetails(lsuValidatorAddress);
//This parses the meta data to try and get the validator name
const validatorName = useFindbyMetaData(validatorDetails.metadata, 'name').typed.value;

async function createAMM() {
  //Ideally you check if the user has the admin badge
  const manifest = `
  CALL_FUNCTION
      Address("${props.ammPackageAddress}")
      "YieldAMM"
      "instantiate_yield_amm"
      Enum<OwnerRole::None>()
      Decimal("50")
      Decimal("1.01")
      Decimal("0.8")
      Address("${props.yieldComponentAddress}")
  ;
  `;

  rdt.walletApi.sendTransaction({
    transactionManifest: manifest,
  });
}

async function setImplied(){
  const manifest = `
  CALL_METHOD
    Address("${props.ammComponentAddress}")
    "set_initial_ln_implied_rate"
    PreciseDecimal("${impliedRate.value}")
  ;
  `;

  rdt.walletApi.sendTransaction({
    transactionManifest: manifest,
  });

}

</script>



<template>
  <UCard class="flex flex-col">
    <template #header>
      {{ validatorName }}
    </template>

    <p>Expiry: todo</p>

    <template #footer>
      <div class="flex flex-col gap-y-4">
        <UButton @click="createAMM()" block>Create AMM</UButton>
        <UButtonGroup class="gap-x-2 "orientation="horizontal">
          <UInput v-model="impliedRate"></UInput>
          <UButton @click="setImplied()">Set Implied</UButton>
        </UButtonGroup>
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
