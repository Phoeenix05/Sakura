<script setup lang="ts">
import { ArrowPathIcon, MinusIcon, PlusIcon } from "@heroicons/vue/24/outline";
import { fetch } from "@tauri-apps/api/http";

const { id } = useRoute().params
const follow = () => {
    const store = useFollowedMangasStore()
    store.addManga(id)
}
const unfollow = () => {
    const store = useFollowedMangasStore()
    store.removeManga(id)
}
// const { data: data, pending: data_pending, error: data_error, refresh: refresh_data } = useAsyncData(async () => {
//     const res = await fetch(`https://api.mangadex.org/manga/${id}`)
//     return res.data as any
// })
const { data: feed, pending: feed_pending, error: feed_error, refresh: refresh_feed } = useAsyncData(async () => {
    const res = await fetch(`https://api.mangadex.org/manga/${id}/feed?limit=500&translatedLanguage[]=en&order[chapter]=desc`)
    return res.data as any
})
</script>

<template>
    <div>
        <ArrowPathIcon @click="refresh_feed()" class="w-6" />
        <PlusIcon @click="follow" class="w-6" />
        <MinusIcon @click="unfollow" class="w-6" />
        <div v-if="!feed_pending && !feed_error" v-for="chapter in feed.data">
            <template v-if="chapter.attributes.title">
                <NuxtLink :to="'/chapter/' + chapter.id">{{ chapter.attributes.title }}</NuxtLink>
            </template>
            <template v-else>
                <NuxtLink :to="'/chapter/' + chapter.id">null</NuxtLink>
            </template>
        </div>
        <div v-if="feed_error">An error occurred while loading data</div>
    </div>
</template>