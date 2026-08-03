'use strict';

const [kind, structure, iterationsText] = process.argv.slice(2);
const iterations = Number(iterationsText);

if (!['original', 'napi'].includes(kind) || !['stack', 'queue'].includes(structure) || !Number.isInteger(iterations) || iterations <= 0) {
  throw new Error('usage: benchmark-rss-worker.js <original|napi> <stack|queue> <iterations>');
}

const base = kind === 'napi' ? '../tests' : '../original/mnemonist';
const Constructor = require(`${base}/${structure}.js`);
const collection = new Constructor();
const add = structure === 'stack' ? 'push' : 'enqueue';
const remove = structure === 'stack' ? 'pop' : 'dequeue';

for (let i = 0; i < iterations; i++) collection[add](i);
for (let i = 0; i < iterations; i++) collection[remove]();

const memory = process.memoryUsage();
console.log(JSON.stringify({
  rss_bytes: memory.rss,
  heap_used_bytes: memory.heapUsed,
  external_bytes: memory.external
}));
