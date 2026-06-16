import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

const statePath = path.join(process.env.XDG_CONFIG_HOME || path.join(os.homedir(), '.config'), 'opencode', '.streetman-lean-active');

function mode() {
  try { return fs.readFileSync(statePath, 'utf8').trim() || 'full'; } catch (_) { return process.env.STREETMAN_LEAN_DEFAULT || 'full'; }
}

function writeMode(value) {
  fs.mkdirSync(path.dirname(statePath), { recursive: true });
  fs.writeFileSync(statePath, value);
}

function instructions(value) {
  if (value === 'off') return '';
  const bin = process.env.STREETMAN_BIN || 'streetman';
  const result = spawnSync(bin, ['lean', 'instructions', '--mode', value, '--host', 'opencode'], { encoding: 'utf8' });
  return result.status === 0 ? result.stdout : `STREETMAN LEAN MODE ACTIVE - level: ${value}\nUse stdlib/native first. No unrequested abstractions or deps.`;
}

export default async () => ({
  'experimental.chat.system.transform': async (_input, output) => {
    const current = mode();
    const text = instructions(current);
    if (text) output.system.push(text);
  },
  'command.execute.before': async (input) => {
    if (input?.command !== 'streetman-lean') return;
    const value = String(input.arguments || 'full').trim().toLowerCase();
    writeMode(['off', 'lite', 'full', 'ultra'].includes(value) ? value : 'full');
  }
});
