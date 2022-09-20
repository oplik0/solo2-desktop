import * as core from "@actions/core";

const releases = JSON.parse(process.argv[2]);
core.setOutput(
	"release_id",
	releases.reduce((acc, release) => release.id ? release.id : acc, false),
);
