<script setup lang="ts">
const query = ref("")
const isOpen = ref(false)

const { metaSymbol } = useShortcuts()
defineShortcuts({
    enter: {
        usingInput: 'search',
        handler: () => {
            if (query.value == "") {
                return
            }
            isOpen.value = false
            navigateTo(`/search/${query.value}`)
        }
    },
    meta_k: {
        usingInput: true,
        handler: () => { isOpen.value = !isOpen.value }
    }
})
</script>

<template>
    <!-- <input name="search" type="text" v-model="query" class="border-2 rounded-lg border-black/75">
    <label for="search" @click="query ? navigate(query) : null">Search</label> -->
    <UModal v-model="isOpen">
        <UInput color="gray" variant="outline" name="search" v-model="query">
            <template #trailing>
                <!-- <UKbd :value="metaSymbol" class="mr-1" /> -->
                <UKbd value="Enter" />
            </template>
        </UInput>
        <!-- <UButton color="primary" variant="solid" @click="navigate(query)">Search</UButton> -->
    </UModal>
</template>