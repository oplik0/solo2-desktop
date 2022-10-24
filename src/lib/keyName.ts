import { Store } from "tauri-plugin-store-api";
import type { Solo2List } from "../types";
const names = new Store("keynames.dat");
export async function loadKeyName(uuid: string): Promise<string> {
	return await names.get(uuid) ?? uuid;
}

export async function saveKeyName(uuid: string, name: string): Promise<void> {
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
