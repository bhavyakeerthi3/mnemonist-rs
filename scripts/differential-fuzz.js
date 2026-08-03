'use strict';

const assert = require('assert');

const NativeStack = require('../tests/stack.js');
const NativeQueue = require('../tests/queue.js');
const NativeLinkedList = require('../tests/linked-list.js');
const NativeFixedStack = require('../tests/fixed-stack.js');
const NativeFixedDeque = require('../tests/fixed-deque.js');
const NativeBitVector = require('../tests/bit-vector.js');
const NativeLruCache = require('../tests/lru-cache.js');
const NativeLruCacheWithDelete = require('../tests/lru-cache-with-delete.js');
const NativeVector = require('../tests/vector.js');
const OriginalStack = require('../original/mnemonist/stack.js');
const OriginalQueue = require('../original/mnemonist/queue.js');
const OriginalLinkedList = require('../original/mnemonist/linked-list.js');
const OriginalFixedStack = require('../original/mnemonist/fixed-stack.js');
const OriginalFixedDeque = require('../original/mnemonist/fixed-deque.js');
const OriginalBitVector = require('../original/mnemonist/bit-vector.js');
const OriginalLruCache = require('../original/mnemonist/lru-cache.js');
const OriginalLruCacheWithDelete = require('../original/mnemonist/lru-cache-with-delete.js');
const OriginalVector = require('../original/mnemonist/vector.js');

const args = new Map(process.argv.slice(2).map(argument => {
  const [key, value] = argument.split('=');
  return [key, value];
}));
const durationMs = Number(args.get('--duration-ms') || 15000);
let state = Number(args.get('--seed') || 0x5eedc0de) >>> 0;

if (!Number.isFinite(durationMs) || durationMs <= 0) {
  throw new Error('--duration-ms must be a positive number.');
}

function random() {
  state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
  return state / 0x100000000;
}

function value() {
  const choices = [null, false, true, 0, -1, 7, 42, 'alpha', 'beta'];
  return choices[Math.floor(random() * choices.length)];
}

function same(label, step, nativeValue, originalValue) {
  try {
    assert.deepStrictEqual(nativeValue, originalValue);
  }
  catch (error) {
    throw new Error(`${label} divergence at step ${step}: ${error.message}`);
  }
}

function returnContract(value, instance) {
  return value === instance ? 'self' : value;
}

function exercise(label, createNative, createOriginal, methods) {
  const native = createNative();
  const original = createOriginal();
  const deadline = Date.now() + durationMs;
  let steps = 0;
  const trail = [];

  while (Date.now() < deadline) {
    const roll = random();
    let nativeResult;
    let originalResult;

    if (roll < 0.36 && native.size < 64) {
      const item = value();
      trail.push(`${methods.add}(${JSON.stringify(item)})`);
      nativeResult = native[methods.add](item);
      originalResult = original[methods.add](item);
    }
    else if (methods.addFront && roll < 0.52 && native.size < 64) {
      const item = value();
      trail.push(`${methods.addFront}(${JSON.stringify(item)})`);
      nativeResult = native[methods.addFront](item);
      originalResult = original[methods.addFront](item);
    }
    else if (roll < 0.70) {
      trail.push(`${methods.remove}()`);
      nativeResult = native[methods.remove]();
      originalResult = original[methods.remove]();
    }
    else if (roll < 0.84) {
      trail.push(`${methods.peek || 'peek'}()`);
      nativeResult = native[methods.peek || 'peek']();
      originalResult = original[methods.peek || 'peek']();
    }
    else if (roll < 0.92) {
      trail.push('clear()');
      native.clear();
      original.clear();
      nativeResult = undefined;
      originalResult = undefined;
    }
    else {
      trail.push('entries()');
      nativeResult = Array.from(native.entries());
      originalResult = Array.from(original.entries());
    }

    try {
      same(label, steps, nativeResult, originalResult);
      same(label, steps, native.size, original.size);
      same(label, steps, native.toArray(), original.toArray());
      same(label, steps, Array.from(native.values()), Array.from(original.values()));
      for (const method of methods.extra || []) {
        // Upstream LinkedList retains a stale tail after its final shift.
        if (method === 'last' && native.size === 0) continue;
        same(`${label}.${method}`, steps, native[method](), original[method]());
      }
    }
    catch (error) {
      throw new Error(`${error.message}\nrecent operations: ${trail.slice(-20).join(', ')}\nnative: ${JSON.stringify(native.toArray())}\noriginal: ${JSON.stringify(original.toArray())}`);
    }
    steps++;
  }

  return steps;
}

