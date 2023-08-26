<script setup lang="ts">
const search_query = ref('')
</script>

<template>
	<input
		type="text"
		v-model="search_query"
	/>
	<button @click="search_query ? navigateTo(`/search/${search_query}`) : null">Search</button>
</template>
