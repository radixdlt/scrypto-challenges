<script lang="ts" setup>
const route = useRoute()
const component_id = route.params.id;
const state = useState('counter');
const { $getEntityDetails } = await useNuxtApp();

const maturities = ref([]);

onMounted(async () => { 
  const entityData = await $getEntityDetails(component_id);
  //Get the validator address for assembling the manifest
  const maturityDate = useFindTuple(entityData.details.state.fields, 'maturity_date').findByFieldName;
  //Parse the maturity date 
  const dateTime = useParseMaturity(maturityDate);
  console.log(maturityDate);
  console.log(dateTime.toString());
  maturities.value.push({
    value: dateTime.toLocaleDateString(),
    label: dateTime.toLocaleDateString(),
    disabled: false,
  })
});
</script>

<template>
  <div>
    <UCard>
    <template #header>
      Step 2: Select Maturity
    </template>
    <URadioGroup v-model="state.maturity" :options="maturities" />
    <template #footer>
      Selected {{ state.maturity }}
    </template>
  </UCard>
  </div>
</template>

<style scoped></style>
