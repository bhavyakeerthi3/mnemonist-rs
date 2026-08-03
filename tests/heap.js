const { native, nullToUndefined } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol' ? require('../adapter/_protocol.js') : null;

function Heap(comparator) {
  if (comparator !== undefined && typeof comparator !== 'function') {
    throw new Error('mnemonist/Heap.constructor: given comparator should be a function.');
  }
  this.comparator = comparator || defaultComparator;
  this._max = false;
  this._customComparator = typeof comparator === 'function';
  this._items = null;
  this._inner = protocol
    ? protocol.create(this._customComparator ? 'comparator-heap' : 'heap', [false])
    : new native.HeapInner(false);
}

function MaxHeap(comparator) {
  if (comparator !== undefined && typeof comparator !== 'function') {
    throw new Error('mnemonist/MaxHeap.constructor: given comparator should be a function.');
  }
  this.comparator = comparator || defaultComparator;
  this._max = true;
  this._customComparator = typeof comparator === 'function';
  this._items = null;
  this._inner = protocol
    ? protocol.create(this._customComparator ? 'comparator-heap' : 'heap', [true])
    : new native.HeapInner(true);
}

const defaultComparator = (a, b) => {
  if (a < b) return -1;
  if (a > b) return 1;
  return 0;
};

function comparisonTable(heap, extra = []) {
  const items = heap._inner.call('items', []).concat(extra);
  const comparisons = [];
  for (const left of items) {
    for (const right of items) {
      comparisons.push([left, right, Math.sign(heap.comparator(left, right))]);
    }
  }
  return comparisons;
}

function callCompared(heap, method, args = [], extra = []) {
  return heap._inner.call(method, [...args, comparisonTable(heap, extra)]);
}

function attachHeapMethods(Constructor) {
  Constructor.prototype.clear = function() { if (this._items) this._items.length = 0; else return protocol ? this._inner.call('clear', []) : this._inner.clear(); };
  Constructor.prototype.push = function(item) {
    if (protocol && this._customComparator) return callCompared(this, 'pushCompared', [item], [item]);
    if (this._items) return this._items.push(item);
    return protocol ? this._inner.call('push', [item]) : this._inner.push(item);
  };
  Constructor.prototype.peek = function() {
    if (protocol && this._customComparator) return callCompared(this, 'peekCompared');
    if (this._items) return this._items.length ? this._items.reduce((best, item) => (this._max ? this.comparator(item, best) > 0 : this.comparator(item, best) < 0) ? item : best) : undefined;
    return nullToUndefined(protocol ? this._inner.call('peek', []) : this._inner.peek());
  };
  Constructor.prototype.pop = function() {
    if (protocol && this._customComparator) return callCompared(this, 'popCompared');
    if (this._customComparator) return popWithComparator(this, this.comparator);
    return nullToUndefined(protocol ? this._inner.call('pop', []) : this._inner.pop());
  };
  Constructor.prototype.replace = function(item) {
    if (protocol && this._customComparator) return callCompared(this, 'replaceCompared', [item], [item]);
    if (this._customComparator) return replaceWithComparator(this, item, this.comparator);
    return nullToUndefined(protocol ? this._inner.call('replace', [item]) : this._inner.replace(item));
  };
  Constructor.prototype.pushpop = function(item) {
    if (protocol && this._customComparator) return callCompared(this, 'pushpopCompared', [item], [item]);
    if (this._customComparator) return pushpopWithComparator(this, item, this.comparator);
    return protocol ? this._inner.call('pushpop', [item]) : this._inner.pushpop(item);
  };
  Constructor.prototype.consume = function() {
    if (protocol && this._customComparator) return callCompared(this, 'consumeCompared');
    if (this._customComparator) return consumeWithComparator(this, this.comparator);
    return protocol ? this._inner.call('consume', []) : this._inner.consume();
  };
  Constructor.prototype.toArray = function() {
    if (protocol && this._customComparator) return callCompared(this, 'toArrayCompared');
    if (this._customComparator) return consumeWithComparator(this, this.comparator);
    return protocol ? this._inner.call('toArray', []) : this._inner.toArray();
  };
  Object.defineProperty(Constructor.prototype, 'size', {
    get: function() { return this._items ? this._items.length : (protocol ? this._inner.size() : this._inner.size); }
  });
}

function popWithComparator(heap, compare) {
  if (heap.size === 0) return undefined;
  const items = heapItems(heap);
  let best = 0;
  for (let i = 1; i < items.length; i++) {
    if (compare(items[i], items[best]) < 0) best = i;
  }
  const value = items[best];
  items.splice(best, 1);
  syncItems(heap, items);
  return value;
}

