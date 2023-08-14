<script setup lang="ts">
import { fetch } from "@tauri-apps/api/http";

const { id } = useRoute().params

const { data, pending, error, refresh } = useAsyncData(async () => {
    const res = await fetch(`https://api.mangadex.org/at-home/server/${id}?forcePort443=false`)
    return res.data as any
})

const high_res = false

const construct_img_url_high_res = (base: string, hash: string, img: string) => `${base}/data/${hash}/${img}`
const construct_img_url_low_res = (base: string, hash: string, img: string) => `${base}/data-saver/${hash}/${img}`
</script>

<template>
    <div v-if="!pending && !error">
        <template v-if="high_res">
            <p>high res</p>
            <template v-for="img in data.chapter.data">
                <img class="w-[768px]" :src="construct_img_url_high_res(data.baseUrl, data.chapter.hash, img)" :alt="img"
                    loading="lazy">
            </template>
        </template>
        <template v-if="!high_res">
            <p>low res</p>
            <template v-for="img in data.chapter.dataSaver">
                <img class="w-[768px]" :src="construct_img_url_low_res(data.baseUrl, data.chapter.hash, img)" :alt="img"
                    loading="lazy">
            </template>
        </template>
    </div>
    <div v-if="error">An error occurred while loading data</div></template>