<script lang="ts" setup>
const state = useState('liquidity');
const route = useRoute()
const component_id = route.params.id;
const { $getWallet, $getRDT, $getEntityDetails } = await useNuxtApp();
const walletAddress = (await $getWallet()).accounts[0].address;
const depositAmount = ref(50);

//Get the AMM Yield State data
const entityData = await $getEntityDetails(component_id);
//Get the lsu address to add liquidity to the amm
const lsuAddress = useFindByFieldName(entityData.details.state.fields, 'lsu_address').findByFieldName;
//Get the tokenizer component
const tokenizerComponent = useFindByFieldName(entityData.details.state.fields, 'tokenizer_component_address').findByFieldName;
//Get the tokenizer state data
const tokenizerData = await $getEntityDetails(tokenizerComponent);
//Get the PT token required to add liquidity
const ptResourceAddress = useFindByFieldName(tokenizerData.details.state.fields, 'pt_rm').findByFieldName;

async function deposit() {
  const manifest = `
  CALL_METHOD
      Address("${walletAddress}")
      "withdraw"
      Address("${ptResourceAddress}")
      Decimal("${depositAmount.value}")
  ;
  CALL_METHOD
      Address("${walletAddress}")
      "withdraw"
      Address("${lsuAddress}")
      Decimal("${depositAmount.value}")
  ;
  TAKE_ALL_FROM_WORKTOP
      Address("${ptResourceAddress}")
      Bucket("pt_resource")
  ;
  TAKE_ALL_FROM_WORKTOP
      Address("${lsuAddress}")
      Bucket("lsu_resource_address")
  ;
  CALL_METHOD
      Address("${component_id}")
      "add_liquidity"
      Bucket("pt_resource")
      Bucket("lsu_resource_address")
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
        Liquidity to Add
      </template>
    
      <div>Balance: {{ state.balance }}</div>
      <UInput v-model="depositAmount" />

      <template #footer>
        <UButton @click="deposit()" block>Deposit {{ depositAmount }}</UButton>
      </template>
    </UCard>
  </div>
</template>

<style scoped></style>
