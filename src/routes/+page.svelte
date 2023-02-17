<!--
Due to Svelte's oddness and my inexperience in routing, this
is how we're handling hrefs which contain links to other words:
 1. Each href links to the same page using the `#`, e.g: #deceive
    will be the href of the deceive word.
 2. The value listener below simply listens to $page changes, and detects
    changes in the URL. It then strips the # prefix and decodes the URL
    value to be appropriately looked up.
 3. We set both query and clickSearch, so that when a page opens on a certain word
    it is immediately looked up.
 4. This probably isn't the cleanest way of doing stuff, but what can you
    expect from a 3-days Sveltian frantically sweeping through the docs
    for the least bit of guidance? x)
-->
<script>
	import DictionaryPage from "$lib/DictionaryPage.svelte";
	import {page} from "$app/stores";
	import {getEntryFromURL} from "../dictionary/sanitizer.ts";
	
	$: value = () => {
		const url = $page.url.href;
		return getEntryFromURL(url)
	}
</script>

<DictionaryPage query={value()} clickSearch={value()}/>