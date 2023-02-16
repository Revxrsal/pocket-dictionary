<script>
	import {getWord} from "../dictionary/mw.ts";
	
	let query = "";
	let searchButton;
</script>

<h3>Pocket Dictionary</h3>

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
			type="text"
			bind:value={query}
			on:keyup={event => {
				if (event.key === "Enter") searchButton.click();
			}}>
	<button
			bind:this={searchButton}
			disabled={query === ""}
			class="search_button"
			on:click={() => {
				getWord(query).then(result => {
					alert(JSON.stringify(result))
				})
			}}
	>Search
	</button>
</div>