'use strict';

const assert = require('assert');
const native = require('../index.js');
const Vector = require('./vector.js');

describe('Native bridge regressions', function() {
  it('stores standard typed Vector data in Rust while preserving typed coercion', function() {
    const vector = new Vector(Uint8Array, 2);

    assert(vector._inner instanceof native.VectorInner);

    vector.push(300);
    vector.push(7);
    vector.resize(3);

    assert.strictEqual(vector.get(0), 44);
    assert.deepStrictEqual(Array.from(vector.values()), [44, 7, 0]);
    assert(vector.array instanceof Uint8Array);

    vector.reallocate(2);
    assert.deepStrictEqual(Array.from(vector), [44, 7]);
  });

  it('keeps arbitrary JavaScript arrays on the compatibility path', function() {
    const vector = new Vector(Array, 1);

    assert.strictEqual(vector._inner, null);
    vector.push({ id: 1 });
    vector.push({ id: 2 });

    assert.deepStrictEqual(Array.from(vector.values()), [{ id: 1 }, { id: 2 }]);
  });
});
