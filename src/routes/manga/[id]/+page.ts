import type { RouteParams } from "./$types";
type fetch_type = typeof fetch

/** @type {import('./$types').PageLoad} */
export async function load({ fetch, params }: { fetch: fetch_type, params: RouteParams }) {
  const { id } = params

  const mangaData = await fetch(`https://api.mangadex.org/manga/${id}/feed?translatedLanguage[]=en&limit=500`, {
    cache: "force-cache"
  }).then(res => res.json())

  return {
    mangaData: mangaData.data.sort((a: any, b: any) => a.attributes.chapter - b.attributes.chapter),
    id
  }
}