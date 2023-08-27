const useFollowStore = defineStore('followStore', {
	state: () => {
		const data: string[] = JSON.parse(localStorage.getItem('followed') || '[]')
		return { data: data }
	},
	getters: {
		ids: (state): string[] => state.data,
		idsQueryString: (state): [string, string][] => {
			const res: [string, string][] = []
			state.data.forEach((id) => res.push(['ids[]', id]))
			return res
		}
	},
	actions: {
		add(id: string) {
			if (this.data.includes(id)) {
				return
			}
			this.data.push(id)
			localStorage.setItem('followed', JSON.stringify(this.data))
		},
		remove(id: string) {
			if (!this.data.includes(id)) {
				return
			}
			const idx = this.data.findIndex((v) => v == id)
			this.data.splice(idx, 1)
			localStorage.setItem('followed', JSON.stringify(this.data))
		}
	}
})
export default useFollowStore
