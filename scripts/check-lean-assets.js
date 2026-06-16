#!/usr/bin/env node
const fs = require('fs');

const required = [
  'AGENTS.md',
  'skills/streetman-lean/SKILL.md',
  'skills/streetman-lean-review/SKILL.md',
  'skills/streetman-lean-audit/SKILL.md',
  '.cursor/rules/streetman-lean.mdc',
  '.windsurf/rules/streetman-lean.md',
  '.clinerules/streetman-lean.md',
  '.github/copilot-instructions.md',
  '.kiro/steering/streetman-lean.md',
  '.zed/streetman-lean.md',
  'vscode-extension/package.json',
  'vscode-extension/extension.js',
  'hooks/streetman-activate.js',
  'hooks/streetman-mode-tracker.js',
  '.opencode/plugins/streetman-lean.mjs',
  'pi-extension/index.js',
  'gemini-extension.json',
  '.codex-plugin/plugin.json',
  '.claude-plugin/plugin.json',
];

const failures = [];
for (const file of required) {
  if (!fs.existsSync(file)) {
    failures.push(`${file}: missing`);
    continue;
  }
  const text = fs.readFileSync(file, 'utf8');
  if (!/streetman/i.test(text)) failures.push(`${file}: missing streetman marker`);
  if (!/(lean|smallest|stdlib|standard library|native)/i.test(text)) {
    failures.push(`${file}: missing lean/minimalism marker`);
  }
}

if (failures.length) {
  console.error(failures.join('\n'));
  process.exit(1);
}

console.log(`lean asset sync ok (${required.length} files)`);
