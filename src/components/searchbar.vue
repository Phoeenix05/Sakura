<script setup lang="ts">
const query = ref("")

const navigate = (q: string) => {
    if (q == "") {
        return
    }
    navigateTo(`/search/${q}`)
}
</script>

<template>
    <input name="search" type="text" v-model="query" class="border-2 rounded-lg border-black/75">
    <label for="search" @click="navigate(query)">Search</label>
</template>