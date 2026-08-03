const { native, nullToUndefined } = require('../adapter/_helpers.js');
const forEach = require('obliterator/foreach');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

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

function LRUMapWithDelete(keyType, valueType, capacity) {
  let cap = capacity;
  if (arguments.length === 1) {
    cap = keyType;
  } else if (arguments.length === 3) {
    cap = capacity;
  }

  if (typeof cap !== 'number' || cap <= 0 || !Number.isInteger(cap) || cap === Infinity) {
    throw new Error('LRUMapWithDelete: capacity must be a positive integer.');
  }

  this._inner = protocol
    ? protocol.create('lru-cache', [cap])
    : new native.LruCacheInner(cap);
  this._values = protocol ? null : new Map();
}

LRUMapWithDelete.prototype.set = function(key, value) {
  if (protocol) return this._inner.callOpaque('set', [key, value]);
  const evictedKey = this._inner.size === this._inner.capacity && !this._inner.has(key) ?
    this._inner.entries()[this._inner.size - 1][0] : undefined;
  this._inner.set(key, value === undefined ? null : value);
  if (evictedKey !== undefined) this._values.delete(evictedKey);
  this._values.set(key, value);
};

LRUMapWithDelete.prototype.setpop = function(key, value) {
  if (protocol) {
    const result = this._inner.callOpaque('setpop', [key, value]);
    return result === undefined ? null : result;
  }
  const previousValue = this._values.get(key);
  const popResult = this._inner.setpop(key, value === undefined ? null : value);
  if (!popResult) {
    this._values.set(key, value);
    return null;
  }
  const displacedValue = popResult.evicted ? this._values.get(popResult.key) : previousValue;
  if (popResult.evicted) this._values.delete(popResult.key);
  this._values.set(key, value);
  return {
    evicted: popResult.evicted,
    key: popResult.key,
    value: displacedValue
  };
};

LRUMapWithDelete.prototype.get = function(key) {
  if (protocol) return this._inner.callOpaque('get', [key]);
  if (!this._inner.has(key)) return undefined;
  this._inner.get(key);
  return this._values.get(key);
};

LRUMapWithDelete.prototype.peek = function(key) {
  if (protocol) return this._inner.callOpaque('peek', [key]);
  if (!this._inner.has(key)) return undefined;
  return this._values.get(key);
};

LRUMapWithDelete.prototype.has = function(key) {
  return protocol ? this._inner.callOpaque('has', [key]) : this._inner.has(key);
};

LRUMapWithDelete.prototype.delete = function(key) {
  if (protocol) return this._inner.callOpaque('delete', [key]);
  const deleted = this._inner.delete(key);
  if (deleted) this._values.delete(key);
  return deleted;
};

LRUMapWithDelete.prototype.remove = function(key, missingMarker) {
  if (protocol) {
    if (!this.has(key)) return missingMarker;
    return this._inner.callOpaque('remove', [key]);
  }
  if (!this._inner.has(key)) {
    return missingMarker;
  }
  this._inner.remove(key);
  const value = this._values.get(key);
  this._values.delete(key);
  return value;
};

LRUMapWithDelete.prototype.clear = function() {
  if (protocol) return this._inner.call('clear', []);
  this._inner.clear();
  this._values.clear();
};

LRUMapWithDelete.prototype.keys = function() {
  return iteratorFrom(protocol ? this._inner.call('keys', []) : this._inner.keys());
};

LRUMapWithDelete.prototype.values = function() {
  return iteratorFrom(protocol ? this._inner.call('values', []) : this._inner.entries().map(entry => this._values.get(entry[0])));
};

LRUMapWithDelete.prototype.entries = function() {
  return iteratorFrom(protocol ? this._inner.call('entries', []) : this._inner.entries().map(entry => [entry[0], this._values.get(entry[0])]));
};

LRUMapWithDelete.prototype.forEach = function(callback, scope) {
  scope = arguments.length > 1 ? scope : this;
  const list = protocol ? this._inner.call('entries', []) : this._inner.entries();
  for (let i = 0; i < list.length; i++) {
    callback.call(scope, protocol ? list[i][1] : this._values.get(list[i][0]), list[i][0], this);
  }
};

LRUMapWithDelete.prototype[Symbol.iterator] = function() {
  return this.entries();
};

Object.defineProperty(LRUMapWithDelete.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

Object.defineProperty(LRUMapWithDelete.prototype, 'capacity', {
  get: function() { return protocol ? this._inner.call('capacity', []) : this._inner.capacity; }
});

Object.defineProperty(LRUMapWithDelete.prototype, 'head', {
  get: function() {
    if (this.size === 0) return 0;
    const entries = protocol ? this._inner.call('entries', []) : this._inner.entries();
    return entries[0][0];
  }
});

Object.defineProperty(LRUMapWithDelete.prototype, 'tail', {
  get: function() {
    if (this.size === 0) return 0;
    const entries = protocol ? this._inner.call('entries', []) : this._inner.entries();
    return entries[entries.length - 1][0];
  }
});

Object.defineProperty(LRUMapWithDelete.prototype, 'items', {
  get: function() {
    const map = new Map();
    const entries = protocol ? this._inner.call('entries', []) : this._inner.entries();
    for (let i = 0; i < entries.length; i++) {
      map.set(entries[i][0], { value: entries[i][1] });
    }
    return map;
  }
});

LRUMapWithDelete.from = function(iterable, keyType, valueType, capacity) {
  let cap = capacity;
  if (arguments.length <= 2) {
    cap = keyType;
  }
  const items = [];
  forEach(iterable, (value, key) => {
    items.push([key, value]);
  });
  if (cap === undefined) {
    cap = items.length;
  }
  const cache = new LRUMapWithDelete(cap);
  for (let i = 0; i < items.length; i++) {
    cache.set(items[i][0], items[i][1]);
  }
  return cache;
};

module.exports = LRUMapWithDelete;
