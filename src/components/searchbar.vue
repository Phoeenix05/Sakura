<script setup lang="ts">
const search_query = ref('')

const navigate = (q: string) => (q ? navigateTo(`/search/${q}`) : null)
</script>

<template>
	<input type="text" v-model="search_query" />
	<button @click="navigate(search_query)">Search</button>
</template>
