import { getClient, ResponseType } from "@tauri-apps/api/http"
import { parse } from "valibot"
import { ApiResponse } from "~/schemas/mangadex"
import { Manga, MangaFeed, MangaFeedSchema, MangaSchema } from "~/schemas/mangadex/manga"

async function get_path(path: string): Promise<any | null> {
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

export async function get_manga_feed(id: string): Promise<MangaFeed> {
    const res = await get_path(`manga/${id}/feed`)
    console.log(res)
    const data = parse(MangaFeedSchema, res)
    return res
}
