<script setup lang="ts">
import { fetch } from '@tauri-apps/api/http'

const { id } = useRoute().params

const {
	data: feed,
	pending: feed_pending,
	error: feed_error
} = useAsyncData(async () => {
	const url = construct_url(`https://api.mangadex.org/manga/${id}/feed`, [
		['limit', '500'],
		['translatedLanguage[]', 'en'],
		['order[chapter]', 'desc']
	])

	const response = await fetch<any>(url, {
		method: 'GET',
		headers: {
			'User-Agent': await userAgent(),
			'Cache-Control': 'max-age=3600'
		}
	})
	return response.data
})
</script>

<template>
	<template v-if="!feed_pending">
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

	<template v-else-if="feed_pending">
		<p>Loading...</p>
	</template>

	<template v-else>
		<p>An error occurred</p>
		<pre>{{ feed_error }}</pre>
	</template>
</template>
