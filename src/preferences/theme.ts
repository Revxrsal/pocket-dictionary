import {writable} from "svelte/store"

/**
 * The dark theme switch
 */
const theme = writable<string>(localStorage.getItem("theme") || "dark")

/**
 * The selected color scheme
 */
const colorScheme = writable<string>(localStorage.getItem("colorScheme") || "blue")

/**
 * Toggles the theme
 */
export function toggleTheme() {
    theme.update(v => v == "dark" ? "light" : "dark")
}

/**
 * Sets a new color scheme
 *
 * @param scheme New scheme to set
 */
export function setColorScheme(scheme: string) {
    colorScheme.set(scheme)
    localStorage.setItem("colorScheme", scheme)
}

/**
 * Handles updating the <html> tag. We yet have to find a better way.
 */
theme.subscribe(v => {
    document.querySelector('html')?.setAttribute('data-theme', v);
    localStorage.setItem("theme", v)
})

export {theme, colorScheme}