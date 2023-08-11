<script setup lang="ts">
const query = ref("tensura")
</script>

<template>
    <main>
        {{ query }}
        <input type="text" v-model="query">
        <NuxtLink :to="'/search/' + query">test</NuxtLink>
    </main>
</template>