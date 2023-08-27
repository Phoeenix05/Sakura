import { any, array, boolean, map, number, object, optional, string } from 'valibot'

export const LocaleSchema = object({
	en: optional(string()),
	'ja-ro': optional(string())
})

export const MangaDataSchema = object({
	id: string(),
	type: string(),
	attributes: object({
		title: LocaleSchema,
		altTitles: optional(LocaleSchema),
		description: LocaleSchema,
		isLocked: boolean(),
		links: map(string(), string()),
		originalLanguage: string(),
		lastVolume: optional(string()),
		lastChapter: optional(string()),
		publicationDemographic: string(),
		status: string(),
		year: number(),
		contentRating: string(),
		tags: array(any()), // TODO: Add tag schema
		state: string(),
		chapterNumbersResetOnNewVolume: boolean(),
		createdAt: string(),
		updatedAt: string(),
		version: number(),
		availableTranslatedLanguages: array(string()),
		latestUploadedChapter: string()
	}),
	relationships: any()
})

export const MangaListSchema = object({
	result: string(),
	response: string(),
	data: array(MangaDataSchema),
	limit: number(),
	offset: number(),
	total: number()
})
