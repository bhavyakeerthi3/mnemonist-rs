'use strict';

const crypto = require('crypto');
const childProcess = require('child_process');
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..');
const directory = path.join(root, 'tests', 'original');
const lines = fs.readdirSync(directory)
  .filter(name => name.endsWith('.js'))
  .sort()
  .map(name => {
    const source = childProcess.execFileSync('git', ['show', `HEAD:tests/original/${name}`], {
      cwd: root
    });
    const digest = crypto.createHash('sha256')
      .update(source)
      .digest('hex')
      .toUpperCase();
    console.log(`${name} ${digest}`);
    return `${name} ${digest}`;
  });

const manifest = crypto.createHash('sha256')
  .update(lines.join('\n'), 'utf8')
  .digest('hex')
  .toUpperCase();

console.log(`\nMANIFEST_SHA256 ${manifest}`);
