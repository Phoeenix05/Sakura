import { any, enumType, number, object, optional, string, type Output } from "valibot";

export const ChapterSchema = object({
    id: string(),
    type: enumType(["chapter"]),
    attributes: object({
        // title: any(),
        // volume: string(),
        // chapter: string(),
        // pages: number(),
        // translatedLanguage: string(),
        // uploader: string(),
        // externalUrl: optional(string()),
        // version: number(),
        // createdAt: string(),
        // updatedAt: string(),
        // publishAt: string(),
        // readableAt: string()
    }),
    relationships: any(),
})
export type Chapter = Output<typeof ChapterSchema>
