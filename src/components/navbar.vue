<script setup lang="ts">
import { BookmarkIcon, HomeIcon } from "@heroicons/vue/24/outline";
</script>

<template>
    <div data-tauri-drag-region class="sticky top-0 flex flex-row w-full gap-2 pl-16 bg-white">
        <NuxtLink to="/" class="flex flex-row">
            <HomeIcon class="w-6" />
        </NuxtLink>
        <NuxtLink to="/followed" class="flex flex-row">
            <BookmarkIcon class="w-6" />
        </NuxtLink>
        <Searchbar />
    </div>
</template>