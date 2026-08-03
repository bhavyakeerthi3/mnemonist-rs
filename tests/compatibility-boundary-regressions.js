'use strict';

const assert = require('assert');
const DefaultMap = require('./default-map.js');
const DefaultWeakMap = require('./default-weak-map.js');
const FibonacciHeap = require('./fibonacci-heap.js');

describe('JavaScript compatibility boundaries', function() {
  it('preserves DefaultMap factory closures, keys, insertion indexes, and identities', function() {
    const offset = 17;
    const map = new DefaultMap((key, index) => ({ key, index, offset }));
    const first = map.get('first');

    assert.deepStrictEqual(first, { key: 'first', index: 0, offset: 17 });
    assert.strictEqual(map.get('first'), first);
    assert.deepStrictEqual(map.get('second'), { key: 'second', index: 1, offset: 17 });
  });

  it('preserves DefaultWeakMap object-key identity and stored undefined', function() {
    let calls = 0;
    const key = {};
    const map = new DefaultWeakMap(received => {
      calls++;
      assert.strictEqual(received, key);
      return undefined;
    });

    assert.strictEqual(map.get(key), undefined);
    assert.strictEqual(map.has(key), true);
    assert.strictEqual(map.get(key), undefined);
    assert.strictEqual(calls, 1);
    assert.strictEqual(map.delete(key), true);
    assert.strictEqual(map.has(key), false);
  });

  it('preserves comparators that close over JavaScript state', function() {
    const preferred = new Set(['priority']);
    const heap = new FibonacciHeap((left, right) => {
      if (preferred.has(left.kind) !== preferred.has(right.kind)) {
        return preferred.has(left.kind) ? -1 : 1;
      }
      return left.rank - right.rank;
    });
    const ordinary = { kind: 'ordinary', rank: 1 };
    const priority = { kind: 'priority', rank: 9 };

    heap.push(ordinary);
    heap.push(priority);
    assert.strictEqual(heap.pop(), priority);
    assert.strictEqual(heap.pop(), ordinary);
  });
});
