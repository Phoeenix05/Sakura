<script setup lang="ts">
import { fetch } from '@tauri-apps/api/http'

// Route params
const { id } = useRoute().params

const {
	data: feed,
	pending,
	error
} = useAsyncData(async () => {
	const url = construct_url(`https://api.mangadex.org/manga/${id}/feed`, [
		['limit', '500'],
		['translatedLanguage[]', 'en'],
		['order[chapter]', 'desc']
	])

	const response = await fetch<any>(url, {
		method: 'GET',
		headers: {
			'User-Agent': await userAgent()
		}
	})
	return response.data
})
</script>

<template>
	<template v-if="!pending">
		<NuxtLink
			class="mb-2 ml-2"
			:to="`/chapter/${chapter.id}`"
			v-for="chapter in feed.data"
		>
			{{ chapter.attributes.title || 'null' }}
			<br />
		</NuxtLink>
		<pre>{{ JSON.stringify(feed, null, 2) }}</pre>
	</template>

	<template v-else-if="pending">
		<p>Loading...</p>
	</template>

	<template v-else-if="error">
		<p>An error occurred</p>
		<pre>{{ error }}</pre>
	</template>
</template>
