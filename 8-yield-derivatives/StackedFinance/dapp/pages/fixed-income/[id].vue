<script lang="ts" setup>
const counter = useState('counter', () => ({
  maturity: 0,
  deposit: 0,
  balance: 0,
  lsuAddress:'',
  XRDBalance:0,
}))
const toast = useToast();
const route = useRoute();
const component_id = route.params.id;
//Check if the component is valid from the package
//Get users Balance of the token

const { $getEntityDetails, $getLSUBalance, $getXRDBalance } = await useNuxtApp();

//await callOnce(async () => {
  //websiteConfig.value = await $fetch('https://my-cms.com/api/website-config')
  const entityData = await $getEntityDetails(component_id);
  //console.log(entityData)
  const { findByFieldName } = useFindByFieldName(entityData.details.state.fields, 'lsu_address');
  console.log(findByFieldName)
  counter.value.lsuAddress = findByFieldName;
  //Ge users balances from LSU
  try{
    counter.value.balance = await $getLSUBalance(findByFieldName);
    counter.value.XRDBalance = await $getXRDBalance();
  }catch(error){
    toast.add({ 
      title: 'Please Connect your wallet this site to function', 
      color : 'red',
      timeout: 0 });
  } 
//})

</script>

<template>
  <UContainer class="flex flex-col gap-y-6">
    <fixed-income-stake></fixed-income-stake>
    <fixed-income-maturity></fixed-income-maturity>
    <fixed-income-deposit></fixed-income-deposit>
  </UContainer>
</template>

<style scoped></style>
