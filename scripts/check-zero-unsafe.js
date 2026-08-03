'use strict';

const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..');
const violations = [];

function visit(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const target = path.join(directory, entry.name);
    if (entry.isDirectory()) visit(target);
    else if (entry.isFile() && entry.name.endsWith('.rs')) scan(target);
  }
}

function scan(file) {
  const lines = fs.readFileSync(file, 'utf8').split(/\r?\n/);
  for (let index = 0; index < lines.length; index++) {
    const code = lines[index].replace(/\/\/.*$/, '');
    if (/\bunsafe\s*(?:\{|fn\b|impl\b|extern\b)/.test(code)) {
      violations.push(`${path.relative(root, file)}:${index + 1}`);
    }
  }
}

visit(path.join(root, 'src'));
visit(path.join(root, 'tests'));
scan(path.join(root, 'build.rs'));

if (violations.length) {
  console.error(`unsafe Rust is forbidden by this port: ${violations.join(', ')}`);
  process.exit(1);
}

console.log('zero-unsafe audit: no unsafe Rust blocks, functions, impls, or extern declarations');
