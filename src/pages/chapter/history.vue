<script setup lang="ts">
import { useReadingHistory } from "../../stores/history";

const history = useReadingHistory()
</script>

<template>
    <template v-for="item in history.all">
        <NuxtLink :to="`/chapter/${item.id}`">
            <p>{{ item.id }}</p>
        </NuxtLink>
        <button @click="history.removeItem(item.hash)">remove from history</button>
    </template>
</template>