function exerciseBitVector() {
  const native = new NativeBitVector();
  const original = new OriginalBitVector();
  const deadline = Date.now() + durationMs;
  let steps = 0;
  const trail = [];

  while (Date.now() < deadline) {
    const roll = random();
    let nativeResult;
    let originalResult;

    if (roll < 0.30 && native.length < 96) {
      // Keeping pushed capacity-fill bits clear avoids upstream's signed
      // high-bit size-cache defect while still driving every growth tier.
      const bit = false;
      trail.push(`push(${bit})`);
      nativeResult = native.push(bit);
      originalResult = original.push(bit);
    }
    else if (roll < 0.48 && native.length > 0) {
      const index = Math.floor(random() * Math.min(native.length, 31));
      const bit = random() < 0.5;
      trail.push(`set(${index}, ${bit})`);
      nativeResult = native.set(index, bit);
      originalResult = original.set(index, bit);
    }
    else if (roll < 0.60 && native.length > 0) {
      const index = Math.floor(random() * Math.min(native.length, 31));
      trail.push(`reset(${index})`);
      nativeResult = native.reset(index);
      originalResult = original.reset(index);
    }
    else if (roll < 0.72 && native.length > 0) {
      const index = Math.floor(random() * Math.min(native.length, 31));
      trail.push(`flip(${index})`);
      nativeResult = native.flip(index);
      originalResult = original.flip(index);
    }
    else {
      const index = native.length === 0 ? 0 : Math.floor(random() * native.length);
      trail.push(`rank(${index})`);
      nativeResult = native.rank(index);
      originalResult = original.rank(index);
    }

    try {
      same('BitVector', steps, returnContract(nativeResult, native), returnContract(originalResult, original));
      same('BitVector.length', steps, native.length, original.length);
      same('BitVector.capacity', steps, native.capacity, original.capacity);
      const nativeValues = Array.from(native.values());
      const originalValues = Array.from(original.values());
      same('BitVector.values', steps, nativeValues, originalValues);
      same('BitVector.entries', steps, Array.from(native.entries()), Array.from(original.entries()));
      same('BitVector.nativeSizeInvariant', steps, native.size, nativeValues.reduce((count, bit) => count + bit, 0));
      // Upstream's select skips zero-valued 32-bit words when advancing its
      // position counter. Its result is therefore invalid beyond the first
      // word after a completely empty word; see UPSTREAM_FINDINGS.md.
      if (native.length <= 32) {
        for (let rank = 0; rank <= native.size + 1; rank++) {
          same('BitVector.select', steps, native.select(rank), original.select(rank));
        }
      }
    }
    catch (error) {
      throw new Error(`${error.message}\nrecent operations: ${trail.slice(-20).join(', ')}\nnative: ${JSON.stringify(Array.from(native.values()))}\noriginal: ${JSON.stringify(Array.from(original.values()))}`);
    }
    steps++;
  }

  return steps;
}

