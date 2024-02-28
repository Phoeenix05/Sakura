<!-- doesn't currently work -->

<script setup lang="ts">
import { type ItemCardProps } from '@/utils/ItemCardProps'

defineProps<ItemCardProps>()
</script>

<template>
  <section>
    <img :src="$props.thumbnail" alt="thumbnail" />
    <div>
      <h1>{{ $props.title }}</h1>
    </div>
  </section>
</template>

<style scoped lang="less">
@import '@/styles/mixins/itemcard.less';

section {
  .defaults();

  display: grid;
  grid-row: 1;
  grid-column: 2;

  width: 24em;
  height: 18em;

  & img {
    width: 50%;
    height: 100%;
    object-fit: cover;
  }
}
</style>
