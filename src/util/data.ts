import { getClient, ResponseType } from "@tauri-apps/api/http"
import { parse } from "valibot"
import { ApiResponse } from "~/schemas/mangadex"
import { Manga, MangaFeed, MangaFeedSchema, MangaSchema } from "~/schemas/mangadex/manga"

async function get_path(path: string, refresh: boolean = false): Promise<any | null> {
    const client = await getClient()
    const response = await client.get<ApiResponse>(`https://api.mangadex.org/${path}`, {
        timeout: 15,
        responseType: ResponseType.JSON,
    })

    if (!response.ok)
        return null
    return response.data.data
}

export async function get_manga(id: string): Promise<Manga> {
    const res = await get_path(`manga/${id}`)
    const data = parse(MangaSchema, res)
    return data
}

export async function get_manga_feed(id: string, validate: boolean = true, refresh: boolean = false): Promise<MangaFeed> {
    // For some reason setting query parameters will 
    // make the data invalid according to valibot
    // const res1 = await get_path(`manga/${id}/feed?limit=500&translatedLanguage[]=en`, refresh)
    const res = await get_path(`manga/${id}/feed?limit=500&translatedLanguage[]=en`, refresh)
    if (validate) {
        const data = parse(MangaFeedSchema, res)
        return data
    }
    return res
}
