'use strict';

const assert = require('assert');
const childProcess = require('child_process');
const path = require('path');

const root = path.join(__dirname, '..');
const executable = path.join(
  root,
  'target',
  'release',
  process.platform === 'win32' ? 'mnemonist.exe' : 'mnemonist'
);

const requests = [
  { id: 'hello', op: 'hello' },
  { id: 'stack', op: 'create', kind: 'stack' },
  { id: 'stack', op: 'call', method: 'push', args: [1] },
  { id: 'stack', op: 'call', method: 'push', args: [null] },
  { id: 'stack', op: 'call', method: 'peek' },
  { id: 'stack', op: 'call', method: 'pop' },
  { id: 'stack', op: 'snapshot' },
  { id: 'queue', op: 'create', kind: 'queue' },
  { id: 'queue', op: 'call', method: 'enqueue', args: ['first'] },
  { id: 'queue', op: 'call', method: 'enqueue', args: ['second'] },
  { id: 'queue', op: 'call', method: 'dequeue' },
  { id: 'queue', op: 'snapshot' },
  { id: 'list', op: 'create', kind: 'linked-list' },
  { id: 'list', op: 'call', method: 'push', args: ['tail'] },
  { id: 'list', op: 'call', method: 'unshift', args: ['head'] },
  { id: 'list', op: 'call', method: 'last' },
  { id: 'list', op: 'snapshot' },
  { id: 'fixed', op: 'create', kind: 'fixed-stack', args: [2] },
  { id: 'fixed', op: 'call', method: 'push', args: ['first'] },
  { id: 'fixed', op: 'call', method: 'push', args: ['second'] },
  { id: 'fixed', op: 'call', method: 'pop' },
  { id: 'fixed', op: 'snapshot' },
  { id: 'weak', op: 'create', kind: 'default-weak-map' },
  { id: 'weak', op: 'call', method: 'set', args: [{ __mnemonist_protocol_type__: 'handle', handle: 901 }, { __mnemonist_protocol_type__: 'handle', handle: 902 }] },
  { id: 'weak', op: 'call', method: 'has', args: [{ __mnemonist_protocol_type__: 'handle', handle: 901 }] },
  { id: 'weak', op: 'call', method: 'release', args: [{ __mnemonist_protocol_type__: 'handle', handle: 901 }] },
  { id: 'weak', op: 'call', method: 'has', args: [{ __mnemonist_protocol_type__: 'handle', handle: 901 }] },
  { id: 'comparator', op: 'create', kind: 'comparator-heap', args: [false] },
  { id: 'comparator', op: 'call', method: 'pushCompared', args: [3, [[3, 3, 0]]] },
  { id: 'comparator', op: 'call', method: 'pushCompared', args: [1, [[1, 1, 0], [1, 3, -1], [3, 1, 1], [3, 3, 0]]] },
  { id: 'comparator', op: 'call', method: 'peekCompared', args: [[[1, 1, 0], [1, 3, -1], [3, 1, 1], [3, 3, 0]]] },
  { id: 'comparator', op: 'call', method: 'replaceCompared', args: [2, [[1, 1, 0], [1, 2, -1], [1, 3, -1], [2, 1, 1], [2, 2, 0], [2, 3, -1], [3, 1, 1], [3, 2, 1], [3, 3, 0]]] },
  { id: 'comparator', op: 'call', method: 'consumeCompared', args: [[[2, 2, 0], [2, 3, -1], [3, 2, 1], [3, 3, 0]]] },
  { id: 'fibonacci', op: 'create', kind: 'fibonacci-heap' },
  { id: 'fibonacci', op: 'call', method: 'push', args: [8] },
  { id: 'fibonacci', op: 'call', method: 'push', args: [1] },
  { id: 'fibonacci', op: 'call', method: 'push', args: [3] },
  { id: 'fibonacci', op: 'call', method: 'pop' },
  { id: 'fibonacci', op: 'call', method: 'consume' }
];

const run = childProcess.spawnSync(executable, [], {
  cwd: root,
  encoding: 'utf8',
  input: `${requests.map(JSON.stringify).join('\n')}\n`
});

assert.ifError(run.error);
assert.strictEqual(run.status, 0, run.stderr);

const responses = run.stdout.trim().split(/\r?\n/).map(JSON.parse);
assert.strictEqual(responses.length, requests.length);
assert.deepStrictEqual(responses[0].result.value, {
  protocol: 'mnemonist-jsonl',
  version: 1,
  collections: [
    'stack',
    'queue',
    'linked-list',
    'fixed-stack',
    'fixed-deque',
    'circular-buffer',
    'sparse-set',
    'lru-cache',
    'bit-set',
    'sparse-queue-set',
    'sparse-map',
    'static-disjoint-set',
    'multi-array',
    'multi-set',
    'multi-map',
    'bi-map',
    'heap',
    'fibonacci-heap',
    'fixed-reverse-heap',
    'hashed-array-tree',
    'set-ops',
    'bloom-filter',
    'bit-vector',
    'vector',
    'suffix-array',
    'generalized-suffix-array',
    'static-interval-tree',
  'trie-map',
  'inverted-index',
  'symspell',
  'bk-tree',
  'passjoin-index',
  'kd-tree',
  'default-map',
  'default-weak-map',
  'fuzzy-map',
    'fuzzy-multi-map',
    'vp-tree',
    'comparator-heap',
    'sort',
    'critbit-tree-map',
    'fixed-critbit-tree-map'
  ]
});
assert.deepStrictEqual(responses[4].result, { kind: 'value', value: null });
assert.deepStrictEqual(responses[5].result, { kind: 'value', value: null });
assert.deepStrictEqual(responses[6].result.value, { kind: 'stack', values: [1] });
assert.deepStrictEqual(responses[11].result.value, { kind: 'queue', values: ['second'] });
assert.deepStrictEqual(responses[15].result, { kind: 'value', value: 'tail' });
assert.deepStrictEqual(responses[16].result.value, {
  kind: 'linked-list',
  values: ['head', 'tail']
});
assert.deepStrictEqual(responses[20].result, { kind: 'value', value: 'second' });
assert.deepStrictEqual(responses[21].result.value, {
  kind: 'fixed-stack',
  values: ['first']
});
assert.deepStrictEqual(responses[22].result, { kind: 'void' });
assert.deepStrictEqual(responses[23].result, { kind: 'void' });
assert.deepStrictEqual(responses[24].result, { kind: 'value', value: true });
assert.deepStrictEqual(responses[25].result, { kind: 'value', value: true });
assert.deepStrictEqual(responses[26].result, { kind: 'value', value: false });
assert.deepStrictEqual(responses[30].result, { kind: 'value', value: 1 });
assert.deepStrictEqual(responses[31].result, { kind: 'value', value: 1 });
assert.deepStrictEqual(responses[32].result, { kind: 'value', value: [2, 3] });
assert.deepStrictEqual(responses[37].result, { kind: 'value', value: 1 });
assert.deepStrictEqual(responses[38].result, { kind: 'value', value: [3, 8] });

console.log(`standalone protocol: ${responses.length} stateful requests passed`);
