'use strict';

const assert = require('assert');

const args = new Map(process.argv.slice(2).map(argument => {
  const [key, value] = argument.split('=');
  return [key, value];
}));
const durationMs = Number(args.get('--duration-ms') || 60000);
const maxSteps = args.has('--steps') ? Number(args.get('--steps')) : Infinity;
let state = Number(args.get('--seed') || 0x5ca1ab1e) >>> 0;

if (!Number.isFinite(durationMs) || durationMs <= 0 ||
    (args.has('--steps') && (!Number.isFinite(maxSteps) || maxSteps <= 0))) {
  throw new Error('--duration-ms and --steps must be positive numbers.');
}

process.env.MNEMONIST_TRANSPORT = 'protocol';
const RustStack = require('../tests/stack.js');
const RustQueue = require('../tests/queue.js');
const RustLruCache = require('../tests/lru-cache.js');
const RustBitVector = require('../tests/bit-vector.js');
delete process.env.MNEMONIST_TRANSPORT;

const OriginalStack = require('../original/mnemonist/stack.js');
const OriginalQueue = require('../original/mnemonist/queue.js');
const OriginalLruCache = require('../original/mnemonist/lru-cache.js');
const OriginalBitVector = require('../original/mnemonist/bit-vector.js');

function random() {
  state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
  return state / 0x100000000;
}

function value() {
  const choices = [null, false, true, 0, -1, 7, 42, 'alpha', 'beta'];
  return choices[Math.floor(random() * choices.length)];
}

function same(label, step, actual, expected, trail) {
  try {
    assert.deepStrictEqual(actual, expected);
  } catch (error) {
    throw new Error(
      `${label} divergence at step ${step}: ${error.message}\n` +
      `recent operations: ${trail.slice(-20).join(', ')}`
    );
  }
}

function runLinear(label, RustCollection, OriginalCollection, methods) {
  const rust = new RustCollection();
  const original = new OriginalCollection();
  const deadline = Date.now() + durationMs;
  const trail = [];
  let steps = 0;

  while (Date.now() < deadline && steps < maxSteps) {
    const roll = random();
    let rustResult;
    let originalResult;
    if (roll < 0.45 && rust.size < 48) {
      const item = value();
      trail.push(`${methods.add}(${JSON.stringify(item)})`);
      rustResult = rust[methods.add](item);
      originalResult = original[methods.add](item);
    } else if (roll < 0.72) {
      trail.push(`${methods.remove}()`);
      rustResult = rust[methods.remove]();
      originalResult = original[methods.remove]();
    } else if (roll < 0.9) {
      trail.push('peek()');
      rustResult = rust.peek();
      originalResult = original.peek();
    } else {
      trail.push('clear()');
      rust.clear();
      original.clear();
      rustResult = undefined;
      originalResult = undefined;
    }
    same(`${label}.result`, steps, rustResult, originalResult, trail);
    same(`${label}.size`, steps, rust.size, original.size, trail);
    same(`${label}.values`, steps, Array.from(rust.values()), Array.from(original.values()), trail);
    steps++;
  }
  return steps;
}

function runLru() {
  const rust = new RustLruCache(4);
  const original = new OriginalLruCache(4);
  const keys = ['alpha', 'beta', 'gamma', 'delta', 'epsilon', 'zeta'];
  const deadline = Date.now() + durationMs;
  const trail = [];
  let steps = 0;

  while (Date.now() < deadline && steps < maxSteps) {
    const key = keys[Math.floor(random() * keys.length)];
    const roll = random();
    let rustResult;
    let originalResult;
    if (roll < 0.38) {
      const item = value();
      trail.push(`set(${key}, ${JSON.stringify(item)})`);
      rustResult = rust.set(key, item);
      originalResult = original.set(key, item);
    } else if (roll < 0.58) {
      const item = value();
      trail.push(`setpop(${key}, ${JSON.stringify(item)})`);
      rustResult = rust.setpop(key, item);
      originalResult = original.setpop(key, item);
    } else if (roll < 0.74) {
      trail.push(`get(${key})`);
      rustResult = rust.get(key);
      originalResult = original.get(key);
    } else if (roll < 0.88) {
      trail.push(`peek(${key})`);
      rustResult = rust.peek(key);
      originalResult = original.peek(key);
    } else if (roll < 0.95) {
      trail.push(`has(${key})`);
      rustResult = rust.has(key);
      originalResult = original.has(key);
    } else {
      trail.push('clear()');
      rust.clear();
      original.clear();
      rustResult = undefined;
      originalResult = undefined;
    }
    same('LRU.result', steps, rustResult, originalResult, trail);
    same('LRU.size', steps, rust.size, original.size, trail);
    same('LRU.entries', steps, Array.from(rust.entries()), Array.from(original.entries()), trail);
    steps++;
  }
  return steps;
}

function runBitVector() {
  const rust = new RustBitVector();
  const original = new OriginalBitVector();
  const deadline = Date.now() + durationMs;
  const trail = [];
  let steps = 0;

  while (Date.now() < deadline && steps < maxSteps) {
    const index = Math.floor(random() * Math.max(1, Math.min(rust.length, 31)));
    const roll = random();
    let rustResult;
    let originalResult;

    // Keep the trace inside the source implementation's documented bit domain.
    if ((rust.length === 0 || roll < 0.35) && rust.length < 64) {
      const bit = random() < 0.5;
      trail.push(`push(${bit})`);
      rustResult = rust.push(bit);
      originalResult = original.push(bit);
    } else if (roll < 0.55) {
      trail.push(`set(${index})`);
      rust.set(index);
      original.set(index);
      rustResult = undefined;
      originalResult = undefined;
    } else if (roll < 0.7) {
      trail.push(`reset(${index})`);
      rust.reset(index);
      original.reset(index);
      rustResult = undefined;
      originalResult = undefined;
    } else if (roll < 0.85) {
      trail.push(`flip(${index})`);
      rust.flip(index);
      original.flip(index);
      rustResult = undefined;
      originalResult = undefined;
    } else {
      trail.push(`rank(${index})`);
      rustResult = rust.rank(index);
      originalResult = original.rank(index);
    }

    same('BitVector.result', steps, rustResult, originalResult, trail);
    same('BitVector.length', steps, rust.length, original.length, trail);
    same('BitVector.size', steps, rust.size, original.size, trail);
    same('BitVector.values', steps, Array.from(rust.values()), Array.from(original.values()), trail);
    steps++;
  }
  return steps;
}

const results = [
  ['stack', runLinear('Stack', RustStack, OriginalStack, { add: 'push', remove: 'pop' })],
  ['queue', runLinear('Queue', RustQueue, OriginalQueue, { add: 'enqueue', remove: 'dequeue' })],
  ['lru-cache', runLru()],
  ['bit-vector', runBitVector()]
];

console.log(`standalone collection differential fuzz passed; seed=${state}`);
for (const [name, steps] of results) {
  console.log(`  ${name}: ${steps} synchronized operations`);
}
console.log(`  total synchronized operations: ${results.reduce((total, [, steps]) => total + steps, 0)}`);
