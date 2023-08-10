<script setup lang="ts">
import { fetch } from "@tauri-apps/api/http";

const route = useRoute()

const { data, pending, error, refresh } = useAsyncData('', async () => {
    const res = await fetch(`https://api.mangadex.org/manga/${route.params.id}`)
    return res.data as any
})
</script>

<template>
    <div>
        {{ route.params.query }}
        <button @click="refresh()">refresh</button>
        <div v-if="!pending && !error">
            <pre>{{ JSON.stringify(data, null, 2) }}</pre>
        </div>
        <div v-if="error">An error occurred while loading data</div>
    </div>
</template>