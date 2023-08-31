<script setup lang="ts">
import { fetch } from '@tauri-apps/api/http'
import { Output } from 'valibot'

// Route params
const { query } = useRoute().params

const { data, pending, error } = useAsyncData(async () => {
	const url = construct_url('https://api.mangadex.org/manga', [
		['title', query as string],
		['limit', '100'],
		['order[followedCount]', 'desc'],
		['includes[]', 'manga'],
		['includes[]', 'author'],
		['includes[]', 'artist'],
		['includes[]', 'cover_art'],
		['availableTranslatedLanguage[]', 'en']
	])

	// Had to use this `Output<typeof MangaListSchema>` here as nuxt3 auto imports
	// don't seem to auto import `MangaList` and when you import it will yell about
	// the file being already included.
	const response = await fetch<Output<typeof MangaListSchema>>(url.toString(), {
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
	<template v-if="!pending">
		<template class="flex flex-wrap justify-center">
			<CardManga
				v-for="manga in data?.data"
				:key="manga.id"
				:storeId="manga.id"
				:mangaData="manga"
				variant="full"
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
