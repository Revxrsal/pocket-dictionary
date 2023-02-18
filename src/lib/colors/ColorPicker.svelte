<script>
	import {schemes} from "./schemes.ts";
	import ColorScheme from "./ColorScheme.svelte";
</script>

<style>
	.picker {
		margin: 1em;
		display: grid;
		grid-gap: 0.75em;
		grid-template-columns: repeat(auto-fit, minmax(5em, 1fr));
	}
</style>

<div class="picker">
	{#each Object.entries(schemes) as entry}
			<ColorScheme scheme={entry[0]} color={entry[1]}/>
	{/each}
</div>