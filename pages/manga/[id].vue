<script setup lang="ts">
import { fetch } from "@tauri-apps/api/http";

const { id } = useRoute().params

const { data: data, pending: data_pending, error: data_error, refresh: refresh_data } = useAsyncData(async () => {
    const res = await fetch(`https://api.mangadex.org/manga/${id}`)
    return res.data as any
})
const { data: feed, pending: feed_pending, error: feed_error, refresh: refresh_feed } = useAsyncData(async () => {
    const res = await fetch(`https://api.mangadex.org/manga/${id}/feed?translatedLanguage[]=en&order[chapter]=desc`)
    return res.data as any
})
</script>

<template>
    <div>
        {{ id }}
        <button @click="refresh_feed()">refresh</button>
        <div v-if="!feed_pending && !feed_error" v-for="chapter in feed.data">
            <template v-if="chapter.attributes.title">
                <NuxtLink :to="'/chapter/' + chapter.id">{{ chapter.attributes.title }}</NuxtLink>
            </template>
            <template v-else>
                <NuxtLink :to="'/chapter/' + chapter.id">null</NuxtLink>
            </template>
        </div>
        <div v-else>An error occurred while loading data</div>
    </div>
</template>