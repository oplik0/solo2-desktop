import type { Solo2List } from "src/types";
import type { Store } from "tauri-plugin-store-api";
let names: Store;
export async function loadKeyName(uuid: string): Promise<string> {
	names = names ?? new (await import("tauri-plugin-store-api")).Store("keynames.dat");
	return await names.get(uuid) ?? uuid;
}

export async function saveKeyName(uuid: string, name: string): Promise<void> {
	names = names ?? new (await import("tauri-plugin-store-api")).Store("keynames.dat");
	await names.set(uuid, name);
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
