export type ApiResponse<T> = {
    result: string
    response: string
    data: T
}

export const fetch_resource = async <T>(url: string): Promise<ApiResponse<T>> => {
    const data = await (await fetch(url)).json()
    console.log(data)
    return data
}
