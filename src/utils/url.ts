export const construct_url = (base: string, queryParams: [string, string][]): string => {
	const url = new URL(base)
	queryParams.forEach(([key, value]) => url.searchParams.append(key, value))
	return url.toString()
}
