<script setup lang="ts"></script>

<template>
  <nav data-tauri-drag-region></nav>
</template>

<style scoped lang="less">
nav {
  box-sizing: border-box;
  position: fixed;
  top: 0;
  left: 0;
  z-index: 99;

  width: 100vi;
  height: 50px;

  background-color: var(--bg-secondary-night);
  border-bottom: var(--border) var(--border-eerie) solid;
}
</style>
