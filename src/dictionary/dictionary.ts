import {invoke} from "@tauri-apps/api/tauri";

/**
 * Returns the HTML linked with the given entry.
 *
 * @param entry Entry to lookup
 */
export async function getHTML(entry: string): Promise<string> {
    return invoke("get_html", {entry})
}

/**
 * Reads the system clipboard. This will return an empty string
 * in cases of non-string clipboard, such as images.
 */
export async function getClipboard(): Promise<string> {
    return invoke("read_clipboard", {})
}
