<script setup lang="ts"></script>

<template>
  <nav data-tauri-drag-region></nav>
</template>

<style scoped lang="less">
nav {
  box-sizing: border-box;
  position: fixed;
  overflow: hidden;
  z-index: 99;

  width: 100vi;
  height: 50px;
  top: 0;

  border-bottom: 1px red solid;
}
</style>
