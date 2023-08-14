export const useFollowedMangasStore = defineStore('followedMangas', {
    state: (): { mangas: string[] } => {
        const data: string | null = localStorage.getItem('followedMangas')
        const mangas = JSON.parse(data ? data : "[]")
        return { mangas: mangas }
    },
    getters: {
        all: (state) => state.mangas,
        range: (state) => (from: number, to: number) => state.mangas.slice(from, to),
    },
    actions: {
        addManga(id: string) {
            if (this.mangas.includes(id)) {
                return
            }
            this.mangas.push(id)
            localStorage.setItem('followedMangas', JSON.stringify(this.mangas))
        },
        removeManga(id: string) {
            if (!this.mangas.includes(id)) {
                return
            }
            const idx = this.mangas.indexOf(id)
            this.mangas.splice(idx, 1)
            localStorage.setItem('followedMangas', JSON.stringify(this.mangas))
        }
    }
})
