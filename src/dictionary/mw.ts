const API_KEY = "<...>"

export async function getWord(word: string): Promise<any> {
    word = encodeURI(word);
    console.log(`https://www.dictionaryapi.com/api/v3/references/collegiate/json/${word}?key=${API_KEY}`)
    return fetch(`https://www.dictionaryapi.com/api/v3/references/collegiate/json/${word}?key=${API_KEY}`)
        .then((response) => response.json())
}