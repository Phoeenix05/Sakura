export type StoreSchema = {
    db: IDBDatabase | null,
    /** 
     * Indicates if the database is working 
     * @default true
     */
    status: boolean,
    md_ids: string[],
}

export const useFollowedMangasStore = defineStore('followedMangas', {
    state: (): StoreSchema => {
        return { db: null, status: true, md_ids: [] };
    },
    getters: {
        /** Get all followed mangas. (mainly used on the 'following' page) */
        all: (state) => { },
        /** Get followed manga by given ID */
        getById: (state) => (id: string) => { }
    },
    actions: {
        /** Sets up db callbacks such as 'onerror' */
        setupDbCallbacks() {
            if (this.db == null) {
                return
            }

            this.db.onerror = (event) => {
                console.log(`IndexedDB error`)
            }
        },

        /** Add manga from the store (IndexedDB) */
        addManga(id: string) {
            if (this.db == null) {
                return
            }

            if (!this.md_ids.includes(id)) {
                this.md_ids.push(id)
            }

            const objectStore = this.db.createObjectStore('mangas', { keyPath: 'ssn' })
            objectStore.createIndex('md_id', 'md_id', { unique: true })

            objectStore.transaction.oncomplete = (event) => {
                const mangaObjectStore = this.db?.transaction('mangas', 'readwrite').objectStore('mangas')
                this.md_ids.forEach((e) => {
                    mangaObjectStore?.add({ md_id: e })
                })
            }
        },
        /** Remove manga from the store (IndexedDB) */
        removeManga(id: string) { }
    }
})

const initIndexedDb = () => {
    const store = useFollowedMangasStore()
    const request = window.indexedDB.open('followedMangas', 3)

    request.onerror = (e) => {
        console.log('Failed to create database')
        store.$patch({ db: null, status: false })
    }
    request.onsuccess = (e) => {
        console.log('Successfully created database')
        store.$patch({ db: request.result })
        store.setupDbCallbacks()
    }
}
initIndexedDb()
