'use strict';

const childProcess = require('child_process');
const path = require('path');

const root = path.join(__dirname, '..');
const mocha = require.resolve('mocha/bin/mocha.js');
const files = [
  'stack.js',
  'queue.js',
  'linked-list.js',
  'fixed-stack.js',
  'fixed-deque.js',
  'circular-buffer.js',
  'sparse-set.js',
  'bit-set.js',
  'sparse-queue-set.js',
  'sparse-map.js',
  'static-disjoint-set.js',
  'multi-array.js',
  'multi-set.js',
  'passjoin-index.js',
  'multi-map.js',
  'bi-map.js',
  'bk-tree.js',
  'heap.js',
  'fixed-reverse-heap.js',
  'fibonacci-heap.js',
  'hashed-array-tree.js',
  'set.js',
  'bloom-filter.js',
  'bit-vector.js',
  'vector.js',
  'suffix-array.js',
  'static-interval-tree.js',
  'trie-map.js',
  'trie.js',
  'inverted-index.js',
  'kd-tree.js',
  'symspell.js',
  'sort.js',
  'critbit-tree-map.js',
  'default-map.js',
  'default-weak-map.js',
  'fixed-critbit-tree-map.js',
  'fuzzy-map.js',
  'fuzzy-multi-map.js',
  'vp-tree.js',
  'lru-cache.js'
]
  .map(file => path.join(root, 'tests', 'original', file));

const result = childProcess.spawnSync(process.execPath, [mocha, '--timeout', '20000', ...files], {
  cwd: root,
  env: { ...process.env, MNEMONIST_TRANSPORT: 'protocol' },
  stdio: 'inherit'
});

if (result.error) throw result.error;
process.exit(result.status === null ? 1 : result.status);
