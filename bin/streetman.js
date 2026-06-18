#!/usr/bin/env node
// Thin launcher: runs the native streetman binary fetched by the postinstall
// step. streetman itself is a Rust single static binary; this npm package only
// downloads and shells out to it.
"use strict";
const { spawnSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const native = path.join(
  __dirname,
  process.platform === "win32" ? "streetman.exe" : "streetman-bin"
);

if (!fs.existsSync(native)) {
  console.error(
    "[streetman] native binary not found. Reinstall the package, or install directly with:\n" +
      "  cargo install --git https://github.com/efij/streetman streetman-cli --bin streetman --locked"
  );
  process.exit(1);
}

const result = spawnSync(native, process.argv.slice(2), { stdio: "inherit" });
if (result.error) {
  console.error("[streetman] failed to launch:", result.error.message);
  process.exit(1);
}
process.exit(result.status === null ? 1 : result.status);
