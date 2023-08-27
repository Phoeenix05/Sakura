<script setup lang="ts">
import useFollowStore from '@/stores/following'
import { Output } from 'valibot'

const props = defineProps({
	storeId: {
		type: String,
		required: true
	},
	mangaData: {
		type: Object,
		required: true
	},
	variant: {
		type: String,
		default: 'full'
	}
})

const id = props.storeId
const data = props.mangaData as Output<typeof MangaDataSchema>
const title = data.attributes.title.en || data.attributes.title['ja-ro']
const description = data.attributes.description.en || data.attributes.description['ja-ro']
const image_src = ((): string => {
	const cover_file = data.relationships.find((e: any) => e.type == 'cover_art').attributes.fileName
	return `https://mangadex.org/covers/${id}/${cover_file}`
})()
const store = useFollowStore()
</script>

<template>
	<template v-if="$props.variant == 'full'">
		<div class="flex m-2 space-x-4 cursor-pointer">
			<img
				@click="navigateTo(`/manga/${id}`)"
				:src="image_src"
				class="w-32 rounded-sm"
			/>
			<div class="space-y-2">
				<p
					@click="navigateTo(`/manga/${id}`)"
					class="text-xl font-bold w-[512px] truncate"
				>
					{{ title }}
				</p>
				<p
					@click="navigateTo(`/manga/${id}`)"
					class="w-[482px] h-24 overflow-hidden"
				>
					{{ description }}
				</p>
				<div class="space-x-2">
					<UButton
						@click="store.add(id)"
						label="Add"
						size="xs"
					/>
					<UButton
						@click="store.remove(id)"
						label="Remove"
						color="red"
						size="xs"
					/>
				</div>
			</div>
		</div>
	</template>

	<template v-if="$props.variant == 'compact'">
		<div class="flex m-2 space-x-4 cursor-pointer">
			<img
				@click="navigateTo(`/manga/${id}`)"
				:src="image_src"
				class="w-32 rounded-sm"
			/>
		</div>
	</template>
</template>
