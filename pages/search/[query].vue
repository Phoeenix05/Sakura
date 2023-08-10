<script setup lang="ts">
import { fetch } from "@tauri-apps/api/http";

const { query } = useRoute().params

const { data, pending, error, refresh } = useAsyncData(async () => {
    const res = await fetch(`https://api.mangadex.org/manga?title=${query}&limit=20&includes[]=cover_art&order[relevance]=desc`)
    return res.data as any
})
</script>

<template>
    <div>
        {{ query }}
        <button @click="refresh()">refresh</button>
        <div v-if="!pending && !error">
            <div v-for="manga in data.data">
                <NuxtLink :to="'/manga/' + manga.id">{{ manga.attributes.title.en }}</NuxtLink>
            </div>
        </div>
        <div v-else>An error occurred while loading data</div>
    </div>
</template>