<script setup lang="ts">
defineProps<{
  href: string
}>()
</script>

<template>
  <slot name="thumbnail"></slot>
  <slot name="title"></slot>
  <slot name="description"></slot>
</template>
