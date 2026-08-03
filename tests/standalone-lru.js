'use strict';

const assert = require('assert');

process.env.MNEMONIST_TRANSPORT = 'protocol';
const LRUCache = require('./lru-cache.js');
const LRUCacheWithDelete = require('./lru-cache-with-delete.js');
const LRUMapWithDelete = require('./lru-map-with-delete.js');

describe('Standalone LRU cache', function() {
  it('runs promotion, eviction, and setpop through the Rust protocol runner', function() {
    const cache = new LRUCache(2);
    cache.set('one', 1);
    cache.set('two', 2);
    assert.strictEqual(cache.get('one'), 1);
    cache.set('three', 3);
    assert.deepStrictEqual(Array.from(cache.entries()), [['three', 3], ['one', 1]]);
    assert.deepStrictEqual(cache.setpop('four', 4), { evicted: true, key: 'one', value: 1 });
    assert.deepStrictEqual(Array.from(cache.entries()), [['four', 4], ['three', 3]]);
  });
});

describe('Standalone LRU map with delete', function() {
  it('preserves opaque key and value identities without a JavaScript value cache', function() {
    const cache = new LRUMapWithDelete(2);
    const key = { key: 1 };
    const value = { value: 1 };
    cache.set(key, value);
    assert.strictEqual(cache.get(key), value);
    assert.strictEqual(cache.remove(key), value);
    assert.strictEqual(cache.items.size, 0);
  });
});

describe('Standalone LRU cache with delete', function() {
  it('preserves object identity and stored undefined through Rust-owned ordering', function() {
    const cache = new LRUCacheWithDelete(2);
    const object = { id: 1 };
    cache.set('object', object);
    cache.set('missing', undefined);
    assert.strictEqual(cache.get('object'), object);
    assert.strictEqual(cache.remove('missing', 'fallback'), undefined);
    assert.strictEqual(cache.remove('missing', 'fallback'), 'fallback');
    cache.set('object', object);
    assert.strictEqual(cache.remove('object'), object);
  });
});