function exerciseLruCache() {
  const native = new NativeLruCache(8);
  const original = new OriginalLruCache(8);
  const deadline = Date.now() + durationMs;
  let steps = 0;
  const keys = ['alpha', 'beta', 'gamma', 'delta', 'epsilon', 'zeta', 'eta', 'theta', 'iota', 'kappa'];
  const trail = [];

  while (Date.now() < deadline) {
    const key = keys[Math.floor(random() * keys.length)];
    const roll = random();
    let nativeResult;
    let originalResult;

    if (roll < 0.34) {
      const item = value();
      trail.push(`set(${key}, ${JSON.stringify(item)})`);
      nativeResult = native.set(key, item);
      originalResult = original.set(key, item);
    }
    else if (roll < 0.55) {
      const item = value();
      trail.push(`setpop(${key}, ${JSON.stringify(item)})`);
      nativeResult = native.setpop(key, item);
      originalResult = original.setpop(key, item);
    }
    else if (roll < 0.72) {
      trail.push(`get(${key})`);
      nativeResult = native.get(key);
      originalResult = original.get(key);
    }
    else if (roll < 0.86) {
      trail.push(`peek(${key})`);
      nativeResult = native.peek(key);
      originalResult = original.peek(key);
    }
    else if (roll < 0.93) {
      trail.push('clear()');
      native.clear();
      original.clear();
      nativeResult = undefined;
      originalResult = undefined;
    }
    else {
      trail.push(`has(${key})`);
      nativeResult = native.has(key);
      originalResult = original.has(key);
    }

    try {
      same('LRUCache', steps, nativeResult, originalResult);
      same('LRUCache.size', steps, native.size, original.size);
      same('LRUCache.entries', steps, Array.from(native.entries()), Array.from(original.entries()));
      same('LRUCache.keys', steps, Array.from(native.keys()), Array.from(original.keys()));
      same('LRUCache.values', steps, Array.from(native.values()), Array.from(original.values()));
    }
    catch (error) {
      throw new Error(`${error.message}\nrecent operations: ${trail.slice(-20).join(', ')}\nnative: ${JSON.stringify(Array.from(native.entries()))}\noriginal: ${JSON.stringify(Array.from(original.entries()))}`);
    }
    steps++;
  }

  return steps;
}

function exerciseLruCacheWithDelete() {
  const native = new NativeLruCacheWithDelete(8);
  const original = new OriginalLruCacheWithDelete(8);
  const deadline = Date.now() + durationMs;
  let steps = 0;
  const keys = ['alpha', 'beta', 'gamma', 'delta', 'epsilon', 'zeta', 'eta', 'theta', 'iota', 'kappa'];
  const trail = [];

  while (Date.now() < deadline) {
    const key = keys[Math.floor(random() * keys.length)];
    const roll = random();
    let nativeResult;
    let originalResult;

    if (roll < 0.34) {
      const item = value();
      trail.push(`set(${key}, ${JSON.stringify(item)})`);
      nativeResult = native.set(key, item);
      originalResult = original.set(key, item);
    }
    else if (roll < 0.55) {
      const item = value();
      trail.push(`setpop(${key}, ${JSON.stringify(item)})`);
      nativeResult = native.setpop(key, item);
      originalResult = original.setpop(key, item);
    }
    else if (roll < 0.72) {
      trail.push(`get(${key})`);
      nativeResult = native.get(key);
      originalResult = original.get(key);
    }
    else if (roll < 0.86) {
      trail.push(`peek(${key})`);
      nativeResult = native.peek(key);
      originalResult = original.peek(key);
    }
    else if (roll < 0.93) {
      trail.push('clear()');
      native.clear();
      original.clear();
      nativeResult = undefined;
      originalResult = undefined;
    }
    else {
      trail.push(`has(${key})`);
      nativeResult = native.has(key);
      originalResult = original.has(key);
    }

    try {
      same('LRUCacheWithDelete', steps, nativeResult, originalResult);
      same('LRUCacheWithDelete.size', steps, native.size, original.size);
      same('LRUCacheWithDelete.entries', steps, Array.from(native.entries()), Array.from(original.entries()));
      same('LRUCacheWithDelete.keys', steps, Array.from(native.keys()), Array.from(original.keys()));
      same('LRUCacheWithDelete.values', steps, Array.from(native.values()), Array.from(original.values()));
    }
    catch (error) {
      throw new Error(`${error.message}\nrecent operations: ${trail.slice(-20).join(', ')}\nnative: ${JSON.stringify(Array.from(native.entries()))}\noriginal: ${JSON.stringify(Array.from(original.entries()))}`);
    }
    steps++;
  }

  return steps;
}

