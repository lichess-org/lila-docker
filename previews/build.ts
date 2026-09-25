#!/usr/bin/env bun

import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { parseArgs } from "node:util";
import { runTagged } from "./lib/proc";

const REPO_OWNER = "lichess-org";
const REPO_NAME = "lila";

function usageError(message: string): never {
    console.error(message);
    console.error("Usage: build.ts --pr <github-pull-request-url> | --branch <branch-name> [--deploy]");
    console.error("Example: build.ts --pr https://github.com/lichess-org/lila/pull/21394");
    console.error("Example: build.ts --branch master --deploy");
    process.exit(1);
}

const { values } = parseArgs({
    args: Bun.argv.slice(2),
    options: {
        pr: { type: "string" },
        branch: { type: "string" },
        deploy: { type: "boolean" },
    },
    strict: true,
});

if (!values.pr && !values.branch) {
    usageError("Error: pass --pr or --branch");
}
if (values.pr && values.branch) {
    usageError("Error: pass either --pr or --branch, not both");
}

let fetchRef: string;
let tag: string;

if (values.pr) {
    const match = values.pr.match(/^https:\/\/github\.com\/([^/]+)\/([^/]+)\/pull\/(\d+)\/?$/);
    if (!match) {
        console.error(`Error: not a valid GitHub pull request URL: ${values.pr}`);
        process.exit(1);
    }
    const prNumber = match[3];
    fetchRef = `pull/${prNumber}/head`;
    tag = `pr-${prNumber}`;
    console.log(`Checking out PR #${prNumber} from ${REPO_OWNER}/${REPO_NAME}...`);
} else {
    const branch = values.branch as string;
    fetchRef = branch;
    tag = `branch-${branch.replace(/[^a-zA-Z0-9_.-]/g, "-")}`;
    console.log(`Checking out branch ${branch} from ${REPO_OWNER}/${REPO_NAME}...`);
}

const scriptDir = dirname(import.meta.path);
const lilaDir = join(scriptDir, "lila");

if (!existsSync(lilaDir)) {
    console.log(`Cloning ${REPO_OWNER}/${REPO_NAME} into ${lilaDir}...`);
    await Bun.$`git clone --depth 1 https://github.com/${REPO_OWNER}/${REPO_NAME}.git ${lilaDir}`;
}

await Bun.$`git -C ${lilaDir} fetch --depth 1 origin ${fetchRef}`;
await Bun.$`git -C ${lilaDir} -c advice.detachedHead=false checkout --detach FETCH_HEAD`;
await Bun.$`git -C ${lilaDir} clean -fdx`;

console.log("Building and publishing assets + server images with depot (in parallel)...");

await Promise.all([
    runTagged(
        "assets",
        [
            "depot",
            "build",
            "--platform",
            "linux/amd64",
            "--save",
            "--save-tag",
            `${tag}-lila-assets`,
            "-f",
            "docker/assets.Dockerfile",
            ".",
        ],
        scriptDir,
    ),
    runTagged(
        "server",
        [
            "depot",
            "build",
            "--platform",
            "linux/amd64",
            "--save",
            "--save-tag",
            `${tag}-lila-server`,
            "-f",
            "docker/server.Dockerfile",
            ".",
        ],
        scriptDir,
    ),
]);

console.log("Done.");

if (values.deploy) {
    console.log(`Deploying tag ${tag}...`);
    const sourceArgs = values.pr ? ["--pr", values.pr] : ["--branch", values.branch as string];
    await Bun.$`bun run ${join(scriptDir, "deploy.ts")} --tag ${tag} ${sourceArgs}`;
}
