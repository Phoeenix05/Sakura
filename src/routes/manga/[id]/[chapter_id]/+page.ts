// import { redirect } from "@sveltejs/kit";
import type { RouteParams } from "./$types";
type fetch_type = typeof fetch

/** @type {import('./$types').PageLoad} */
export async function load({ fetch, params }: { fetch: fetch_type, params: RouteParams }) {  
  const { id, chapter_id } = params

  const chapterData = await fetch(`https://api.mangadex.org/at-home/server/${chapter_id}`, {
    cache: "force-cache"
  }).then(res => res.json())

  return {
    baseUrl: chapterData.baseUrl,
    chapterHash: chapterData.chapter.hash,
    imageData: chapterData.chapter.data,
    id,
    chapterId: chapter_id
  }
}