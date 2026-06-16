import { spawnSync } from "node:child_process";

export type StreetmanMessage = {
  role?: string;
  content?: unknown;
  [key: string]: unknown;
};

export type StreetmanOptions = {
  binary?: string;
  mode?: "lite" | "full" | "ultra" | "auto";
  domain?: string;
};

export function compile(text: string, options: StreetmanOptions = {}) {
  return runJson(
    options.binary ?? "streetman",
    ["compile", "--mode", options.mode ?? "full", "--domain", options.domain ?? "auto", "--json", "--no-archive"],
    text,
  );
}

export function compressText(text: string, options: StreetmanOptions = {}) {
  return runJson(
    options.binary ?? "streetman",
    ["compress", "--mode", options.mode ?? "full", "--domain", options.domain ?? "auto", "--json", "--no-archive"],
    text,
  );
}

export function compress(messages: StreetmanMessage[], options: StreetmanOptions = {}) {
  return messages.map((message) => {
    if (typeof message.content !== "string") return { ...message };
    return { ...message, content: compile(message.content, options).wire };
  });
}

function runJson(binary: string, args: string[], stdin: string) {
  const result = spawnSync(binary, args, {
    input: stdin,
    encoding: "utf8",
  });
  if (result.status !== 0) {
    throw new Error(result.stderr || `${binary} ${args.join(" ")} failed`);
  }
  return JSON.parse(result.stdout);
}
