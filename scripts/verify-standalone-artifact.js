'use strict';

const childProcess = require('child_process');
const path = require('path');

const root = path.join(__dirname, '..');

function run(command, args, options = {}) {
  const result = childProcess.spawnSync(command, args, {
    cwd: root,
    encoding: 'utf8',
    ...options
  });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    process.stderr.write(result.stderr || '');
    process.exit(result.status || 1);
  }
  return result.stdout || '';
}

const tree = run('cargo', ['tree', '--no-default-features', '-e', 'normal']);
if (/\bnapi(?:-derive)? v/.test(tree)) {
  throw new Error('standalone dependency tree includes the optional Node N-API bridge');
}

run('cargo', ['build', '--release', '--no-default-features', '--bin', 'mnemonist'], {
  stdio: 'inherit'
});

const executable = path.join(
  root,
  'target',
  'release',
  process.platform === 'win32' ? 'mnemonist.exe' : 'mnemonist'
);
const version = run(executable, ['--version']);
if (!/^mnemonist-jsonl \d+\r?\n$/.test(version)) {
  throw new Error(`unexpected standalone artifact response: ${JSON.stringify(version)}`);
}

console.log('standalone artifact: Rust-only dependency tree and runnable release executable verified');
