import { SHA256 } from 'crypto-js'
/**
 * History items do not need the 'url' field like Manga items
 * as only chapters available on mangadex can be added to the history.
 */
type ChapterSchema = {
	hash: string
	id: string
}

export const useReadingHistory = defineStore('readingHistory', {
	state: (): { data: ChapterSchema[] } => {
		// TODO: Make this part prettier
		const data = localStorage.getItem('readingHistory')
		const json = JSON.parse(data ? data : '[]')
		return { data: json }
	},
	getters: {
		all: (state) => state.data,
		range: (state) => (from: number, to: number) => state.data.slice(from, to)
	},
	actions: {
		updateLocalStorage() {
			localStorage.setItem('readingHistory', JSON.stringify(this.data))
		},
		addItem(id: string) {
			const hash = SHA256(id).toString()

			// If item was already in the user's reading history
			// then remove it from the history and push it to the top of the list
			if (this.data.find((e) => e.hash === hash)) {
				const idx = this.data.findIndex((e) => e.hash === hash)
				this.data.splice(idx, 1)
			}

			this.data.unshift({ hash, id })
			this.updateLocalStorage()
		},
		removeItem(hash: string) {
			const idx = this.data.findIndex((e) => e.hash === hash)
			this.data.splice(idx, 1)
			this.updateLocalStorage()
		}
	}
})

export const useSearchHistory = defineStore('searchHistory', {
	state: (): { data: string[]; maxLength: number } => {
		// TODO: Make this part prettier
		const data = localStorage.getItem('searchHistory')
		const json = JSON.parse(data ? data : '[]')
		return { data: json, maxLength: 10 }
	},
	getters: {
		all: (state) => state.data
	},
	actions: {
		updateLocalStorage() {
			localStorage.setItem('searchHistory', JSON.stringify(this.data))
		},
		addItem(query: string) {
			if (this.data.includes(query)) {
				const idx = this.data.indexOf(query)
				this.data.splice(idx, 1)
			}
			this.data.unshift(query)

			if (this.data.length >= this.maxLength) {
				this.data.pop()
			}
			this.updateLocalStorage()
		}
	}
})
