<script setup lang="ts">
import index from "../data/index.json";
</script>

<template>
  <UContainer class="flex w-full flex-col gap-y-4">
    <UContainer class="flex w-full flex-row justify-around items-center cover-gradient">
      <div class="flex flex-col gap-y-4">
        <h1 class="text-4xl animate-in slide-in-from-top">Sign Up Now!</h1>
        <h2 class="text-2xl">A Fast Growing Web3 App</h2>
        <div class="flex flex-row gap-x-4">
          <UButton class="flex" to="/fixed-income">Get Started</UButton>
          <UButton class="flex" >Learn More</UButton>
        </div>
      </div>
      <div class="flex">
        <img src="https://placehold.co/400x400">
      </div>
    
    </UContainer>
    <UContainer class="flex">
      <UCard>
        <div class="flex justify-between">
          <h1 class="flex">Current Market</h1>
          <UButton class="flex" to="fixed-income">View More</UButton>
        </div>

        <UTable :rows="index" />

        <template #footer>
          <Placeholder class="h-8" />
        </template>
      </UCard>
      
    </UContainer>
  </UContainer>
</template>

<style scoped>
.cover-gradient {
    background: linear-gradient(169.4deg,rgba(57,132,244,.1) -6.01%,rgba(12,211,255,.1) 36.87%,rgba(47,124,240,.1) 78.04%,rgba(14,101,232,.1) 103.77%)
}
</style>
