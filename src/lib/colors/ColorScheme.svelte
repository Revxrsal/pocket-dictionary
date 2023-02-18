<script>
	import {setColorScheme} from "../../preferences/theme.ts";
	import {colorScheme} from "../../preferences/theme.ts";
	import Tick from "$lib/icons/Tick.svelte";
	import {fade, fly} from 'svelte/transition';
	
	export let scheme;
	export let color;
</script>

<style>
	button {
		width: 40px;
		height: 40px;
		border: 0;
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>

<button class="container" style:background-color={color} aria-label="Activate ${color} theme" on:click={() => {
	setColorScheme(scheme)
}}>
	{#if $colorScheme === scheme}
		<div in:fade={{duration: 100}}>
			<Tick/>
		</div>
	{/if}
</button>
