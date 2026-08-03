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

function LRUCache(keyType, valueType, capacity) {
  let cap = capacity;
  if (arguments.length === 1) {
    cap = keyType;
  } else if (arguments.length === 3) {
    cap = capacity;
  }

  if (typeof cap !== 'number' || cap <= 0 || !Number.isInteger(cap) || cap === Infinity) {
    throw new Error('LRUCache: capacity must be a positive integer.');
  }

  this._inner = protocol
    ? protocol.create('lru-cache', [cap])
    : new native.LruCacheInner(cap);
}

LRUCache.prototype.set = function(key, value) {
  return protocol ? this._inner.callOpaque('set', [key, value]) : this._inner.set(key, value);
};

LRUCache.prototype.setpop = function(key, value) {
  const popResult = protocol
    ? this._inner.callOpaque('setpop', [key, value])
    : this._inner.setpop(key, value);
  if (!popResult) return null;
  return {
    evicted: popResult.evicted,
    key: popResult.key,
    value: popResult.value
  };
};

LRUCache.prototype.get = function(key) {
  if (protocol) return this._inner.callOpaque('get', [key]);
  if (!this._inner.has(key)) return undefined;
  return this._inner.get(key);
};

LRUCache.prototype.peek = function(key) {
  if (protocol) return this._inner.callOpaque('peek', [key]);
  if (!this._inner.has(key)) return undefined;
  return this._inner.peek(key);
};

LRUCache.prototype.has = function(key) {
  return protocol ? this._inner.callOpaque('has', [key]) : this._inner.has(key);
};

LRUCache.prototype.clear = function() {
  return protocol ? this._inner.call('clear', []) : this._inner.clear();
};

LRUCache.prototype.keys = function() {
  return iteratorFrom(protocol ? this._inner.call('keys', []) : this._inner.keys());
};

LRUCache.prototype.values = function() {
  return iteratorFrom(protocol ? this._inner.call('values', []) : this._inner.values());
};

LRUCache.prototype.entries = function() {
  return iteratorFrom(protocol ? this._inner.call('entries', []) : this._inner.entries());
};

LRUCache.prototype.forEach = function(callback, scope) {
  scope = arguments.length > 1 ? scope : this;
  const list = protocol ? this._inner.call('entries', []) : this._inner.entries();
  for (let i = 0; i < list.length; i++) {
    callback.call(scope, list[i][1], list[i][0], this);
  }
};

LRUCache.prototype[Symbol.iterator] = function() {
  return this.entries();
};

Object.defineProperty(LRUCache.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

Object.defineProperty(LRUCache.prototype, 'capacity', {
  get: function() { return protocol ? this._inner.call('capacity', []) : this._inner.capacity; }
});

Object.defineProperty(LRUCache.prototype, 'head', {
  get: function() {
    if (this.size === 0) return 0;
    const entries = protocol ? this._inner.call('entries', []) : this._inner.entries();
    return entries[0][0];
  }
});

Object.defineProperty(LRUCache.prototype, 'tail', {
  get: function() {
    if (this.size === 0) return 0;
    const entries = protocol ? this._inner.call('entries', []) : this._inner.entries();
    return entries[entries.length - 1][0];
  }
});

Object.defineProperty(LRUCache.prototype, 'items', {
  get: function() {
    const obj = {};
    const entries = protocol ? this._inner.call('entries', []) : this._inner.entries();
    for (let i = 0; i < entries.length; i++) {
      obj[entries[i][0]] = { value: entries[i][1] };
    }
    return obj;
  }
});

LRUCache.from = function(iterable, keyType, valueType, capacity) {
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
  const cache = new LRUCache(cap);
  if (protocol) {
    for (let i = 0; i < items.length; i++) cache.set(items[i][0], items[i][1]);
  } else {
    cache._inner = native.lruCacheFrom(items, cap);
  }
  return cache;
};

module.exports = LRUCache;
