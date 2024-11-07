<script lang="ts" setup>
const state = useState('liquidity');
const route = useRoute()
const component_id = route.params.id;
const { $getWallet, $getRDT, $getEntityDetails } = await useNuxtApp();
const walletAddress = (await $getWallet()).accounts[0].address;
const assetTop = ref(0);
const assetBottom = ref(0);
const swapTop = ref("pt");
const swapBottom = ref("lsu");
const rdt = await $getRDT();
const toast = useToast()


enum SWAPS {
  TwelveMonths = 0,
  EighteenMonths = 1,
  TwentyFourMonths = 2,
}

const entityData = await $getEntityDetails(component_id);
console.log(entityData)

const lsuAddress = useFindByFieldName(entityData.details.state.fields, 'lsu_address').findByFieldName;


const tokenizerComponent = useFindByFieldName(entityData.details.state.fields, 'tokenizer_component_address').findByFieldName;
//Get the tokenizer state data
const tokenizerData = await $getEntityDetails(tokenizerComponent);
//Get the PT token required to add liquidity
const ptResourceAddress = useFindByFieldName(tokenizerData.details.state.fields, 'pt_rm').findByFieldName;

const ytResourceAddress = useFindByFieldName(tokenizerData.details.state.fields, 'yt_rm').findByFieldName;

const assetAddress = "";
const poolUnitAddress = "resource_tdx_2_1tk6xnhan7rr77y3ag6y8ap9svzka8kklyfs2cmz977uw5ngvcdw526";

const itemsTop = [
  [{
    label: 'PT',
    icon: 'i-heroicons-archive-box-20-solid',
    click: () => {
      switchTop('pt')
    }
  }, {
    label: 'YT',
    icon: 'i-heroicons-arrow-right-circle-20-solid',
    click: () => {
      switchTop('yt')
    }
  }, {
    label: 'LSU',
    icon: 'i-heroicons-trash-20-solid',
    click: () => {
      switchTop('lsu')
    }
  }]
]

const itemsBottom = [
  [{
    label: 'PT',
    icon: 'i-heroicons-archive-box-20-solid',
    click: () => {
      switchBottom('pt')
    }
  }, {
    label: 'YT',
    icon: 'i-heroicons-arrow-right-circle-20-solid',
    click: () => {
      switchBottom('yt')
    }
  }, {
    label: 'LSU',
    icon: 'i-heroicons-trash-20-solid',
    click: () => {
      switchBottom('lsu')
    }
  }]
]

async function switchTop(choice){
  swapTop.value=choice;
}

async function switchBottom(choice){
  swapBottom.value=choice;
}

async function swap() {
  const choice=swapTop.value+swapBottom.value;
  let manifest = null;
  console.log(choice);
  switch(choice){
    case 'ptlsu': 
      manifest =swapPTforLSU(5);
    break;
    case 'lsupt': 
      manifest =swapLSUforPT(5);
    break;
    case 'ytlsu': 
      toast.add({ title: 'High Slippage Expected, please reduce guarantees.' })
      manifest =swapYTforLSU(1);
    break;
    case 'lsuyt': 
      manifest =swapLSUforYT(5);
    break;
    default: 
      toast.add({ title: 'Unsupported Swap Pair' })
    return;
  }
  //const manifest = swapPTforLSU(5);
  //const manifest = swapLSUforPT(5);
  //const manifest = swapYTforLSU(1);
  //const manifest = swapLSUforYT(5);
  /*+`
  CALL_METHOD
    Address("${walletAddress}")
    "withdraw"
    Address("${ptResourceAddress}")
    Decimal("10")
  ;
  TAKE_ALL_FROM_WORKTOP
      Address("${ptResourceAddress}")
      Bucket("pt_resource")
  ;
  CALL_METHOD
      Address("${component_id}")
      "swap_exact_${swapTop.value}_for_${swapBottom.value}"
      Bucket("pt_resource")
  ;
  CALL_METHOD
      Address("${walletAddress}")
      "deposit_batch"
      Expression("ENTIRE_WORKTOP")
  ;

  `;
*/
console.log(manifest);
  rdt.walletApi.sendTransaction({
    transactionManifest: manifest,
  });
}

