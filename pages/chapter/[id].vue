<script setup lang="ts">
import { useMangaStore } from "@/stores/manga";
import { fetch } from "@tauri-apps/api/http";

const { id } = useRoute().params
const manga = useMangaStore()
console.log(manga.$manga_id)

const { data, pending, error, refresh } = useAsyncData(async () => {
    const res = await fetch(`https://api.mangadex.org/at-home/server/${id}?forcePort443=false`)
    return res.data as any
})
const construct_img_url = (base: string, hash: string, img: string) => `${base}/data/${hash}/${img}`
</script>

<template>
    {{ id }}
    <div v-if="!pending && !error">
        <template v-for="img in data.chapter.data">
            <img class="w-[768px]" :src="construct_img_url(data.baseUrl, data.chapter.hash, img)" alt="">
        </template>
    </div>
    <div v-else>An error occurred while loading data</div>
</template>