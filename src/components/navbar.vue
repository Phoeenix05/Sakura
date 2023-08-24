<script setup lang="ts">
const query = ref("")
</script>

<template>
    <!-- <div class="absolute z-50 h-16 transform -translate-x-1/2 border-2 border-red-500 rounded-md bottom-4 left-1/2 w-52">
    </div> -->
    <NuxtLink to="/">Home</NuxtLink>
    <br>
    <NuxtLink to="/chapter/history">History</NuxtLink>
    <br>
    <!-- <Searchbar /> -->
    <input name="search" type="text" v-model="query" class="border-2 rounded-lg border-black/75">
    <label for="search" @click="query ? navigateTo('/search/' + query) : null">Search</label>
</template>