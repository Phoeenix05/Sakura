<script setup lang="ts">
import ItemCardRegular from './components/ItemCardRegular.vue'
import NavBar from './components/NavBar.vue'
</script>

<template>
  <NavBar />
  <div>
    <ItemCardRegular
      v-for="_ in 10"
      v-bind:key="_"
      source_url="_"
      sid="_"
      title="Eminence in Shadow"
      thumbnail="https://mangadex.org/covers/77bee52c-d2d6-44ad-a33a-1734c1fe696a/143ca3b0-a980-4d01-bec3-7404f01d377d.jpg.512.jpg"
      description="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
    />
  </div>
</template>

<style scoped lang="less">
div {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}
</style>
