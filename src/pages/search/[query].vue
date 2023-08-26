<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { fetch } from '@tauri-apps/api/http'

// Route params
const { query } = useRoute().params

const { data, pending, error } = useAsyncData(async () => {
	const url = new URL('https://api.mangadex.org/manga')
	url.searchParams.append('title', query as string)
	url.searchParams.append('availableTranslatedLanguage[]', 'en')

	const response = await fetch(url.toString(), {
		method: 'GET',
		headers: {
			'User-Agent': await invoke<string>('user_agent')
		}
	})
	console.log(response.data)
	return response.data as any
})
</script>

<template>
	<template v-if="!pending">
		<p v-for="manga in data.data">
			<CardManga
				:storeId="manga.id"
				:data="manga"
			/>
		</p>
	</template>

	<template v-else-if="pending">
		<p>Loading...</p>
	</template>

	<template v-else-if="error">
		<p>An error occurred: {{ error }}</p>
	</template>
</template>
