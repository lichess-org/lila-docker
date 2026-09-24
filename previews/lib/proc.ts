// Runs a command with its stdout/stderr streamed live, each line prefixed
// with `label`. Used to interleave output from commands run in parallel
// (e.g. `Promise.all`) without garbling it.
export async function runTagged(label: string, cmd: string[], cwd: string): Promise<void> {
    const proc = Bun.spawn(cmd, { cwd, stdout: "pipe", stderr: "pipe" });

    const pump = async (stream: ReadableStream<Uint8Array>, write: (s: string) => void) => {
        const reader = stream.getReader();
        const decoder = new TextDecoder();
        let buffer = "";
        while (true) {
            const { done, value } = await reader.read();
            if (done) break;
            buffer += decoder.decode(value, { stream: true });
            const lines = buffer.split("\n");
            buffer = lines.pop() ?? "";
            for (const line of lines) write(`[${label}] ${line}\n`);
        }
        if (buffer.length > 0) write(`[${label}] ${buffer}\n`);
    };

    await Promise.all([
        pump(proc.stdout, (s) => process.stdout.write(s)),
        pump(proc.stderr, (s) => process.stderr.write(s)),
    ]);

    const exitCode = await proc.exited;
    if (exitCode !== 0) {
        throw new Error(`${cmd.join(" ")} exited with code ${exitCode}`);
    }
}
