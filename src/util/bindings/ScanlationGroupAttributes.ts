// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Language } from "./Language";
import type { MangaDexDateTime } from "./MangaDexDateTime";
import type { MangaDexDuration } from "./MangaDexDuration";

export interface ScanlationGroupAttributes { name: string, altNames: Array<Record>, website: string | null, ircServer: string | null, ircChannel: string | null, discord: string | null, contactEmail: string | null, description: string | null, twitter: string, mangaUpdates: string, focusedLanguages: Array<Language> | null, locked: boolean, official: boolean, verified: boolean, inactive: boolean, exLicensed?: boolean, publishDelay: MangaDexDuration | null, version: number, createdAt: MangaDexDateTime, updatedAt: MangaDexDateTime, }