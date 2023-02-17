import {getHTML} from "./dictionary";

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

export function getEntryFromURL(url: string): string {
    const indexOf = url.indexOf('#')
    if ((indexOf === -1) || (indexOf === url.length - 1))
        return "";
    return decodeURIComponent(url.substring(url.indexOf('#') + 1))
}

/**
 * Strips a suffix from the word
 *
 * @param word   Word to remove from
 * @param suffix Suffix to strip
 */
export function stripSuffix(word: string, suffix: string): string {
    if (word.endsWith(suffix))
        return word.substring(0, word.length - suffix.length)
    return word
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

    let result = await getHTML(word)

    if (result)
        return result

    /* Try replacing '*ies' with '*y' */
    if (word.endsWith("ies"))
        word = stripSuffix(word, "ies") + "y"
    result = await getHTML(word)
    if (result)
        return result

    /* Try removing 'ly' */
    word = stripSuffix(word, "ly")
    result = await getHTML(word)
    if (result)
        return result

    /* Try removing 'ed' */
    word = stripSuffix(word, "ed")
    result = await getHTML(word)
    if (result)
        return result

    /* Try removing 'ing' */
    word = stripSuffix(word, "ing")
    result = await getHTML(word)
    if (result)
        return result

    /* Try removing 'al' */
    word = stripSuffix(word, "al")
    result = await getHTML(word)
    if (result)
        return result

    /* Try removing plural 's' */
    word = stripSuffix(word, "s")
    result = await getHTML(word)
    if (result)
        return result

    /* Try adding an 'e' */
    result = await getHTML(word + "e")
    if (result)
        return result

    /* Try removing the last character (in case of words like 'admitting') */
    word = word.substring(0, word.length - 1)
    result = await getHTML(word)
    if (result)
        return result

    /* Give up */
    return ""
}