function swapPTforLSU(amount){
  const manifest = `
  CALL_METHOD
    Address("${walletAddress}")
    "withdraw"
    Address("${ptResourceAddress}")
    Decimal("${amount}")
  ;
  TAKE_ALL_FROM_WORKTOP
      Address("${ptResourceAddress}")
      Bucket("pt_resource")
  ;
  CALL_METHOD
      Address("${component_id}")
      "swap_exact_pt_for_lsu"
      Bucket("pt_resource")
  ;
  CALL_METHOD
      Address("${walletAddress}")
      "deposit_batch"
      Expression("ENTIRE_WORKTOP")
  ;
  `;
  rdt.walletApi.sendTransaction({
    transactionManifest: manifest,
  });
}

function swapLSUforPT(amount){
  return `
  CALL_METHOD
    Address("${walletAddress}")
    "withdraw"
    Address("${lsuAddress}")
    Decimal("${amount}")
  ;
  TAKE_ALL_FROM_WORKTOP
      Address("${lsuAddress}")
      Bucket("lsu_resource_address")
  ;
  CALL_METHOD
      Address("${component_id}")
      "swap_exact_lsu_for_pt"
      Bucket("lsu_resource_address")
      Decimal("${amount}")
  ;
  CALL_METHOD
      Address("${walletAddress}")
      "deposit_batch"
      Expression("ENTIRE_WORKTOP")
  ;
  `;
}

//Broken atm
function swapYTforLSU(amount){
  return `
  CALL_METHOD
    Address("${walletAddress}")
    "withdraw"
    Address("${ytResourceAddress}")
    Decimal("${amount}")
  ;
  TAKE_ALL_FROM_WORKTOP
      Address("${ytResourceAddress}")
      Bucket("yt_resource")
  ;
  CALL_METHOD
      Address("${component_id}")
      "swap_exact_yt_for_lsu"
      Bucket("yt_resource")
      Decimal("1")
  ;
  CALL_METHOD
      Address("${walletAddress}")
      "deposit_batch"
      Expression("ENTIRE_WORKTOP")
  ;
  `;
}

function swapLSUforYT(amount){
  return `
  CALL_METHOD
    Address("${walletAddress}")
    "withdraw"
    Address("${lsuAddress}")
    Decimal("${amount}")
  ;
  TAKE_ALL_FROM_WORKTOP
      Address("${lsuAddress}")
      Bucket("lsu_resource_address")
  ;
  CALL_METHOD
      Address("${component_id}")
      "swap_exact_lsu_for_yt"
      Bucket("lsu_resource_address")
  ;
  CALL_METHOD
      Address("${walletAddress}")
      "deposit_batch"
      Expression("ENTIRE_WORKTOP")
  ;
  `;
}
</script>

<template>
  <div>
    <UCard>
      <template #header>
        Swap
      </template>
    
      <div>Balance: {{ state.balance }}</div>
      <UButtonGroup size="xl" orientation="horizontal">
        <UDropdown :items="itemsTop" :popper="{ placement: 'bottom-start' }">
          <UButton color="white" :label="swapTop" trailing-icon="i-heroicons-chevron-down-20-solid" />
        </UDropdown>
        <UInput v-model="state.deposit" />
      </UButtonGroup>
      
      <div>Balance: {{ state.balance }}</div>
      <UButtonGroup size="xl" orientation="horizontal">
        <UDropdown :items="itemsBottom" :popper="{ placement: 'bottom-start' }">
          <UButton color="white" :label="swapBottom" trailing-icon="i-heroicons-chevron-down-20-solid" />
        </UDropdown>
        <UInput v-model="state.deposit" />
      </UButtonGroup>

      <template #footer>
        <UButton @click="swap()" block>Swap</UButton>
      </template>
    </UCard>
  </div>
</template>

<style scoped></style>
