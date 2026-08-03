'use strict';

const { native, nullToUndefined } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol' ? require('../adapter/_protocol.js') : null;

const defaultComparator = (a, b) => (a < b ? -1 : a > b ? 1 : 0);

function createHeap(max, comparator) {
  if (comparator !== undefined && typeof comparator !== 'function') {
    throw new Error('mnemonist/FibonacciHeap.constructor: given comparator should be a function.');
  }

  this._max = max;
  this._comparator = comparator || defaultComparator;
  this._items = protocol ? null : (comparator ? [] : null);
  this._inner = protocol
    ? protocol.create(comparator ? 'comparator-heap' : 'fibonacci-heap', [max])
    : (comparator ? null : new native.HeapInner(max));
}

function comparisonTable(heap, extra = []) {
  const items = heap._inner.call('items', []).concat(extra);
  const comparisons = [];
  for (const left of items) {
    for (const right of items) {
      comparisons.push([left, right, Math.sign(heap._comparator(left, right))]);
    }
  }
  return comparisons;
}

function callCompared(heap, method, args = [], extra = []) {
  return heap._inner.call(method, [...args, comparisonTable(heap, extra)]);
}

function FibonacciHeap(comparator) {
  createHeap.call(this, false, comparator);
}

function MaxFibonacciHeap(comparator) {
  createHeap.call(this, true, comparator);
}

function attachMethods(Constructor) {
  Constructor.prototype.clear = function() {
    if (this._items) this._items.length = 0;
    else return protocol ? this._inner.call('clear', []) : this._inner.clear();
  };

  Constructor.prototype.push = function(item) {
    if (protocol && this._items === null && this._comparator !== defaultComparator) {
      return callCompared(this, 'pushCompared', [item], [item]);
    }
    if (!this._items) return protocol ? this._inner.call('push', [item]) : this._inner.push(item);
    this._items.push(item);
    return this._items.length;
  };

  Constructor.prototype.peek = function() {
    if (protocol && this._items === null && this._comparator !== defaultComparator) {
      return callCompared(this, 'peekCompared');
    }
    if (!this._items) return nullToUndefined(protocol ? this._inner.call('peek', []) : this._inner.peek());
    return select(this, false);
  };

  Constructor.prototype.pop = function() {
    if (protocol && this._items === null && this._comparator !== defaultComparator) {
      return callCompared(this, 'popCompared');
    }
    if (!this._items) return nullToUndefined(protocol ? this._inner.call('pop', []) : this._inner.pop());
    const index = select(this, true);
    return index === -1 ? undefined : this._items.splice(index, 1)[0];
  };

  Object.defineProperty(Constructor.prototype, 'size', {
    get: function() { return this._items ? this._items.length : (protocol ? this._inner.size() : this._inner.size); }
  });
}

function select(heap, indexOnly) {
  if (!heap._items.length) return indexOnly ? -1 : undefined;
  let best = 0;
  for (let i = 1; i < heap._items.length; i++) {
    const compared = heap._comparator(heap._items[i], heap._items[best]);
    if (heap._max ? compared > 0 : compared < 0) best = i;
  }
  return indexOnly ? best : heap._items[best];
}

function from(iterable, comparator, Constructor) {
  const heap = new Constructor(comparator);
  for (const item of iterable) heap.push(item);
  return heap;
}

attachMethods(FibonacciHeap);
attachMethods(MaxFibonacciHeap);

FibonacciHeap.from = function(iterable, comparator) {
  return from(iterable, comparator, FibonacciHeap);
};
MaxFibonacciHeap.from = function(iterable, comparator) {
  return from(iterable, comparator, MaxFibonacciHeap);
};
FibonacciHeap.MaxFibonacciHeap = MaxFibonacciHeap;

module.exports = FibonacciHeap;
