#!/usr/bin/env bun

// Creates/updates or removes the Portainer stack for a lila PR preview.
// Runs in Depot CI (.depot/workflows/preview.yml), after the images are built.
//
// Usage: deploy.ts up|down <pr-number>

import { appendFileSync } from "node:fs";

const PORTAINER_URL = "https://manage.lichess.app";
const ENDPOINT_ID = 4;
const IMAGE_REPO = "ft8xr42m62.registry.depot.dev/jxm5r03mgs";

interface Stack {
    Id: number;
    Name: string;
    EndpointId: number;
    Env?: { name: string; value: string }[];
}

const [action, pr] = Bun.argv.slice(2);
if ((action !== "up" && action !== "down") || !/^\d+$/.test(pr ?? "")) {
    console.error("Usage: deploy.ts up|down <pr-number>");
    process.exit(1);
}

const apiKey = process.env.PORTAINER_API_KEY;
if (!apiKey) {
    console.error("PORTAINER_API_KEY is not set");
    process.exit(1);
}

const tag = `pr-${pr}`;
const name = `lila-preview-${tag}`;

async function portainer<T>(method: string, path: string, body?: unknown): Promise<T> {
    const res = await fetch(`${PORTAINER_URL}/api${path}`, {
        method,
        headers: { "X-API-Key": apiKey!, "Content-Type": "application/json" },
        body: body === undefined ? undefined : JSON.stringify(body),
    });
    const text = await res.text();
    if (!res.ok) {
        throw new Error(`${method} ${path} -> ${res.status}: ${text}`);
    }
    return (text ? JSON.parse(text) : undefined) as T;
}

// Printed to the log, and written to the step summary so that
// preview.ts can fetch it with `depot ci summary`.
function report(lines: string[]): void {
    console.log(lines.join("\n"));
    const summaryPath = process.env.GITHUB_STEP_SUMMARY;
    if (summaryPath) {
        appendFileSync(summaryPath, ["```", ...lines, "```", ""].join("\n"));
    }
}

function randomPassword(): string {
    const pick = (chars: string) => chars[Math.floor(Math.random() * chars.length)];
    return Array.from({ length: 8 }, (_, i) => pick(i % 2 ? "aeiou" : "bcdfghjklmnprstvwxyz")).join("");
}

// Portainer's `filters` query param doesn't work on this instance, so filter client-side.
const stacks = await portainer<Stack[]>("GET", "/stacks");
const existing = stacks.find((s) => s.EndpointId === ENDPOINT_ID && s.Name.toLowerCase() === name);

if (action === "down") {
    if (existing) {
        await portainer("DELETE", `/stacks/${existing.Id}?endpointId=${ENDPOINT_ID}`);
        report([`Removed stack '${name}'`]);
    } else {
        report([`Stack '${name}' does not exist, nothing to remove`]);
    }
    process.exit(0);
}

// Keep the seed passwords of an existing stack so they don't change on redeploy.
const oldEnv: Record<string, string> = {};
if (existing) {
    const details = await portainer<Stack>("GET", `/stacks/${existing.Id}?endpointId=${ENDPOINT_ID}`);
    for (const { name, value } of details.Env ?? []) oldEnv[name] = value;
}

const userPassword = oldEnv.USER_SEED_PASSWORD ?? randomPassword();
const privilegedPassword = oldEnv.PRIVILEGED_SEED_PASSWORD ?? randomPassword();
const env = Object.entries({
    PREVIEW_NAMESPACE: name,
    SUBDOMAIN: tag,
    LILA_SITE_NAME: tag,
    LILA_SERVER_IMAGE: `${IMAGE_REPO}:${tag}-lila-server`,
    LILA_ASSETS_IMAGE: `${IMAGE_REPO}:${tag}-lila-assets`,
    USER_SEED_PASSWORD: userPassword,
    PRIVILEGED_SEED_PASSWORD: privilegedPassword,
}).map(([name, value]) => ({ name, value }));

if (existing) {
    console.log(`Redeploying stack '${name}' (id=${existing.Id})`);
    await portainer("PUT", `/stacks/${existing.Id}/git/redeploy?endpointId=${ENDPOINT_ID}`, {
        env,
        repositoryReferenceName: "refs/heads/main",
        repullImageAndRedeploy: true,
    });
} else {
    console.log(`Creating stack '${name}'`);
    const swarm = await portainer<{ ID: string }>("GET", `/endpoints/${ENDPOINT_ID}/docker/swarm`);
    await portainer("POST", `/stacks/create/swarm/repository?endpointId=${ENDPOINT_ID}`, {
        name,
        swarmID: swarm.ID,
        repositoryUrl: "https://github.com/lichess-org/lila-docker",
        repositoryReferenceName: "refs/heads/main",
        composeFile: "stacks/lila-preview/compose.yml",
        env,
        supportRelativePath: true,
        filesystemPath: "/mnt",
    });
}

const site = `https://preview.${tag}.lichess.app`;
report([
    `Site:                     ${site}`,
    `Source:                   https://github.com/lichess-org/lila/pull/${pr}`,
    `Test users:               ${site}/page/test-users`,
    `Regular user password:    ${userPassword}`,
    `Privileged user password: ${privilegedPassword}`,
    `Logs:                     ${site}/logs`,
    `Mailpit:                  ${site}/mailpit`,
    `MongoDB:                  ${site}/mongodb`,
    `Explorer:                 https://preview.${tag}.explorer.lichess.app/masters`,
    `Portainer management:     ${PORTAINER_URL}/#!/${ENDPOINT_ID}/docker/stacks/${name}?type=1&external=true`,
    `Tear down:                $ preview down ${pr}`,
]);
