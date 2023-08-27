<template>
	<NuxtLink to="/">Home</NuxtLink>
	<Searchbar />
	<slot></slot>
</template>

<style>
* {
	user-select: none !important;
}
</style>
