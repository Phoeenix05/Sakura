<script setup lang="ts">
import ItemCard from '@/components/ItemCard.vue'
import { type ItemCardProps } from '@/utils/ItemCardProps'

defineProps<ItemCardProps>()
</script>

<template>
  <ItemCard href="">
    <template #thumbnail></template>
    <template #title></template>
    <template #description></template>
  </ItemCard>
</template>

<style scoped lang="less">
@import '@/styles/mixins/itemcard.less';

section {
  .defaults();

  width: 8em;
  height: 12em;
}
</style>
