#!/usr/bin/env node
const fs = require('fs');
const os = require('os');
const path = require('path');

const isCodex = Boolean(process.env.PLUGIN_DATA);
const statePath = isCodex
  ? path.join(process.env.PLUGIN_DATA, '.streetman-lean-active')
  : path.join(os.homedir(), '.streetman', '.streetman-lean-active');

let input = '';
process.stdin.on('data', chunk => { input += chunk; });
process.stdin.on('end', () => {
  try {
    const data = JSON.parse(input.replace(/^\uFEFF/, ''));
    const prompt = String(data.prompt || '').trim().toLowerCase();
    const match = prompt.match(/^[/@$]streetman-lean(?:\s+(off|lite|full|ultra))?/);
    let mode = match ? (match[1] || 'full') : null;
    if (/\b(stop streetman lean|streetman normal|normal mode)\b/.test(prompt)) mode = 'off';
    if (!mode) return;
    if (mode === 'off') fs.rmSync(statePath, { force: true });
    else {
      fs.mkdirSync(path.dirname(statePath), { recursive: true });
      fs.writeFileSync(statePath, mode);
    }
    if (isCodex) {
      process.stdout.write(JSON.stringify({
        systemMessage: `STREETMAN:LEAN:${mode.toUpperCase()}`,
        hookSpecificOutput: { hookEventName: 'UserPromptSubmit', additionalContext: `STREETMAN LEAN MODE ${mode.toUpperCase()}` }
      }));
    } else {
      process.stdout.write(`STREETMAN LEAN MODE ${mode.toUpperCase()}`);
    }
  } catch (_) {}
});
