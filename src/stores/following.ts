import SHA256 from "crypto-js/sha256"

/**
 * The way this is made is that `id` is present if the manga was found be searching it
 * in this app (so the manga has to be found on Mangadex for `id` to be present).
 * 
 * `url` is present when you create a manga entry manually.
 * 
 * `hash` field is used as the identifier for the DOM element. Mainly for easier deletion
 * from the `following` list.
 * 
 * **Note**: If you use bot `id` and `url` for creating a manga entry, well the deletion
 * should still work as expected.
 */
type MangaSchema = {
    hash: string,
    id: string,
    url: string,
}

export const useFollowingStore = defineStore('followed', {
    state: (): { data: MangaSchema[] } => {
        // TODO: Make this part prettier
        const data = localStorage.getItem('followed')
        const json = JSON.parse(data ? data : "[]")
        return { data: json }
    },
    getters: {
        all: (state) => state.data,
        range: (state) => (from: number, to: number) => state.data.slice(from, to),
    },
    actions: {
        /**
         * This function can be called anytime time to save the data to `localStorage`.
         * Though this function is called automatically by `addManga` and `removeManga` methods.
         */
        updateLocalStorage() {
            localStorage.setItem('followed', JSON.stringify(this.data))
        },
        addManga(id: string, url: string) {
            const hash = SHA256(id + url).toString()
            if (this.data.find((e) => e.hash === hash)) return

            this.data.unshift({ hash, id, url })
            this.updateLocalStorage()
        },
        removeManga(hash: string) {
            const idx = this.data.findIndex((e) => e.hash === hash)
            this.data.splice(idx, 1)
            this.updateLocalStorage()
        }
    }
})
