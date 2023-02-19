<script>
	import {theme} from "../../preferences/theme.ts";
	import {toggleTheme} from "../../preferences/theme.ts";
	import ColorPicker from "$lib/colors/ColorPicker.svelte";
</script>

<h1>Settings</h1>
<h2>Appearance</h2>

<style>
	div {
		margin-top: 2rem;
		margin-bottom: 2rem;
	}
	
	label {
		margin-top: 0.5em;
		margin-bottom: 1em;
	}
</style>

<div class="container">
	<label for="dark_mode">Dark mode</label>
	<input type="checkbox" role="switch" id="dark_mode" name="remember" checked={$theme === "dark"}
	       on:change={toggleTheme}>
</div>

<div class="container">
	<p>Color scheme</p>
	<ColorPicker/>
</div>