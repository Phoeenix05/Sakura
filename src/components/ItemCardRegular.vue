<script setup lang="ts">
import { type ItemCardProps } from '@/utils/ItemCardProps'
defineProps<ItemCardProps>()
</script>

<template>
  <section>
    <img :src="$props.thumbnail" alt="thumbnail" />
    <span>
      <h1>{{ $props.title }}</h1>
      <p>{{ $props.description }}</p>
    </span>
  </section>
</template>

<style scoped lang="less">
@import '@/styles/mixins/itemcard.less';

section {
  .defaults();

  width: 12em;
  height: 16em;

  & img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    object-position: center;
    transition: all 125ms;
  }

  &:hover img {
    filter: brightness(50%) blur(1px);
  }
}
</style>
