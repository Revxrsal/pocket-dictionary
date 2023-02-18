<script>
	import "@picocss/pico";
	import "../styles/color-schemes/schemes.css"
	import "../styles/app.css";
	import "../styles/dictionary.css";
	import Settings from "$lib/icons/Settings.svelte";
	import {page} from "$app/stores";
	import Home from "$lib/icons/Home.svelte";
	import {colorScheme} from "../preferences/theme.ts";
	import {crossfade} from '../preferences/crossfade.ts'
	const [send, receive] = crossfade

</script>

<title>Pocket Dictionary</title>

<main out:send="{{key: 'a'}}" in:receive="{{key: 'a'}}" class="container" color-scheme={$colorScheme}>
	<nav>
		<ul>
		</ul>
		<ul>
			<li>
				{#if !$page.url.href.endsWith("/settings")}
					<a href="/settings">
						<Settings/>
					</a>
				{:else }
					<a href="/">
						<Home/>
					</a>
				{/if}
			</li>
		</ul>
	</nav>
	<slot />
</main>