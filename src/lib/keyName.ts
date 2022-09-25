import type { Solo2, Solo2List } from "src/types";

// TODO: use tauri store plugin once I have wifi
const names: Record<string, string> = {};

export async function loadKeyName(uuid: string): Promise<string> {
	return names[uuid];
}

export async function saveKeyName(uuid: string, name: string): Promise<void> {
	names[uuid] = name;
}

export async function addNamesToKeys(keys: Solo2List) {
	return Object.fromEntries(
		await Promise.all(
			Object.entries(keys).map(async (entry) => {
				entry[1].name = entry[1].name ?? await loadKeyName(entry[0]);
				return entry;
			}),
		),
	);
}
