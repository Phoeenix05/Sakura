import { any, array, boolean, enumType, number, object, record, string, type Output } from "valibot"
import { ChapterSchema } from "./chapter"

export const MangaSchema = object({
    id: string(),
    type: enumType(["manga"]),
    attributes: object({
        title: record(string()),
        // altTitle: array(object({ x: string() })),
        description: record(string()),
        isLocked: boolean(),
        links: record(string()),
        originalLanguage: string(),
        lastVolume: string(),
        lastChapter: string(),
        publicationDemographic: string(),
        status: enumType(["completed", "ongoing", "hiatus"]),
        year: number(),
        contentRating: string(),
        chapterNumbersResetOnNewVolume: boolean(),
        availableTranslatedLanguages: array(string()),
        latestUploadedChapter: string(),
        tags: array(any()),
        state: string(),
        version: number(),
        createdAt: string(),
        updatedAt: string(),
    }),
    relationships: any(),
})
export type Manga = Output<typeof MangaSchema>

export const MangaFeedSchema = array(ChapterSchema)
export type MangaFeed = Output<typeof MangaFeedSchema>
