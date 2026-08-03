'use strict';

const crypto = require('crypto');
const childProcess = require('child_process');
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..');
const metadata = fs.readFileSync(path.join(root, '.port-mortem.toml'), 'utf8');
const expected = /^hash_at_kickoff\s*=\s*"([A-Fa-f0-9]+)"$/m.exec(metadata);

if (!expected) throw new Error('missing hash_at_kickoff in .port-mortem.toml');

const directory = path.join(root, 'tests', 'original');
const diff = childProcess.spawnSync('git', ['diff', '--quiet', 'HEAD', '--', 'tests/original'], {
  cwd: root
});

if (diff.error) throw diff.error;
if (diff.status !== 0) {
  throw new Error('tests/original has changes relative to HEAD; preserved tests must be unmodified');
}

const lines = fs.readdirSync(directory)
  .filter(name => name.endsWith('.js'))
  .sort()
  .map(name => {
    // Read Git's committed blob, not checkout bytes. This keeps the kickoff
    // proof stable when a clean clone uses core.autocrlf or another EOL policy.
    const source = childProcess.execFileSync('git', ['show', `HEAD:tests/original/${name}`], {
      cwd: root
    });
    const digest = crypto.createHash('sha256')
      .update(source)
      .digest('hex')
      .toUpperCase();
    return `${name} ${digest}`;
  });
const actual = crypto.createHash('sha256')
  .update(lines.join('\n'), 'utf8')
  .digest('hex')
  .toUpperCase();

if (actual !== expected[1].toUpperCase()) {
  throw new Error(`original test manifest mismatch: expected ${expected[1]}, got ${actual}`);
}

console.log(`original test manifest: ${lines.length} committed files match kickoff ${actual}`);
