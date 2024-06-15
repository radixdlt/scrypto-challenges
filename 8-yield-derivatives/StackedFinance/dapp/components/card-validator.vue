<script lang="ts" setup>
import { RdtGatewayApiClient } from "@radixdlt/radix-dapp-toolkit";

const { $getRDT } = useNuxtApp();

async function handleButtonClick() {
  // Your custom logic here
  console.log("Button clicked!"); // Example: Log a message
  //$createComponent();
  // You can call other functions, make API requests, etc.
  const manifest = `
  CALL_FUNCTION
      Address("package_tdx_2_1p5g49dthgn56t5htw9fk72dav6jwrwavh22qkwwkaqzv8avmak0qyk")
      "YieldTokenizer"
      "instantiate_yield_tokenizer"
      Enum<1u8>()
      Address("${props.stakeUnitResourceAddress}")
  ;`;
  console.log(manifest);
  const rdt = await $getRDT();

  rdt.walletApi.sendTransaction({
    transactionManifest: manifest,
  });
}

const expiries = [
  [
    {
      label: "2025-03-31",
      shortcuts: ["365 days"],
    },
    {
      label: "2026-03-31",
      shortcuts: ["700 days"],
    },
  ],
];
const props = defineProps([
  "validatorAddress",
  "claimTokenResourceAddress",
  "stakeUnitResourceAddress",
  "feeFactor",
]);

const validatorAddress =
  props.validatorAddress.substring(0, 20) +
  "..." +
  props.validatorAddress.substring(props.validatorAddress.length - 6);
</script>

<template>
  <div>
    <UCard>
      <template #header>
        <div class="flex flex-col items-center gap-y-2">
          <h2 class="flex">Validator: {{ validatorAddress }}</h2>
          <h2 class="flex">Claim Token: {{ claimTokenResourceAddress }}</h2>
          <h2 class="flex">Stake Unit: {{ stakeUnitResourceAddress }}</h2>
          <h2 class="flex">Fee Factor: {{ feeFactor }}</h2>
          <UDropdown
            class="flex"
            :items="expiries"
            :popper="{ placement: 'bottom-start' }"
          >
            <UButton
              color="white"
              label="Expiry"
              trailing-icon="i-heroicons-chevron-down-20-solid"
            />
          </UDropdown>
        </div>
      </template>
      <div class="flex justify-center">30% APY</div>

      <template #footer>
        <UButton color="white" variant="solid" @click="handleButtonClick"
          >Issue Product</UButton
        >
      </template>
    </UCard>
  </div>
</template>

<style scoped></style>
