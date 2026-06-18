#!/usr/bin/env node
// postinstall: download the prebuilt streetman binary for this platform from the
// latest GitHub release. On any failure it prints the cargo fallback and exits 0
// (so `npm install` never hard-fails); the launcher reports a clear error if the
// binary is missing at run time.
"use strict";
const https = require("https");
const fs = require("fs");
const path = require("path");

const REPO = "efij/streetman";
const ASSET = {
  "darwin arm64": "streetman-darwin-arm64",
  "darwin x64": "streetman-darwin-x64",
  "linux arm64": "streetman-linux-arm64",
  "linux x64": "streetman-linux-x64",
  "win32 x64": "streetman-windows-x64.exe",
};

const key = `${process.platform} ${process.arch}`;
const asset = ASSET[key];
const binDir = path.join(__dirname, "..", "bin");
const dest = path.join(binDir, process.platform === "win32" ? "streetman.exe" : "streetman-bin");

function fallback(reason) {
  console.error(
    `\n[streetman] ${reason}\n` +
      `Install directly with Rust instead:\n` +
      `  cargo install --git https://github.com/${REPO} streetman-cli --bin streetman --locked\n`
  );
  process.exit(0); // do not fail the whole npm install
}

if (!asset) fallback(`no prebuilt binary published for ${key}.`);

function download(url, depth) {
  if (depth > 6) return fallback("too many redirects while downloading.");
  https
    .get(url, { headers: { "User-Agent": "streetman-npm-install" } }, (res) => {
      if ([301, 302, 303, 307, 308].includes(res.statusCode)) {
        return download(res.headers.location, depth + 1);
      }
      if (res.statusCode !== 200) return fallback(`download failed (HTTP ${res.statusCode}).`);
      fs.mkdirSync(binDir, { recursive: true });
      const file = fs.createWriteStream(dest);
      res.pipe(file);
      file.on("finish", () =>
        file.close(() => {
          try {
            if (process.platform !== "win32") fs.chmodSync(dest, 0o755);
          } catch (_) {}
          console.log(`[streetman] installed prebuilt binary: ${asset}`);
        })
      );
      file.on("error", (e) => fallback(e.message));
    })
    .on("error", (e) => fallback(e.message));
}

download(`https://github.com/${REPO}/releases/latest/download/${asset}`, 0);
