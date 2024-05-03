<script lang="ts" setup>
const route = useRoute()
const component_id = route.params.id;
const { $getWallet, $getRDT,$getEntityDetails } = await useNuxtApp();
const walletAddress = (await $getWallet()).accounts[0].address;
const state = useState('counter');
const deposit = ref(50);
const rdt = await $getRDT();

async function stake() {
  //Get the entitity data stored in the component
  const entityData = await $getEntityDetails(component_id);
  //Get the validator address for assembling the manifest
  const validatorAddress = useFindByFieldName(entityData.details.state.fields, 'lsu_validator_component').findByFieldName;
   //Get the LSU Address for assembling the manifest
  const lsuAddress = useFindByFieldName(entityData.details.state.fields, 'lsu_address').findByFieldName;
  const manifest = `
  CALL_METHOD
      Address("${walletAddress}")
      "withdraw"
      Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
      Decimal("${deposit.value}")
  ;
  TAKE_ALL_FROM_WORKTOP
      Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
      Bucket("bucket1")
  ;
  CALL_METHOD
      Address("${validatorAddress}")
      "stake"
      Bucket("bucket1")
  ;
  TAKE_ALL_FROM_WORKTOP
      Address("${lsuAddress}")
      Bucket("bucket2")
  ;
  CALL_METHOD
      Address("${walletAddress}")
      "deposit"
      Bucket("bucket2")
  ;
  `;

  rdt.walletApi.sendTransaction({
      transactionManifest: manifest,
    });
}

async function faucet(){
  const manifest = `
  CALL_METHOD
      Address("component_tdx_2_1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxyulkzl")
      "free"
  ;
  CALL_METHOD
      Address("account_tdx_2_128fpdfl9qzha85luverks8plcdwqayhvhgw8mlsr2tr720dj3exm0h")
      "try_deposit_batch_or_abort"
      Expression("ENTIRE_WORKTOP")
      Enum<0u8>()
  ;
  `

  rdt.walletApi.sendTransaction({
      transactionManifest: manifest,
    });
}

//const selected = ref('sms');
</script>

<template>
  <div>
    <UCard>
    <template #header>
        Step 1: Stake to create LSU
        <UButton @click="faucet()" block>Faucet 10000</UButton>
    </template>
      
    <div>XRD Balance: {{ state.XRDBalance }}</div>
    <UInput v-model="deposit" />
    <template #footer>
      <UButton @click="stake()" block>Stake {{ deposit.value }}</UButton>
    </template>
  </UCard>
  </div>
</template>

<style scoped></style>
