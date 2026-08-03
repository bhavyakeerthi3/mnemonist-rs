'use strict';

const fs = require('fs');
const path = require('path');

const candidates = [
  path.join(__dirname, 'mnemonist.node'),
  path.join(__dirname, 'target', 'release', 'mnemonist.node'),
  path.join(__dirname, 'target', 'release', 'mnemonist.dll'),
  path.join(__dirname, 'target', 'release', 'libmnemonist.so'),
  path.join(__dirname, 'target', 'release', 'libmnemonist.dylib')
];

for (const candidate of candidates) {
  if (fs.existsSync(candidate)) {
    module.exports = require(candidate);
    return;
  }
}

throw new Error(
  'Native mnemonist binding not found. Run `npm run build:native` before the original JS adapter tests.'
);
