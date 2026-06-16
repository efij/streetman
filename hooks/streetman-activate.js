#!/usr/bin/env node
const fs = require('fs');
const os = require('os');
const path = require('path');
const { spawnSync } = require('child_process');

const isCodex = Boolean(process.env.PLUGIN_DATA);
const mode = normalize(process.env.STREETMAN_LEAN_DEFAULT || readConfigDefault() || 'full');
const statePath = isCodex
  ? path.join(process.env.PLUGIN_DATA, '.streetman-lean-active')
  : path.join(os.homedir(), '.streetman', '.streetman-lean-active');

function normalize(value) {
  return ['off', 'lite', 'full', 'ultra'].includes(String(value).toLowerCase())
    ? String(value).toLowerCase()
    : 'full';
}

function readConfigDefault() {
  try {
    const raw = fs.readFileSync(path.join(os.homedir(), '.config', 'streetman', 'config.json'), 'utf8');
    return JSON.parse(raw).leanDefaultMode;
  } catch (_) {
    return null;
  }
}

function instructions() {
  if (mode === 'off') return 'STREETMAN LEAN MODE OFF';
  const bin = process.env.STREETMAN_BIN || 'streetman';
  const result = spawnSync(bin, ['lean', 'instructions', '--mode', mode, '--host', isCodex ? 'codex' : 'claude'], { encoding: 'utf8' });
  if (result.status === 0 && result.stdout.trim()) return result.stdout;
  return `STREETMAN LEAN MODE ACTIVE - level: ${mode}\nUse stdlib/native first, no unrequested abstractions or deps, one runnable check for non-trivial logic, mark shortcuts with streetman: ceiling + upgrade path.`;
}

try {
  if (mode === 'off') {
    fs.rmSync(statePath, { force: true });
  } else {
    fs.mkdirSync(path.dirname(statePath), { recursive: true });
    fs.writeFileSync(statePath, mode);
  }
} catch (_) {}

const context = instructions();
if (isCodex) {
  process.stdout.write(JSON.stringify({
    systemMessage: `STREETMAN:LEAN:${mode.toUpperCase()}`,
    hookSpecificOutput: { hookEventName: 'SessionStart', additionalContext: context }
  }));
} else {
  process.stdout.write(context);
}
