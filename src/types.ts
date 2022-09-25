export interface Solo2 {
	name?: string;
	uuid: string;
	version: string;
	secure?: boolean;
}
export interface Solo2List {
	[uuid: string]: Solo2;
}
