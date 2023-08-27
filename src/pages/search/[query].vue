<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { fetch } from '@tauri-apps/api/http'
import { Output } from 'valibot'

// Route params
const { query } = useRoute().params

const { data, pending, error } = useAsyncData(async () => {
	const url = new URL('https://api.mangadex.org/manga')
	url.searchParams.append('title', query as string)
	url.searchParams.append('limit', '100')
	url.searchParams.append('order[followedCount]', 'desc')
	url.searchParams.append('includes[]', 'manga')
	url.searchParams.append('includes[]', 'author')
	url.searchParams.append('includes[]', 'artist')
	url.searchParams.append('includes[]', 'cover_art')
	url.searchParams.append('availableTranslatedLanguage[]', 'en')

	// Had to use this `Output<typeof MangaListSchema>` here as nuxt3 auto imports
	// don't seem to auto import `MangaList` and when you import it will yell about
	// the file being already included.
	const response = await fetch<Output<typeof MangaListSchema>>(url.toString(), {
		method: 'GET',
		headers: {
			'User-Agent': await invoke<string>('user_agent')
		}
	})
	return response.data
})
</script>

<template>
	<template v-if="!pending">
		<CardManga
			v-for="manga in data?.data"
			:storeId="manga.id"
			:mangaData="manga"
		/>
	</template>

	<template v-else-if="pending">
		<p>Loading...</p>
	</template>

	<template v-else-if="error">
		<p>An error occurred: {{ error }}</p>
	</template>
</template>
