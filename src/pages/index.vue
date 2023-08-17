<script setup lang="ts">
// NOTE: Do not remove ——————————————————
localStorage.setItem('User-Agent', await userAgent())
// ——————————————————————————————————————

import { useFollowingStore } from '@/stores/following';

const following = useFollowingStore()
</script>

<template>
    <template v-for="item in following.all">
        <NuxtLink :to="`/manga/${item.id}`">
            <p>{{ item.id }}</p>
        </NuxtLink>
        <button @click="following.removeManga(item.hash)">remove from history</button>
    </template>
</template>