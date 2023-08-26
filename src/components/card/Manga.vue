<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { fetch } from '@tauri-apps/api/http'

const props = defineProps({
	storeId: {
		type: String,
		required: true
	}
})

const data2 = ref()

const { /* data, */ pending, error } = useAsyncData(async () => {
	const url = new URL(`https://api.mangadex.org/manga/${props.storeId}`)
	url.searchParams.append('translatedLanguage[]', 'en')
	url.searchParams.append('includes[]', 'cover_art')

	const response = await fetch(url.toString(), {
		method: 'GET',
		headers: {
			'User-Agent': await invoke<string>('user_agent')
		}
	})
	data2.value = response.data
	// return response.data as any
})

const image = (id: string): string => {
	const cover_art = data2.value.data.relationships.find((e: any) => e.type == 'cover_art')
	const fileName = cover_art.attributes.fileName

	const url = new URL('https://mangadex.org/')
	url.pathname = `covers/${props.storeId}/${fileName}`

	return url.toString()
}
</script>

<template>
	<template v-if="!pending">
		<div class="flex m-2 space-x-4 cursor-pointer">
			<img
				:src="image($props.storeId)"
				class="w-32 rounded-md"
			/>
			<div class="space-y-2">
				<!-- Title -->
				<p class="text-xl font-bold w-[512px] truncate">
					{{ data2.data.attributes.title.en || data2.data.attributes.title['ja-ro'] }}
				</p>

				<!-- Description -->
				<!-- TODO: Add truncation for description -->
				<p class="w-[482px] h-24 overflow-hidden">{{ data2.data.attributes.description.en }}</p>

				<!-- Latest chapter -->
				<!-- <a
					:href="`https://mangadex.org/chapter/${data.data.attributes.latestUploadedChapter}`"
					target="_blank"
					class="w-[512px] h-7 text-neutral-300 font-semibold cursor-pointer"
				>
					{{ data.data.attributes.latestUploadedChapter }}
				</a> -->
			</div>
		</div>
	</template>

	<template v-else-if="pending">
		<div class="flex m-2 space-x-4">
			<USkeleton class="w-32 rounded-md h-[182px]" />
			<div class="space-y-2">
				<USkeleton class="w-[512px] h-7" />
				<USkeleton class="w-[482px] h-24" />
				<!-- <USkeleton class="w-[356px] h-7" /> -->
			</div>
		</div>
	</template>

	<template v-else-if="error">
		<!-- Not sure what to put here -->
	</template>
</template>
