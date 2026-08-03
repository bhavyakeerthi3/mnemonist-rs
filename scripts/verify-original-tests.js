'use strict';

const crypto = require('crypto');
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..');
const metadata = fs.readFileSync(path.join(root, '.port-mortem.toml'), 'utf8');
const expected = /^hash_at_kickoff\s*=\s*"([A-Fa-f0-9]+)"$/m.exec(metadata);

if (!expected) throw new Error('missing hash_at_kickoff in .port-mortem.toml');

const directory = path.join(root, 'tests', 'original');
const lines = fs.readdirSync(directory)
  .filter(name => name.endsWith('.js'))
  .sort()
  .map(name => {
    const digest = crypto.createHash('sha256')
      .update(fs.readFileSync(path.join(directory, name)))
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

console.log(`original test manifest: ${lines.length} files match kickoff ${actual}`);
