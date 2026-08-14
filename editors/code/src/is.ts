// biome-ignore lint/suspicious/noExplicitAny: todo
export function string(value: any): value is string {
	return typeof value === "string" || value instanceof String;
}

// biome-ignore lint/suspicious/noExplicitAny: todo
export function array<T>(value: any): value is T[] {
	return Array.isArray(value);
}

// biome-ignore lint/suspicious/noExplicitAny: todo
export function typedArray<T>(value: any, check: (value: any) => boolean): value is T[] {
	// biome-ignore lint/suspicious/noExplicitAny: todo
	return Array.isArray(value) && (<any[]>value).every(check);
}
