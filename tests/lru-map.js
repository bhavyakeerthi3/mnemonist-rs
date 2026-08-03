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

function LRUMap(keyType, valueType, capacity) {
  let cap = capacity;
  if (arguments.length === 1) {
    cap = keyType;
  } else if (arguments.length === 3) {
    cap = capacity;
  }

  if (typeof cap !== 'number' || cap <= 0 || !Number.isInteger(cap) || cap === Infinity) {
    throw new Error('LRUMap: capacity must be a positive integer.');
  }

  this._inner = protocol ? protocol.create('lru-cache', [cap]) : new native.LruCacheInner(cap);
}

LRUMap.prototype.set = function(key, value) {
  if (protocol) return this._inner.callOpaque('set', [key, value]);
  this._inner.set(key, value);
};

LRUMap.prototype.setpop = function(key, value) {
  if (protocol) {
    const result = this._inner.callOpaque('setpop', [key, value]);
    return result === undefined ? null : result;
  }
  const popResult = this._inner.setpop(key, value);
  if (!popResult) return null;
  return {
    evicted: popResult.evicted,
    key: popResult.key,
    value: popResult.value
  };
};

LRUMap.prototype.get = function(key) {
  if (protocol) return this._inner.callOpaque('get', [key]);
  if (!this._inner.has(key)) return undefined;
  return this._inner.get(key);
};

LRUMap.prototype.peek = function(key) {
  if (protocol) return this._inner.callOpaque('peek', [key]);
  if (!this._inner.has(key)) return undefined;
  return this._inner.peek(key);
};

LRUMap.prototype.has = function(key) {
  if (protocol) return this._inner.callOpaque('has', [key]);
  return this._inner.has(key);
};

LRUMap.prototype.clear = function() {
  if (protocol) return this._inner.call('clear', []);
  this._inner.clear();
};

LRUMap.prototype.keys = function() {
  if (protocol) return iteratorFrom(this._inner.call('keys', []));
  return iteratorFrom(this._inner.keys());
};

LRUMap.prototype.values = function() {
  if (protocol) return iteratorFrom(this._inner.call('values', []));
  return iteratorFrom(this._inner.values());
};

LRUMap.prototype.entries = function() {
  if (protocol) return iteratorFrom(this._inner.call('entries', []));
  return iteratorFrom(this._inner.entries());
};

LRUMap.prototype.forEach = function(callback, scope) {
  scope = arguments.length > 1 ? scope : this;
  if (protocol) {
    const list = this._inner.call('entries', []);
    for (let i = 0; i < list.length; i++) callback.call(scope, list[i][1], list[i][0], this);
    return;
  }
  const list = this._inner.entries();
  for (let i = 0; i < list.length; i++) {
    callback.call(scope, list[i][1], list[i][0], this);
  }
};

LRUMap.prototype[Symbol.iterator] = function() {
  return this.entries();
};

Object.defineProperty(LRUMap.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

Object.defineProperty(LRUMap.prototype, 'capacity', {
  get: function() { return protocol ? this._inner.call('capacity', []) : this._inner.capacity; }
});

Object.defineProperty(LRUMap.prototype, 'head', {
  get: function() {
    if (this.size === 0) return 0;
    const entries = protocol ? this._inner.call('entries', []) : this._inner.entries();
    return entries[0][0];
  }
});

Object.defineProperty(LRUMap.prototype, 'tail', {
  get: function() {
    if (this.size === 0) return 0;
    const entries = protocol ? this._inner.call('entries', []) : this._inner.entries();
    return entries[entries.length - 1][0];
  }
});

Object.defineProperty(LRUMap.prototype, 'items', {
  get: function() {
    const map = new Map();
    const entries = protocol ? this._inner.call('entries', []) : this._inner.entries();
    for (let i = 0; i < entries.length; i++) {
      map.set(entries[i][0], { value: entries[i][1] });
    }
    return map;
  }
});

LRUMap.from = function(iterable, keyType, valueType, capacity) {
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
  const cache = new LRUMap(cap);
  if (protocol) {
    for (let i = 0; i < items.length; i++) cache.set(items[i][0], items[i][1]);
  } else {
    cache._inner = native.lruCacheFrom(items, cap);
  }
  return cache;
};

module.exports = LRUMap;
