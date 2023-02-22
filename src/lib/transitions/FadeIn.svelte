<script>
	import {fade} from "svelte/transition";
	
	export let path;
	export let duration = 100;
</script>

{#key path}
	<div in:fade={{duration: duration}}>
		<slot/>
	</div>
{/key}