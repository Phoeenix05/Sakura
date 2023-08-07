import { any, array, enumType, number, object, optional, string, type Output } from "valibot"

export const ApiErrorSchema = object({
    result: enumType(["error"]),
    errors: array(object({
        id: string(),
        status: number(),
        title: string(),
        detail: string(),
        context: string(),
    }))
})
export type ApiError = Output<typeof ApiErrorSchema>

export const ApiResponseSchema = object({
    result: enumType(["ok"]),
    type: enumType(["entity", "collection"]),
    data: any(),
    // The following three fields are only present if type is "collection"
    limit: optional(number()),
    offset: optional(number()),
    total: optional(number()),
})
export type ApiResponse = Output<typeof ApiResponseSchema>