function exerciseVector() {
  const native = new NativeVector(Uint16Array, 4);
  const original = new OriginalVector(Uint16Array, 4);
  const deadline = Date.now() + durationMs;
  let steps = 0;
  const trail = [];

  while (Date.now() < deadline) {
    const roll = random();
    let nativeResult;
    let originalResult;

    if (roll < 0.34 && native.length < 64) {
      const item = Math.floor(random() * 80000) - 1000;
      trail.push(`push(${item})`);
      nativeResult = native.push(item);
      originalResult = original.push(item);
    }
    else if (roll < 0.52 && native.length > 0) {
      const index = Math.floor(random() * native.length);
      const item = Math.floor(random() * 80000) - 1000;
      trail.push(`set(${index}, ${item})`);
      nativeResult = native.set(index, item);
      originalResult = original.set(index, item);
    }
    else if (roll < 0.66) {
      trail.push('pop()');
      nativeResult = native.pop();
      originalResult = original.pop();
    }
    else if (roll < 0.78) {
      const length = Math.floor(random() * 65);
      trail.push(`resize(${length})`);
      nativeResult = native.resize(length);
      originalResult = original.resize(length);
    }
    else if (roll < 0.89) {
      const capacity = Math.floor(random() * 65);
      trail.push(`reallocate(${capacity})`);
      nativeResult = native.reallocate(capacity);
      originalResult = original.reallocate(capacity);
    }
    else {
      const capacity = Math.floor(random() * 65);
      trail.push(`grow(${capacity})`);
      nativeResult = native.grow(capacity);
      originalResult = original.grow(capacity);
    }

    try {
      same('Vector', steps, returnContract(nativeResult, native), returnContract(originalResult, original));
      same('Vector.length', steps, native.length, original.length);
      same('Vector.capacity', steps, native.capacity, original.capacity);
      same('Vector.values', steps, Array.from(native.values()), Array.from(original.values()));
      same('Vector.entries', steps, Array.from(native.entries()), Array.from(original.entries()));
      same('Vector.array', steps, Array.from(native.array.slice(0, native.length)), Array.from(original.array.slice(0, original.length)));
    }
    catch (error) {
      throw new Error(`${error.message}\nrecent operations: ${trail.slice(-20).join(', ')}\nnative: ${JSON.stringify(Array.from(native.values()))}\noriginal: ${JSON.stringify(Array.from(original.values()))}`);
    }
    steps++;
  }

  return steps;
}

const stackSteps = exercise('Stack', () => new NativeStack(), () => new OriginalStack(), { add: 'push', remove: 'pop' });
const queueSteps = exercise('Queue', () => new NativeQueue(), () => new OriginalQueue(), { add: 'enqueue', remove: 'dequeue' });
const linkedListSteps = exercise('LinkedList', () => new NativeLinkedList(), () => new OriginalLinkedList(), {
  add: 'push',
  addFront: 'unshift',
  remove: 'shift',
  extra: ['first', 'last']
});
const fixedStackSteps = exercise('FixedStack', () => new NativeFixedStack(Array, 64), () => new OriginalFixedStack(Array, 64), {
  add: 'push',
  remove: 'pop'
});
const fixedDequeSteps = exercise('FixedDeque', () => new NativeFixedDeque(Array, 64), () => new OriginalFixedDeque(Array, 64), {
  add: 'push',
  addFront: 'unshift',
  remove: 'shift',
  peek: 'peekFirst',
  extra: ['peekLast']
});
const bitVectorSteps = exerciseBitVector();
const lruCacheSteps = exerciseLruCache();
const lruCacheWithDeleteSteps = exerciseLruCacheWithDelete();
const vectorSteps = exerciseVector();

console.log(JSON.stringify({
  status: 'pass',
  seed: `0x${Number(args.get('--seed') || 0x5eedc0de).toString(16)}`,
  duration_ms_per_structure: durationMs,
  stack_steps: stackSteps,
  queue_steps: queueSteps,
  linked_list_steps: linkedListSteps,
  fixed_stack_steps: fixedStackSteps,
  fixed_deque_steps: fixedDequeSteps,
  bit_vector_steps: bitVectorSteps,
  lru_cache_steps: lruCacheSteps,
  lru_cache_with_delete_steps: lruCacheWithDeleteSteps,
  vector_steps: vectorSteps,
  total_steps: stackSteps + queueSteps + linkedListSteps + fixedStackSteps + fixedDequeSteps + bitVectorSteps + lruCacheSteps + lruCacheWithDeleteSteps + vectorSteps
}));
