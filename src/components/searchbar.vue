<script setup lang="ts">
const query = ref("")

const nav = () => {
    if (query.value != "") {
        navigateTo('/search/' + query.value)
    }
}
</script>

<template>
    <div class="flex items-center">
        <input type="text" v-model="query" class="border-[1px] border-black rounded-md m-1 p-1">
        <p @click="nav">search</p>
    </div>
</template>