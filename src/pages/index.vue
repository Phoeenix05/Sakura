<script setup lang="ts">
// NOTE: Do not remove ——————————————————
localStorage.setItem('User-Agent', await userAgent())
// ——————————————————————————————————————

import MangaCard from "../components/MangaCard.vue";
import { useFollowingStore } from "../stores/following";

const following = useFollowingStore()
</script>

<template>
    <MangaCard v-for="item in following.all" :uuid="item.id" size="large" />
</template>