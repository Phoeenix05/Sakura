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

  background-color: #222;
  border-bottom: 1px red solid;
}
</style>
