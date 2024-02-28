<!-- doesn't currently work -->

<script setup lang="ts">
import { type ItemCardProps } from '@/utils/ItemCardProps'

defineProps<ItemCardProps>()
</script>

<template>
  <section>
    <img :src="$props.thumbnail" alt="thumbnail" />
  </section>
</template>

<style scoped lang="less">
@import '@/styles/mixins/itemcard.less';

section {
  .defaults();

  width: 8em;
  height: 12em;
}
</style>
