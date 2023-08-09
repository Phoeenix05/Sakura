<script setup lang="ts">
const route = useRoute()
</script>

<template>
    <div>
        <p>{{ route.params.id }}</p>
        <li>
            <NuxtLink to="/">Home</NuxtLink>
        </li>
    </div>
</template>
