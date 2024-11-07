<script lang="ts" setup>
const { $getRDT, $getEntityDetails } = useNuxtApp();
const props = defineProps(['componentAddress']);

//Get tokenizer vault details
const entityData = await $getEntityDetails(props.componentAddress);
console.log(entityData);

const lsuVaultAddress = useFindByFieldName(entityData.details.state.fields, 'lsu_vault').findByFieldName;
const lsuValidatorAddress = useFindByFieldName(entityData.details.state.fields, 'lsu_validator_component').findByFieldName;
const lsuVaultDetails = await $getEntityDetails(lsuVaultAddress);
const lsuVaultBalance = lsuVaultDetails.details.balance.amount;

//Getting the entitiy details so that we can get the validator name
const validatorDetails = await $getEntityDetails(lsuValidatorAddress);
//This parses the meta data to try and get the validator name
const validatorName = useFindbyMetaData(validatorDetails.metadata, 'name').typed.value;

const items = [
  [
    {
      label: "12 Months",
      shortcuts: ["365 days"],
    },
    {
      label: "18 Months",
      shortcuts: ["365 days"],
    },
    {
      label: "12 Months",
      shortcuts: ["365 days"],
    },
  ],
];

</script>

<template>
  <div>
    <UCard>
      <template #header>
        <div class="flex flex-col items-center gap-y-2">
          <h2 class="flex">{{validatorName}}</h2>
        </div>
      </template>
      <div class="flex justify-center">Vault TVL: {{ lsuVaultBalance }}</div>

      <template #footer class="text-center">
        <UButton :to="`/fixed-income/${componentAddress}`" target="_self" block>Deposit Stake</UButton>
      </template>
    </UCard>
  </div>
</template>

<style scoped></style>
