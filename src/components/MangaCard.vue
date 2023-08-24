<template>
    <template v-if="$props.style == 'large'">
        <NuxtLink :to="`/manga/${$props.uuid}`">
            <p>{{ $props.uuid }}</p>
        </NuxtLink>
        <button @click="following.removeManga($props.hash)">remove from history</button>
    </template>

    <!-- ———————————————————————————————————————————————————————————— -->

    <template v-if="$props.style == 'small'">

    </template>
</template>

<script lang="ts">
export default defineComponent({
    // setup: (props) => {
    //     props.id
    // },
    props: {
        uuid: {
            type: String,
            required: true,
        },
        size: {
            type: String,
            required: true,
        }
    },
})
</script>