function replaceWithComparator(heap, item, compare) {
  if (heap.size === 0) throw new Error('mnemonist/heap.replace: cannot pop an empty heap.');
  const popped = heap.peek();
  const items = heapItems(heap);
  items[0] = item;
  syncItems(heap, items);
  return popped;
}

function pushpopWithComparator(heap, item, compare) {
  if (heap.size === 0) return item;
  const top = heap.peek();
  if (compare(top, item) < 0) {
    heap.pop();
    heap.push(top);
    return item;
  }
  return item;
}

function consumeWithComparator(heap, compare) {
  const items = heapItems(heap);
  items.sort(compare);
  heap.clear();
  return items;
}

function heapItems(heap) {
  return heap._items ? heap._items.slice() : (protocol ? heap._inner.call('toArray', []) : heap._inner.toArray());
}

function syncItems(heap, items) {
  if (heap._items) heap._items = items;
  else {
    heap.clear();
    for (const item of items) heap.push(item);
  }
}

// --- Array-based static heap utilities (Heap.heapify / Heap.consume) ---
//
// These operate directly on a plain array using a standard binary-heap
// layout, mirroring upstream mnemonist's static free functions. They don't
// go through the native bridge since they're pure array algorithms.

function siftDown(array, compare, start, end) {
  let root = start;
  for (;;) {
    let child = root * 2 + 1;
    if (child > end) break;
    if (child + 1 <= end && compare(array[child], array[child + 1]) > 0) child++;
    if (compare(array[root], array[child]) > 0) {
      const tmp = array[root];
      array[root] = array[child];
      array[child] = tmp;
      root = child;
    } else {
      break;
    }
  }
}

function heapifyArray(compare, array) {
  const n = array.length;
  for (let i = Math.floor(n / 2) - 1; i >= 0; i--) {
    siftDown(array, compare, i, n - 1);
  }
  return array;
}

function consumeArray(compare, array) {
  const result = [];
  let end = array.length - 1;
  while (end >= 0) {
    result.push(array[0]);
    array[0] = array[end];
    end--;
    siftDown(array, compare, 0, end);
  }
  array.length = 0;
  return result;
}

attachHeapMethods(Heap);
attachHeapMethods(MaxHeap);

Heap.from = function(iterable, comparator) {
  if (typeof comparator === 'function') {
    const heap = new Heap(comparator);
    for (const value of iterable) heap.push(value);
    return heap;
  }
  const heap = new Heap();
  if (protocol) for (const value of iterable) heap.push(value);
  else heap._inner = native.heapFrom(Array.from(iterable), false);
  return heap;
};

MaxHeap.from = function(iterable, comparator) {
  if (typeof comparator === 'function') {
    const heap = new MaxHeap(comparator);
    for (const value of iterable) heap.push(value);
    return heap;
  }
  const heap = new MaxHeap();
  if (protocol) for (const value of iterable) heap.push(value);
  else heap._inner = native.heapFrom(Array.from(iterable), true);
  return heap;
};

Heap.heapify = function(comparator, array) {
  return heapifyArray(comparator, array);
};

Heap.consume = function(comparator, array) {
  return consumeArray(comparator, array);
};

function parseSelectionArgs(args) {
  if (typeof args[0] === 'function') {
    return { comparator: args[0], n: args[1], iterable: args[2] };
  }
  return { comparator: defaultComparator, n: args[0], iterable: args[1] };
}

Heap.nsmallest = function() {
  const { comparator, n, iterable } = parseSelectionArgs(arguments);
  if (comparator === defaultComparator) {
    return protocol ? Array.from(iterable).sort(defaultComparator).slice(0, n) : native.heapNsmallest(n, Array.from(iterable));
  }
  return Array.from(iterable).sort(comparator).slice(0, n);
};

Heap.nlargest = function() {
  const { comparator, n, iterable } = parseSelectionArgs(arguments);
  if (comparator === defaultComparator) {
    return protocol ? Array.from(iterable).sort((a, b) => -defaultComparator(a, b)).slice(0, n) : native.heapNlargest(n, Array.from(iterable));
  }
  return Array.from(iterable).sort((a, b) => -comparator(a, b)).slice(0, n);
};

Heap.MinHeap = Heap;
Heap.MaxHeap = MaxHeap;

module.exports = Heap;
module.exports.MaxHeap = MaxHeap;
