<script setup lang="ts">
import { fetch } from '@tauri-apps/api/http'

const { id } = useRoute().params

const { data, pending, error } = useAsyncData(async () => {
	const response = await fetch<any>(`https://api.mangadex.org/at-home/server/${id}`, {
		method: 'GET',
		headers: {
			'User-Agent': await userAgent()
		}
	})
	return response.data
})
</script>

<template>
	<img
		v-if="!pending && !error"
		v-for="img in data.chapter.data"
		:src="`https://uploads.mangadex.org/data/${data.chapter.hash}/${img}`"
	/>
</template>
