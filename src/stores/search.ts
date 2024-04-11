import { defineStore } from 'pinia'

// works as a sort of cache / search store / db
//
// uses cache store for the caching functionality
export const useSearch = defineStore('search', {
  state: () => {
    return {}
  },
  getters: {
    history: async (): Promise<any[]> => {
      return []
    }
  },
  actions: {
    async query(q: string): Promise<{}> {
      return {}
    }
  }
})
