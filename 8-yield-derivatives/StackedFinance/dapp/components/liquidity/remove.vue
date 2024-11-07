<script lang="ts" setup>
const state = useState('liquidity');
const route = useRoute()
const component_id = route.params.id;
const { $getWallet, $getRDT, $getEntityDetails } = await useNuxtApp();
const walletAddress = (await $getWallet()).accounts[0].address;
const depositAmount = ref(50);

// Get the pool component from the amm component
const entityData = await $getEntityDetails(component_id);
const poolComponentAddress = useFindByFieldName(entityData.details.state.fields, 'pool_component').findByFieldName;

//Get the state details from the pool component
const poolEntityData = await $getEntityDetails(poolComponentAddress);
const poolUnitAddress = poolEntityData.details.state.pool_unit_resource_address;

async function deposit() {
  const manifest = `
  CALL_METHOD
      Address("${walletAddress}")
      "withdraw"
      Address("${poolUnitAddress}")
      Decimal("${depositAmount.value}")
  ;
  TAKE_ALL_FROM_WORKTOP
      Address("${poolUnitAddress}")
      Bucket("pool_unit")
  ;
  CALL_METHOD
      Address("${component_id}")
      "remove_liquidity"
      Bucket("pool_unit")
  ;
  CALL_METHOD
      Address("${walletAddress}")
      "deposit_batch"
      Expression("ENTIRE_WORKTOP")
  ;
  `
  const rdt = await $getRDT();

  rdt.walletApi.sendTransaction({
    transactionManifest: manifest,
  });
}
</script>

<template>
  <div>
    <UCard>
      <template #header>
        Liquidity to Remove
      </template>
    
      <div>Balance: {{ state.balance }}</div>
      <UInput v-model="depositAmount" />

      <template #footer>
        <UButton @click="deposit()" block>Remove {{ depositAmount }}</UButton>
      </template>
    </UCard>
  </div>
</template>

<style scoped></style>
