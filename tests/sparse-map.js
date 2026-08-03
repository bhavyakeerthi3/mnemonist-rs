const { native, nullToUndefined } = require('../adapter/_helpers.js');
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

function SparseMap(typedArrayClass, capacity) {
  if (arguments.length > 1) {
    this._inner = protocol ? protocol.create('sparse-map', [capacity]) : new native.SparseMapInner(capacity);
  } else {
    this._inner = protocol ? protocol.create('sparse-map', [typedArrayClass]) : new native.SparseMapInner(typedArrayClass);
  }
}

SparseMap.prototype.set = function(key, value) {
  return protocol ? this._inner.call('set', [key, value]) : this._inner.set(key, value);
};

SparseMap.prototype.get = function(key) {
  return protocol ? this._inner.call('get', [key]) : nullToUndefined(this._inner.get(key));
};

SparseMap.prototype.has = function(key) {
  return protocol ? this._inner.call('has', [key]) : this._inner.has(key);
};

SparseMap.prototype.delete = function(key) {
  return protocol ? this._inner.call('delete', [key]) : this._inner.delete(key);
};

SparseMap.prototype.clear = function() {
  return protocol ? this._inner.call('clear', []) : this._inner.clear();
};

SparseMap.prototype.keys = function() {
  return iteratorFrom(protocol ? this._inner.call('keys', []) : this._inner.keys());
};

SparseMap.prototype.values = function() {
  return iteratorFrom(protocol ? this._inner.call('values', []) : this._inner.values());
};

SparseMap.prototype.entries = function() {
  return iteratorFrom(protocol ? this._inner.call('entries', []) : this._inner.entries());
};

SparseMap.prototype.forEach = function(callback, scope) {
  scope = arguments.length > 1 ? scope : this;
  const entries = protocol ? this._inner.call('entries', []) : this._inner.entries();
  for (let i = 0; i < entries.length; i++) {
    callback.call(scope, entries[i][1], entries[i][0], this);
  }
};

Object.defineProperty(SparseMap.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

Object.defineProperty(SparseMap.prototype, 'length', {
  get: function() { return protocol ? this._inner.call('length', []) : this._inner.length; }
});

module.exports = SparseMap;
