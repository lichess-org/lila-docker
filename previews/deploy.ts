#!/usr/bin/env bun

import { parseArgs } from "node:util";
import { imageFor } from "./lib/depot";
import {
    createStack,
    deleteStack,
    findStack,
    getStack,
    getSwarmId,
    redeployStack,
    type StackEnv,
} from "./lib/portainer";

const REPO_OWNER = "lichess-org";
const REPO_NAME = "lila";

function stackNameFor(tag: string): string {
    return `lila-preview-${tag}`;
}

function printAccessInfo(
    portainerUrl: string,
    endpointId: number,
    name: string,
    tag: string,
    subdomain: string,
    userSeedPassword: string,
    privilegedSeedPassword: string,
    sourceLink: string | undefined,
): void {
    const site = `https://preview.${subdomain}.lichess.app`;
    const stackUrl = `${portainerUrl}/#!/${endpointId}/docker/stacks/${name}?type=1&external=true`;
    console.log("");
    console.log("#".repeat(80));
    console.log(`Site:                     ${site}`);
    if (sourceLink) {
        console.log(`Source:                   ${sourceLink}`);
    }
    console.log(`Test users:               ${site}/page/test-users`);
    console.log(`Regular user password:    ${userSeedPassword}`);
    console.log(`Privileged user password: ${privilegedSeedPassword}`);
    console.log(`Logs:                     ${site}/logs`);
    console.log(`Mailpit:                  ${site}/mailpit`);
    console.log(`MongoDB:                  ${site}/mongodb`);
    console.log(`Explorer:                 https://preview.${subdomain}.explorer.lichess.app/masters`);
    console.log(`Portainer management:     ${stackUrl}`);
    console.log(`Tear down:                $ preview down ${tag}`);
}

function randomPassword(): string {
    const vowels = "aeiou";
    const consonants = "bcdfghjklmnprstvwxyz";

    function randomWord(length: number): string {
        let word = "";
        for (let i = 0; i < length; i++) {
            if (i % 2 === 0) {
                word += consonants[Math.floor(Math.random() * consonants.length)];
            } else {
                word += vowels[Math.floor(Math.random() * vowels.length)];
            }
        }
        return word;
    }

    const word1 = randomWord(4);
    const word2 = randomWord(4);
    return word1 + word2;
}

interface DeployOverrides {
    subdomain?: string;
    siteName?: string;
    serverImage?: string;
    assetsImage?: string;
    userSeedPassword?: string;
}

async function deployPreview(
    baseUrl: string,
    apiKey: string,
    endpointId: number,
    tag: string,
    dryRun: boolean,
    overrides: DeployOverrides,
    sourceLink: string | undefined,
): Promise<void> {
    const name = stackNameFor(tag);
    const subdomain = overrides.subdomain ?? tag;

    const existing = await findStack(baseUrl, apiKey, endpointId, name);

    let userSeedPassword: string;
    let privilegedSeedPassword: string;
    if (existing) {
        const details = await getStack(baseUrl, apiKey, endpointId, existing.Id);
        const existingEnv = Object.fromEntries((details.Env ?? []).map((e) => [e.name, e.value]));
        userSeedPassword = overrides.userSeedPassword ?? existingEnv.USER_SEED_PASSWORD ?? randomPassword();
        privilegedSeedPassword = existingEnv.PRIVILEGED_SEED_PASSWORD ?? randomPassword();
    } else {
        userSeedPassword = overrides.userSeedPassword ?? randomPassword();
        privilegedSeedPassword = overrides.userSeedPassword ?? randomPassword();
    }

    const stackEnv: StackEnv = {
        PREVIEW_NAMESPACE: name,
        SUBDOMAIN: subdomain,
        LILA_SITE_NAME: overrides.siteName ?? subdomain,
        LILA_SERVER_IMAGE: overrides.serverImage ?? imageFor(tag, "server"),
        LILA_ASSETS_IMAGE: overrides.assetsImage ?? imageFor(tag, "assets"),
        USER_SEED_PASSWORD: userSeedPassword,
        PRIVILEGED_SEED_PASSWORD: privilegedSeedPassword,
    };

    if (existing) {
        console.log(`Stack '${name}' already exists (id=${existing.Id})`);
        if (dryRun) {
            console.log(
                `[dry-run] would PUT /stacks/${existing.Id}/git/redeploy with env=${JSON.stringify(stackEnv)}`,
            );
        } else {
            console.log("Updating and redeploying");
            await redeployStack(baseUrl, apiKey, existing.Id, endpointId, stackEnv);
            printAccessInfo(
                baseUrl,
                endpointId,
                name,
                tag,
                subdomain,
                userSeedPassword,
                privilegedSeedPassword,
                sourceLink,
            );
        }
    } else {
        console.log(`Stack '${name}' does not yet exist`);
        if (dryRun) {
            console.log(
                `[dry-run] would POST /stacks/create/swarm/repository with name='${name}' env=${JSON.stringify(stackEnv)}`,
            );
        } else {
            console.log("Creating new stack...");
            const swarmId = await getSwarmId(baseUrl, apiKey, endpointId);
            await createStack(baseUrl, apiKey, endpointId, swarmId, name, stackEnv);
            printAccessInfo(
                baseUrl,
                endpointId,
                name,
                tag,
                subdomain,
                userSeedPassword,
                privilegedSeedPassword,
                sourceLink,
            );
        }
    }
}

