<script setup lang="ts">
import { type ItemCardProps } from '@/utils/ItemCardProps'

defineProps<ItemCardProps>()
</script>

<template>
  <div id="container"></div>
</template>

<style lang="less">
#container {
  width: 12em;
  height: 18em;

  border: 2px red solid;
  border-radius: var(--border-radius);
}
</style>
