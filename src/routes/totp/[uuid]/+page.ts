import type { Load } from "@sveltejs/kit";

export const prerender = true;
export const csr = true;

export const load: Load = ({ params }) => {
	return {
		uuid: params.uuid,
	};
};
