<script setup lang="ts">
import useFollowStore from '@/stores/following'
import { fetch } from '@tauri-apps/api/http'
import { Output } from 'valibot'

const { data, pending, error } = useAsyncData(async () => {
	const followStore = useFollowStore()
	const ids = followStore.idsQueryString

	if (!ids.length) {
		return
	}

	const url = construct_url('https://api.mangadex.org/manga', [
		['includes[]', 'manga'],
		['includes[]', 'author'],
		['includes[]', 'artist'],
		['includes[]', 'cover_art'],
		...ids
	])

	const response = await fetch<Output<typeof MangaListSchema>>(url.toString(), {
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
		<template class="flex flex-wrap justify-center">
			<CardManga
				v-for="manga in data?.data"
				:key="manga.id"
				:storeId="manga.id"
				:mangaData="manga"
				variant="compact"
			/>
		</template>
	</template>

	<template v-else-if="pending">
		<p>Loading...</p>
	</template>

	<template v-else-if="error">
		<p>An error occurred</p>
		<pre>{{ error }}</pre>
	</template>
</template>
