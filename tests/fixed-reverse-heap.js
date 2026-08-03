'use strict';

const { native, nullToUndefined } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol' ? require('../adapter/_protocol.js') : null;

function asArray(ArrayClass, values) {
  return ArrayClass === Array ? values : new ArrayClass(values);
}

function FixedReverseHeap(ArrayClass, comparator, capacity) {
  if (arguments.length === 2) {
    capacity = comparator;
    comparator = null;
  }

  if (typeof capacity !== 'number' || capacity <= 0) {
    throw new Error('mnemonist/FixedReverseHeap.constructor: capacity should be a number > 0.');
  }
  if (comparator !== null && typeof comparator !== 'function') {
    throw new Error('mnemonist/FixedReverseHeap.constructor: given comparator should be a function.');
  }

  this.ArrayClass = ArrayClass;
  this.capacity = capacity;
  this._comparator = comparator;
  this._fallback = comparator ? [] : null;
  this._inner = comparator ? null : (protocol ? protocol.create('fixed-reverse-heap', [capacity]) : new native.FixedReverseHeapInner(capacity));
}

FixedReverseHeap.prototype.clear = function() {
  if (this._fallback) this._fallback.length = 0;
  else return protocol ? this._inner.call('clear', []) : this._inner.clear();
};

FixedReverseHeap.prototype.push = function(value) {
  if (!this._fallback) return protocol ? this._inner.call('push', [value]) : this._inner.push(value);

  this._fallback.push(value);
  this._fallback.sort(this._comparator);
  if (this._fallback.length > this.capacity) this._fallback.pop();
  return this._fallback.length;
};

FixedReverseHeap.prototype.peek = function() {
  return this._fallback ? this._fallback[this._fallback.length - 1] : nullToUndefined(protocol ? this._inner.call('peek', []) : this._inner.peek());
};

FixedReverseHeap.prototype.consume = function() {
  const values = this._fallback ? this._fallback.splice(0) : (protocol ? this._inner.call('consume', []) : this._inner.consume());
  return asArray(this.ArrayClass, values);
};

FixedReverseHeap.prototype.toArray = function() {
  const values = this._fallback ? this._fallback.slice() : (protocol ? this._inner.call('toArray', []) : this._inner.toArray());
  return asArray(this.ArrayClass, values);
};

Object.defineProperty(FixedReverseHeap.prototype, 'size', {
  get: function() { return this._fallback ? this._fallback.length : (protocol ? this._inner.size() : this._inner.size); }
});

module.exports = FixedReverseHeap;
