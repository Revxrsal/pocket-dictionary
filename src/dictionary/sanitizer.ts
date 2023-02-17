import {lookupEntry} from "./dictionary";

/**
 * Cleans up punctuation marks from a string
 *
 * @param str String to remove from
 */
function cleanUpPunctuations(str: string): string {
    return str
        .replace(".", "")
        .replace(",", "")
        .replace("?", "")
        .replace(";", "")
        .replace("!", "")
        .replace("'", "")
        .replace("\"", "")
        .trim()
        .toLowerCase()
}

/**
 * Reads the entry name from the URL
 *
 * @param url URL to retrieve from
 */
export function getEntryFromURL(url: string): string {
    const indexOf = url.indexOf('#')
    if ((indexOf === -1) || (indexOf === url.length - 1))
        return "";
    return decodeURIComponent(url.substring(url.indexOf('#') + 1))
}

/**
 * Looks up the given word. This will perform sanitizations to the word
 * to try its best to find it
 *
 * @param word       Word to lookup
 * @param setQueryTo An optional function to
 */
export async function lookup(word: string, setQueryTo?: (word: string) => void): Promise<string> {
    if (word === "")
        return ""

    word = cleanUpPunctuations(word)
    setQueryTo?.(word)

    let result = await lookupEntry(word)
    return result || ""
}