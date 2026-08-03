const { native } = require('../adapter/_helpers.js');
const obliterator = require('obliterator');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol' ? require('../adapter/_protocol.js') : null;

function iteratorFrom(array) {
  let i = 0;
  return {
    next() {
      if (i >= array.length) return { done: true };
      return { value: array[i++], done: false };
    },
    [Symbol.iterator]() { return this; }
  };
}

function MultiSet() {
  this._inner = protocol ? protocol.create('multi-set') : new native.MultiSetInner();
}

MultiSet.prototype.clear = function() {
  return protocol ? this._inner.call('clear', []) : this._inner.clear();
};

MultiSet.prototype.add = function(item, count) {
  if (count !== undefined && typeof count !== 'number') {
    throw new TypeError('mnemonist/multi-set.add: given count is not a number.');
  }
  return protocol ? this._inner.call('add', count === undefined ? [item] : [item, count]) : this._inner.add(item, count);
};

MultiSet.prototype.set = function(item, count) {
  if (typeof count !== 'number') {
    throw new TypeError('mnemonist/multi-set.set: given count is not a number.');
  }
  return protocol ? this._inner.call('set', [item, count]) : this._inner.set(item, count);
};

MultiSet.prototype.has = function(item) {
  return protocol ? this._inner.call('has', [item]) : this._inner.has(item);
};

MultiSet.prototype.delete = function(item) {
  return protocol ? this._inner.call('delete', [item]) : this._inner.delete(item);
};

MultiSet.prototype.remove = function(item, count) {
  if (count !== undefined && typeof count !== 'number') {
    throw new TypeError('mnemonist/multi-set.remove: given count is not a number.');
  }
  return protocol ? this._inner.call('remove', count === undefined ? [item] : [item, count]) : this._inner.remove(item, count);
};

MultiSet.prototype.edit = function(from, to) {
  return protocol ? this._inner.call('edit', [from, to]) : this._inner.edit(from, to);
};

MultiSet.prototype.multiplicity = function(item) {
  return protocol ? this._inner.call('multiplicity', [item]) : this._inner.multiplicity(item);
};

MultiSet.prototype.frequency = function(item) {
  return protocol ? this._inner.call('frequency', [item]) : this._inner.frequency(item);
};

MultiSet.prototype.top = function(n) {
  return protocol ? this._inner.call('top', [n]) : this._inner.top(n);
};

MultiSet.prototype.values = function() {
  return iteratorFrom(protocol ? this._inner.call('values', []) : this._inner.values());
};

MultiSet.prototype.keys = function() {
  return iteratorFrom(protocol ? this._inner.call('keys', []) : this._inner.keys());
};

MultiSet.prototype.multiplicities = function() {
  return iteratorFrom(protocol ? this._inner.call('multiplicities', []) : this._inner.multiplicities());
};

MultiSet.prototype.forEach = function(callback, scope) {
  scope = arguments.length > 1 ? scope : this;
  const list = protocol ? this._inner.call('forEach', []) : this._inner.forEach();
  for (let i = 0; i < list.length; i++) {
    callback.call(scope, list[i][0], list[i][1], this);
  }
};

MultiSet.prototype.forEachMultiplicity = function(callback, scope) {
  scope = arguments.length > 1 ? scope : this;
  const entries = protocol ? this._inner.call('multiplicities', []) : this._inner.multiplicities();
  for (let i = 0; i < entries.length; i++) {
    callback.call(scope, entries[i][1], entries[i][0], this);
  }
};

MultiSet.prototype[Symbol.iterator] = function() {
  return this.values();
};

Object.defineProperty(MultiSet.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

Object.defineProperty(MultiSet.prototype, 'dimension', {
  get: function() { return protocol ? this._inner.call('dimension', []) : this._inner.dimension; }
});

MultiSet.from = function(iterable) {
  const items = [];
  obliterator.forEach(iterable, item => {
    items.push(item);
  });
  const set = new MultiSet();
  if (protocol) {
    for (let i = 0; i < items.length; i++) set.add(items[i]);
  } else {
    set._inner = native.multiSetFrom(items);
  }
  return set;
};

MultiSet.isSubset = function(a, b) {
  if (protocol) {
    const candidate = MultiSet.from(a.values());
    const target = MultiSet.from(b.values());
    for (const key of candidate.keys()) {
      if (candidate.multiplicity(key) > target.multiplicity(key)) return false;
    }
    return true;
  }
  return native.multiSetIsSubset(a._inner.values(), b._inner.values());
};

MultiSet.isSuperset = function(a, b) {
  return MultiSet.isSubset(b, a);
};

module.exports = MultiSet;
