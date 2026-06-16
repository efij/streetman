const vscode = require('vscode');
const cp = require('child_process');

function runStreetman(args) {
  const bin = process.env.STREETMAN_BIN || 'streetman';
  const cwd = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  const result = cp.spawnSync(bin, args, { cwd, encoding: 'utf8' });
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error(result.stderr || result.stdout);
  return result.stdout;
}

async function showDocument(name, text) {
  const doc = await vscode.workspace.openTextDocument({ language: 'markdown', content: text });
  await vscode.window.showTextDocument(doc, { preview: true });
}

function activate(context) {
  context.subscriptions.push(vscode.commands.registerCommand('streetmanLean.instructions', async () => {
    await showDocument('streetman-lean', runStreetman(['lean', 'instructions', '--mode', 'full', '--host', 'vscode']));
  }));
  context.subscriptions.push(vscode.commands.registerCommand('streetmanLean.review', async () => {
    await showDocument('streetman-lean-review', runStreetman(['lean', 'review', '--diff']));
  }));
  context.subscriptions.push(vscode.commands.registerCommand('streetmanLean.gate', async () => {
    await showDocument('streetman-lean-gate', runStreetman(['lean', 'gate', '--before', 'base', '--after', 'HEAD']));
  }));
  context.subscriptions.push(vscode.commands.registerCommand('streetmanLean.prove', async () => {
    await showDocument('streetman-lean-proof', runStreetman(['lean', 'prove', '--diff']));
  }));
}

module.exports = { activate };
