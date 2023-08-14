<script setup lang="ts">
import { MinusIcon } from '@heroicons/vue/24/outline';

const followedStore = useFollowedMangasStore()
const mangas = followedStore.mangas
</script>

<template>
    <div v-for="manga in mangas">
        <p>{{ manga }}</p>
        <MinusIcon @click="followedStore.removeManga(manga)" class="w-6" />
    </div>
</template>