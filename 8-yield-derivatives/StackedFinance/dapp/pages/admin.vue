<script lang="ts" setup>
const items = [{
  label: 'Tokenize a new Validator LSU',
  icon: 'i-heroicons-information-circle',
  defaultOpen: true,
  slot: 'tokenize'
}, {
  label: 'Add an AMM',
  icon: 'i-heroicons-arrow-down-tray',
  slot: 'add-amm'
}]
</script>

<template>
  <UContainer class="flex flex-col gap-y-4">
    <UCard>
      <p>This would be an admin area locked behind a badge for managing issuing of new 
        tokenized yield components and their AMM's (Admin Badge Not Implmented)</p>
    </UCard>
    <UAccordion :items="items">
      <template #tokenize>
        <UCard class="text-gray-900 dark:text-white">
          <ol class="list-decimal p-2">
            <li>Go to the Available Validators Section</li>
            <li>Click the Action button</li>
            <li>Select the Maturity Range</li>
            <li>Approve Transaction</li>
            <li>Copy the Resulting Component to <strong>yieldComponentAddress</strong> in the liquidity.json (Ideally this data is keystore in the future)</li>
          </ol>
        </UCard>
      </template>
      <template #add-amm>
        <UCard class="text-gray-900 dark:text-white">
          <ol class="list-decimal p-2">
            <li>If you haven't already, create the yield tokenizer</li>
            <li>Click Create AMM</li>
            <li>Approve Transaction</li>
            <li>Update the associated <strong>ammComponentAddress</strong> in the liquidity.json</li>
            <li>Set the Implied Rate to above 1</li>
            <li>Add some liquidity or you will get an error message</li>
          </ol>
        </UCard>
      </template>
    </UAccordion>
    <UAlert title="Existing Products"/>
    <validator-management></validator-management>
    <UAlert title="Available Validators"/>
    <validator-list></validator-list>
  </UContainer>
</template>

<style scoped></style>
