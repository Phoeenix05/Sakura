import { defineStore } from 'pinia'

// works as a sort of cache / search store / db
//
// also works only for mangadex.org, for now
export const useSearch = defineStore('search', {
  state: () => {
    return {}
  },
  getters: {
    history: async (): Promise<{}> => {
      return {}
    }
  },
  actions: {
    async query(): Promise<{}> {
      return {}
    }
  }
})
