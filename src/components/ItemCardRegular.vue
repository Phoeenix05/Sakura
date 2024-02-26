<script setup lang="ts">
import { type ItemCardProps } from '@/utils/ItemCardProps'

defineProps<ItemCardProps>()
</script>

<template>
  <section></section>
</template>

<style scoped lang="less">
section {
  margin: 0.5em;

  width: 12em;
  height: 18em;

  border: 2px red solid;
  border-radius: var(--border-radius);
}
</style>
