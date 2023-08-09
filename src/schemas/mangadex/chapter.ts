import { any, enumType, number, object, optional, string, type Output, union } from "valibot";

export const ChapterSchema = object({
    id: string(),
    type: enumType(["chapter"]),
    attributes: object({
        title: optional(any()),
        volume: string(),
        chapter: string(),
        pages: number(),
        translatedLanguage: string(),
        // This field doesn't seem to exist in the data returned from MangaDex API
        // uploader: string(),
        // externalUrl: string(), // Throws an error
        version: number(),
        createdAt: string(),
        updatedAt: string(),
        publishAt: string(),
        readableAt: string()
    }),
    relationships: any(),
})
export type Chapter = Output<typeof ChapterSchema>
