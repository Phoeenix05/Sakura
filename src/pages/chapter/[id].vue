<script setup lang="ts">
interface AtHomeServer {
    result: string
    baseUrl: string
    chapter: {
        hash: string
        data: string[]
        dataSaver: string[]
    }
}

// ————————————————————————————————————————————————————————————
import { useReadingHistory } from "@/stores/history";
import { fetch } from "@tauri-apps/api/http";

const { id } = useRoute().params
const history = useReadingHistory()
history.addItem(id)

const { data, pending, error, refresh } = useAsyncData(async () => {
    const url = `https://api.mangadex.org/at-home/server/${id}`
    const res = await fetch<AtHomeServer>(url, {
        method: 'GET',
        timeout: 10,
        headers: {
            'User-Agent': localStorage.getItem('User-Agent')!
        }
    })
    return res.data
})
</script>

<template>
    <!-- Loading data -->
    <p v-if="pending">Loading...</p>

    <!-- Data loaded successfully -->
    <main v-if="!pending && !error">
        <template v-for="image in data?.chapter.dataSaver">
            <img :src="`${data?.baseUrl}/data-saver/${data?.chapter.hash}/${image}`" :alt="image" loading="lazy">
        </template>
    </main>

    <!-- Error occurred -->
    <div v-if="error">
        <button @click="refresh()">Try again</button>
        <code>{{ error }}</code>
    </div>
</template>