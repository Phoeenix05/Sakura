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

  width: 8em;
  height: 12em;

  border: 2px red solid;
  border-radius: var(--border-radius);
}
</style>
