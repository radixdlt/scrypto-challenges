<script lang="ts" setup>
import {type ValidatorCollectionItem } from '@radixdlt/radix-dapp-toolkit'
const { $getRDT, $getValidators } = useNuxtApp();
const validators = ref<ValidatorCollectionItem>([]);
const addresses = ref([]);

enum Duration {
  TwelveMonths = 0,
  EighteenMonths = 1,
  TwentyFourMonths = 2,
}

async function instantiateTokenizer(stakeUnitResourceAddress: string, duration: Duration) {
  // Creating the manifest to call the YieldTokenizer
  const manifest = `
  CALL_FUNCTION
      Address("package_tdx_2_1p4vfemgll9y7ykuhrsfymdyuxcd5wr4stpncle8t2we8aptff440u8")
      "YieldTokenizer"
      "instantiate_yield_tokenizer"
      Enum<${duration}u8>()
      Address("${stakeUnitResourceAddress}")
  ;`;
  const rdt = await $getRDT();

  rdt.walletApi.sendTransaction({
    transactionManifest: manifest,
  });
}

const columns = [{
  key: 'actions',
  label: 'actions',
},{
  key: 'feeFactor',
  label: 'Fee Factor'
},{
  key: 'validatorAddress',
  label: 'Address'
}, {
  key: 'claimTokenResourceAddress',
  label: 'Claim Token Resource'
}, {
  key: 'stakeUnitResourceAddress',
  label: 'Stake Unit Resource'
} ]

const actions = (row) => [
  [{
    label: 'Tokenize 12 Months',
    icon: 'i-heroicons-pencil-square-20-solid',
    click: () => instantiateTokenizer(row.stakeUnitResourceAddress,Duration.TwelveMonths)
  },{
    label: 'Tokenize 18 Months',
    icon: 'i-heroicons-pencil-square-20-solid',
    click: () => instantiateTokenizer(row.stakeUnitResourceAddress,Duration.EighteenMonths)
  },{
    label: 'Tokenize 24 Months',
    icon: 'i-heroicons-pencil-square-20-solid',
    click: () => instantiateTokenizer(row.stakeUnitResourceAddress,Duration.TwentyFourMonths)
  }]
]


async function handleButtonClick() {
  try{
    console.log(1);
    validators.value = await $getValidators();
    console.log(validators)
  }catch(error){
    console.log(error);
  }
  
  //console.log(validators);
}

onMounted(async () => {
  const nftAddresses = (await $getValidators()).map(item => ({
    //name: item.metadata.items[3],
    feeFactor: item.effective_fee_factor.current.fee_factor,
    validatorAddress: item.address,
    
    claimTokenResourceAddress: item.state.claim_token_resource_address,
    stakeUnitResourceAddress: item.state.stake_unit_resource_address,
  }));

  addresses.value = nftAddresses;

  validators.value = await $getValidators();
  //console.log(await $getValidators());
});

</script>

<template>
  <UTable :rows="addresses" :columns="columns">
    <template #actions-data="{ row }">
      <UDropdown :items="actions(row)">
        <UButton color="gray" variant="ghost" icon="i-heroicons-ellipsis-horizontal-20-solid" />
      </UDropdown>
    </template>
  </UTable>
</template>

<style scoped></style>
