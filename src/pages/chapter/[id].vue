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
import { fetch } from "@tauri-apps/api/http";
import { useReadingHistory } from "../../stores/history";

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
    <div v-if="!pending && !error" class="flex flex-col items-center w-full">
        <div v-for="image in data?.chapter.dataSaver" class="w-[800px] h-auto">
            <img :src="`${data?.baseUrl}/data-saver/${data?.chapter.hash}/${image}`" :alt="image" loading="lazy">
        </div>
    </div>

    <!-- Error occurred -->
    <div v-if="error">
        <button @click="refresh()">Try again</button>
        <code>{{ error }}</code>
    </div>
</template>