export type ImageComponent = "server" | "assets";

function requireEnv(name: string): string {
    const value = process.env[name];
    if (!value) {
        throw new Error(`${name} environment variable is not set`);
    }
    return value;
}

export function imageFor(tag: string, component: ImageComponent): string {
    const registry = requireEnv("DEPOT_REGISTRY");
    const projectId = requireEnv("DEPOT_PROJECT_ID");
    return `${registry}/${projectId}:${tag}-lila-${component}`;
}
