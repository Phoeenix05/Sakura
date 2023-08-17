import { invoke } from "@tauri-apps/api"

export const userAgent = async (): Promise<string> => {
    return await invoke<string>('user_agent')
}

// NOTE: Do not remove ——————————————————
localStorage.setItem('User-Agent', await userAgent())
// ——————————————————————————————————————
