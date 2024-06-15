<script lang="ts" setup>
const state = useState('counter');
const { $getWallet,$getRDT } = await useNuxtApp();
const walletAddress = (await $getWallet()).accounts[0].address;
const route = useRoute();
const componentId = route.params.id;

async function deposit() {
  //Check for min deposit of 1 satoshi = 0.00000001
  const manifest = `
  CALL_METHOD
    Address("${walletAddress}")
    "withdraw"
    Address("${state.value.lsuAddress}")
    Decimal("${state.value.deposit}")
    ;
    TAKE_ALL_FROM_WORKTOP
        Address("${state.value.lsuAddress}")
        Bucket("LSU Bucket")
    ;
    CALL_METHOD
        Address("${componentId}")
        "tokenize_yield"
        Bucket("LSU Bucket")
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
   <UCard>
    <template #header>
      Step 3: Deposit to Earn
    </template>
  
    <div>Balance: {{ state.balance }}</div>
    <UInput v-model="state.deposit" />

    <template #footer>
      <UButton @click="deposit()" block>Deposit {{ state.deposit }}</UButton>
    </template>
  </UCard>
</template>

<style scoped></style>
