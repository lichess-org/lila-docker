#!/usr/bin/env bun

// Dispatches the Depot CI preview workflow (.depot/workflows/preview.yml),
// waits for it to finish, and prints the result written by deploy.ts.
//
// Usage: preview.ts up|down <lila PR number or URL>

interface Run {
    status: "queued" | "running" | "finished" | "failed" | "cancelled";
    workflows: { jobs: { job_id: string; job_key: string }[] }[];
}

const [action, prArg] = Bun.argv.slice(2);
const pr = prArg?.match(/^(?:https:\/\/github\.com\/lichess-org\/lila\/pull\/)?(\d+)\/?$/)?.[1];
if ((action !== "up" && action !== "down") || !pr) {
    console.error("Usage: preview up|down <lila PR number or URL>");
    process.exit(1);
}

const dispatched =
    await Bun.$`depot ci dispatch --repo lichess-org/lila-docker --workflow preview.yml --ref main --input action=${action} --input pr=${pr}`.text();
process.stdout.write(dispatched);

const runId = dispatched.match(/run (\S+) queued/)?.[1];
if (!runId) {
    throw new Error("could not find the run id in `depot ci dispatch` output");
}

let run: Run;
let lastStatus = "";
while (true) {
    run = await Bun.$`depot ci status ${runId} --output json`.json();
    if (run.status !== lastStatus) {
        console.log(`${new Date().toLocaleTimeString()} ${run.status}`);
        lastStatus = run.status;
    }
    if (run.status !== "queued" && run.status !== "running") break;
    await Bun.sleep(5000);
}

if (run.status !== "finished") {
    console.error("See the link above for logs.");
    process.exit(1);
}

const deployJob = run.workflows.flatMap((w) => w.jobs).find((j) => j.job_key.endsWith(":deploy"));
const summary = await Bun.$`depot ci summary ${deployJob!.job_id}`.text();
console.log("");
console.log(
    summary
        .split("\n")
        .filter((line) => !line.startsWith("```"))
        .join("\n")
        .trim(),
);
