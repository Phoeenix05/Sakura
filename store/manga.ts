import { defineStore } from 'pinia'

export const useMangaStore = defineStore('manga', {
    state: () => {
        return { manga_id: '' }
    },
    actions: {
        changeManga(id: string) {
            this.manga_id = id
        }
    }
})