const REPOSITORY_URL = "https://github.com/lichess-org/lila-docker";
const REPOSITORY_REFERENCE = "refs/heads/main";
const COMPOSE_FILE_PATH = "stacks/lila-preview/compose.yml";
const RELATIVE_PATH_MOUNT = "/mnt";

export interface PortainerStack {
    Id: number;
    Name: string;
    EndpointId: number;
}

export interface PortainerStackDetails extends PortainerStack {
    Env?: { name: string; value: string }[];
}

interface SwarmInfo {
    ID: string;
}

export type StackEnv = Record<string, string>;

type HttpMethod = "GET" | "POST" | "PUT" | "DELETE";

async function portainerRequest<T>(
    baseUrl: string,
    apiKey: string,
    method: HttpMethod,
    path: string,
    opts: { query?: Record<string, string | number>; body?: unknown } = {},
): Promise<T> {
    const query = opts.query ?? {};
    const qs =
        Object.keys(query).length > 0
            ? `?${new URLSearchParams(query as Record<string, string>).toString()}`
            : "";
    const url = `${baseUrl.replace(/\/+$/, "")}/api${path}${qs}`;

    const res = await fetch(url, {
        method,
        headers: {
            "X-API-Key": apiKey,
            ...(opts.body !== undefined ? { "Content-Type": "application/json" } : {}),
        },
        body: opts.body !== undefined ? JSON.stringify(opts.body) : undefined,
    });

    const text = await res.text();
    if (!res.ok) {
        console.error(`${method} ${url} -> ${res.status}: ${text}`);
        process.exit(1);
    }

    return text.length === 0 ? ({} as T) : (JSON.parse(text) as T);
}

function envToList(env: StackEnv): { name: string; value: string }[] {
    return Object.entries(env).map(([name, value]) => ({ name, value }));
}

export async function getSwarmId(baseUrl: string, apiKey: string, endpointId: number): Promise<string> {
    const swarm = await portainerRequest<SwarmInfo>(
        baseUrl,
        apiKey,
        "GET",
        `/endpoints/${endpointId}/docker/swarm`,
    );
    return swarm.ID;
}

export async function findStack(
    baseUrl: string,
    apiKey: string,
    endpointId: number,
    name: string,
): Promise<PortainerStack | undefined> {
    // Portainer's `filters={"EndpointID": ...}` query param does not
    // actually filter anything on this instance, so filter client-side.
    const stacks = await portainerRequest<PortainerStack[]>(baseUrl, apiKey, "GET", "/stacks");
    return stacks.find(
        (s) => s.EndpointId === endpointId && s.Name.toLowerCase() === name.toLowerCase(),
    );
}

export async function getStack(
    baseUrl: string,
    apiKey: string,
    endpointId: number,
    stackId: number,
): Promise<PortainerStackDetails> {
    return portainerRequest<PortainerStackDetails>(baseUrl, apiKey, "GET", `/stacks/${stackId}`, {
        query: { endpointId },
    });
}

export async function createStack(
    baseUrl: string,
    apiKey: string,
    endpointId: number,
    swarmId: string,
    name: string,
    stackEnv: StackEnv,
): Promise<PortainerStack> {
    const payload = {
        name,
        swarmID: swarmId,
        repositoryUrl: REPOSITORY_URL,
        repositoryReferenceName: REPOSITORY_REFERENCE,
        composeFile: COMPOSE_FILE_PATH,
        env: envToList(stackEnv),
        supportRelativePath: true,
        filesystemPath: RELATIVE_PATH_MOUNT,
    };
    return portainerRequest<PortainerStack>(baseUrl, apiKey, "POST", "/stacks/create/swarm/repository", {
        query: { endpointId },
        body: payload,
    });
}

export async function redeployStack(
    baseUrl: string,
    apiKey: string,
    stackId: number,
    endpointId: number,
    stackEnv: StackEnv,
): Promise<void> {
    const payload = {
        env: envToList(stackEnv),
        repositoryReferenceName: REPOSITORY_REFERENCE,
        repullImageAndRedeploy: true,
    };
    await portainerRequest(baseUrl, apiKey, "PUT", `/stacks/${stackId}/git/redeploy`, {
        query: { endpointId },
        body: payload,
    });
}

export async function deleteStack(
    baseUrl: string,
    apiKey: string,
    stackId: number,
    endpointId: number,
): Promise<void> {
    await portainerRequest(baseUrl, apiKey, "DELETE", `/stacks/${stackId}`, { query: { endpointId } });
}
