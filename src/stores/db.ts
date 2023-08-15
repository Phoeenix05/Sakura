import Dexie, { Table } from "dexie";

class MangaStore extends Dexie {
    followed!: Table<{ md_id: string, external_url: string }>

    constructor() {
        super('mangaStore')
        this.version(1).stores({
            followed: '++id, md_id, external_url'
        })
    }
}

export const db = new MangaStore()