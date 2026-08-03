'use strict';

const crypto = require('crypto');
const fs = require('fs');
const path = require('path');

const directory = path.join(__dirname, '..', 'tests', 'original');
const lines = fs.readdirSync(directory)
  .filter(name => name.endsWith('.js'))
  .sort()
  .map(name => {
    const digest = crypto.createHash('sha256')
      .update(fs.readFileSync(path.join(directory, name)))
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