async function removePreview(
    baseUrl: string,
    apiKey: string,
    endpointId: number,
    tag: string,
    dryRun: boolean,
): Promise<void> {
    const name = stackNameFor(tag);
    const existing = await findStack(baseUrl, apiKey, endpointId, name);
    if (!existing) {
        console.log(`Stack '${name}' does not exist, nothing to remove`);
        return;
    }

    if (dryRun) {
        console.log(`[dry-run] would DELETE /stacks/${existing.Id}`);
    } else {
        console.log(`Removing stack '${name}' (id=${existing.Id})`);
        await deleteStack(baseUrl, apiKey, existing.Id, endpointId);
    }
}

const { values } = parseArgs({
    args: Bun.argv.slice(2),
    options: {
        tag: { type: "string" },
        "portainer-url": { type: "string" },
        "api-key": { type: "string" },
        "endpoint-id": { type: "string" },
        subdomain: { type: "string" },
        "site-name": { type: "string" },
        "server-image": { type: "string" },
        "assets-image": { type: "string" },
        "user-seed-password": { type: "string" },
        pr: { type: "string" },
        branch: { type: "string" },
        remove: { type: "boolean" },
        "dry-run": { type: "boolean" },
    },
    strict: true,
});

if (!values.tag) {
    console.error("Error: --tag is required (e.g. pr-1234, branch-master)");
    process.exit(1);
}

if (values.pr && values.branch) {
    console.error("Error: pass either --pr or --branch, not both");
    process.exit(1);
}

let sourceLink: string | undefined;
if (values.pr) {
    if (!/^https:\/\/github\.com\/[^/]+\/[^/]+\/pull\/\d+\/?$/.test(values.pr)) {
        console.error(`Error: not a valid GitHub pull request URL: ${values.pr}`);
        process.exit(1);
    }
    sourceLink = values.pr;
} else if (values.branch) {
    sourceLink = `https://github.com/${REPO_OWNER}/${REPO_NAME}/tree/${values.branch}`;
}

const portainerUrl = values["portainer-url"] ?? process.env.PORTAINER_URL;
const apiKey = values["api-key"] ?? process.env.PORTAINER_API_KEY;
const endpointId = Number(values["endpoint-id"] ?? process.env.PORTAINER_ENDPOINT_ID ?? "4");

if (!portainerUrl || !apiKey) {
    console.error(
        "Error: --portainer-url and --api-key must be set (directly or via PORTAINER_URL, PORTAINER_API_KEY)",
    );
    process.exit(1);
}

const dryRun = values["dry-run"] ?? false;

if (values.remove) {
    await removePreview(portainerUrl, apiKey, endpointId, values.tag, dryRun);
} else {
    await deployPreview(
        portainerUrl,
        apiKey,
        endpointId,
        values.tag,
        dryRun,
        {
            subdomain: values.subdomain,
            siteName: values["site-name"],
            serverImage: values["server-image"],
            assetsImage: values["assets-image"],
            userSeedPassword: values["user-seed-password"],
        },
        sourceLink,
    );
}
