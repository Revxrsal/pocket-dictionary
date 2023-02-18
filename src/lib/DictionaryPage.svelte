<script>
	import {getClipboard} from "../dictionary/dictionary.ts";
	import {onDestroy, onMount} from "svelte";
	import {getEntryFromURL, lookup} from "../dictionary/sanitizer.ts";
	import {page} from "$app/stores";
	import Definition from "$lib/Definition.svelte";
	
	export let query;
	export let clickSearch = "";
	
	let searchButton;
	let result = "";
	let clipboard = "";
	
	$: value = () => {
		const url = $page.url.href;
		const entry = getEntryFromURL(url)
		lookup(entry, word => query = word).then(res => result = res)
	}
	
	/* When page loads */
	onMount(async () => {
		if (query)
			lookup(query, word => query = word).then(res => result = res)
	})
	
	/* Listen to href clicks that redirect to other words */
	$: {
		if (clickSearch)
			lookup(clickSearch, word => query = word).then(res => result = res)
	}
	
	/* Listen to clipboard changes */
	$:{
		if (clipboard)
			lookup(clipboard, word => query = word).then(res => result = res)
	}
	
	/* Listen to clipboard changes */
	const interval = setInterval(async () => {
		const newClipboard = await getClipboard()
		if (clipboard !== newClipboard)
			query = clipboard = newClipboard // triggers a page refresh
	}, 100);
	
	/* Clean the interval of checking the clipboard */
	onDestroy(() => clearInterval(interval));
</script>

<h1>Pocket Dictionary</h1>

<style>
	.input_area {
		width: 80%;
		margin: 0 1rem;
	}
	
	.search_button {
		width: 20%;
		margin: 0 1rem;
	}
	
	.input_group {
		display: flex;
	}
</style>

<div class="input_group">
	<input
			class="input_area"
			placeholder="Word here..."
			role="searchbox"
			type="search"
			bind:value={query}
			on:keyup={event => {
				if (event.key === "Enter") searchButton.click();
			}}>
	<button
			bind:this={searchButton}
			disabled={query === ""}
			class="search_button"
			on:click={async () => lookup(query, word => query = word).then(res => result = res)}
	>Search
	</button>
</div>

<Definition bind:result/>
