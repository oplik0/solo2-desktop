window.__TAURI_ISOLATION_HOOK__ = (payload) => {
	console.log(payload);
	return payload;
};
