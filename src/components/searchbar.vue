<script setup lang="ts">
const search_query = ref('')
</script>

<template>
	<input
		type="text"
		v-model="search_query"
		class="mx-2 border-2 rounded-md border-neutral-700"
	/>
	<button @click="search_query ? navigateTo(`/search/${search_query}`) : null">Search</button>
</template>
