<script setup lang="ts">
const { query } = useRoute().params
</script>

<template>
	<div>
		{{ query }}
	</div>
</template>
