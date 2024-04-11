import { defineStore } from 'pinia'

export const useCache = defineStore('cache', {
  state: () => {
    return {}
  },
  actions: {
    async set(key: string, data: string): Promise<boolean> {
      return false
    },
    async get(key: string): Promise<{} | null> {
      return null
    }
  }
})
