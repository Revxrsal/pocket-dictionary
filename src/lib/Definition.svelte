<script>
	export let result;
</script>

<style>
	div {
		margin-top: 1.25em;
		margin-bottom: 1.25em;
	}
</style>

{#if result !== ""}
	<div>
		{@html result}
	</div>
{/if}