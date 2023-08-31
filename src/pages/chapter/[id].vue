<script setup lang="ts">
import { fetch } from '@tauri-apps/api/http'

const { id } = useRoute().params

const {
	data: chapter,
	pending: chapter_pending,
	error: chapter_error
} = useAsyncData(async () => {
	return (
		await fetch<any>(`https://api.mangadex.org/at-home/server/${id}`, {
			method: 'GET',
			headers: {
				'User-Agent': await userAgent(),
				'Cache-Control': 'max-age=870'
			}
		})
	).data
})
</script>

<template>
	<img
		v-if="!chapter_pending && !chapter_error"
		v-for="img in chapter.chapter.data"
		:src="`https://uploads.mangadex.org/data/${chapter.chapter.hash}/${img}`"
	/>
</template>
