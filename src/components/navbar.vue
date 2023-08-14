<script setup lang="ts">
import { HomeIcon, BookmarkIcon } from "@heroicons/vue/24/outline";
</script>

<template>
    <div class="flex flex-row gap-2 ml-16">
        <NuxtLink to="/" class="flex flex-row">
            <HomeIcon class="w-6" />
        </NuxtLink>
        <NuxtLink to="/followed" class="flex flex-row">
            <BookmarkIcon class="w-6" />
        </NuxtLink>
        <Searchbar />
    </div>
</template>