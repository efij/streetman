import { spawnSync } from "node:child_process";

function runStreetman(args) {
  const bin = process.env.STREETMAN_BIN || "streetman";
  return spawnSync(bin, args, { encoding: "utf8" });
}

export function parseStreetmanLeanCommand(text) {
  const value = String(text || "full").trim().toLowerCase();
  return ["off", "lite", "full", "ultra"].includes(value) ? value : "full";
}

export default function streetmanExtension(pi) {
  let mode = process.env.STREETMAN_LEAN_DEFAULT || "full";

  pi.registerCommand("streetman-lean", {
    description: "Set Streetman Lean mode",
    handler: async (args, ctx) => {
      mode = parseStreetmanLeanCommand(args);
      ctx?.ui?.notify?.(`Streetman Lean mode set to ${mode}.`, "info");
    },
  });

  pi.registerCommand("streetman-lean-review", {
    description: "Run Streetman Lean review",
    handler: async (_args, ctx) => {
      pi.sendUserMessage("/skill:streetman-lean-review", ctx?.isIdle?.() === false ? { deliverAs: "followUp" } : undefined);
    },
  });

  pi.on("before_agent_start", async (event) => {
    if (mode === "off") return;
    const result = runStreetman(["lean", "instructions", "--mode", mode, "--host", "pi"]);
    const instructions = result.status === 0 && result.stdout.trim()
      ? result.stdout
      : `STREETMAN LEAN MODE ACTIVE - level: ${mode}\nUse stdlib/native first. No unrequested abstractions or deps.`;
    return { systemPrompt: `${event.systemPrompt}\n\n${instructions}` };
  });
}
