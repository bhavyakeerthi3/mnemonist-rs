const { native, nullToUndefined } = require('../adapter/_helpers.js');
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

function BiMap(isInverted, mainMap) {
  if (mainMap) {
    this._inner = mainMap._inner;
    this._isInverted = isInverted;
    this.inverse = mainMap;
  } else {
    this._inner = protocol ? protocol.create('bi-map') : new native.BiMapInner();
    this._isInverted = false;
    this.inverse = new BiMap(true, this);
  }
}

BiMap.prototype.set = function(key, value) {
  if (this._isInverted) {
    protocol ? this._inner.call('set', [value, key]) : this._inner.set(value, key);
  } else {
    protocol ? this._inner.call('set', [key, value]) : this._inner.set(key, value);
  }
};

BiMap.prototype.get = function(key) {
  if (this._isInverted) {
    return nullToUndefined(protocol ? this._inner.call('inverseGet', [key]) : this._inner.inverseGet(key));
  } else {
    return nullToUndefined(protocol ? this._inner.call('get', [key]) : this._inner.get(key));
  }
};

BiMap.prototype.has = function(key) {
  if (this._isInverted) {
    return protocol ? this._inner.call('inverseHas', [key]) : this._inner.inverseHas(key);
  } else {
    return protocol ? this._inner.call('has', [key]) : this._inner.has(key);
  }
};

BiMap.prototype.delete = function(key) {
  if (this._isInverted) {
    return protocol ? this._inner.call('inverseDelete', [key]) : (() => {
      const realKey = this._inner.inverseGet(key);
      return realKey !== null && realKey !== undefined ? this._inner.delete(realKey) : false;
    })();
  } else {
    return protocol ? this._inner.call('delete', [key]) : this._inner.delete(key);
  }
};

BiMap.prototype.clear = function() {
  return protocol ? this._inner.call('clear', []) : this._inner.clear();
};

BiMap.prototype.keys = function() {
  if (this._isInverted) {
    return iteratorFrom(protocol ? this._inner.call('inverseKeys', []) : this._inner.inverseKeys());
  } else {
    return iteratorFrom(protocol ? this._inner.call('keys', []) : this._inner.keys());
  }
};

BiMap.prototype.values = function() {
  if (this._isInverted) {
    return iteratorFrom(protocol ? this._inner.call('inverseValues', []) : this._inner.inverseValues());
  } else {
    return iteratorFrom(protocol ? this._inner.call('values', []) : this._inner.values());
  }
};

BiMap.prototype.entries = function() {
  if (this._isInverted) {
    return iteratorFrom(protocol ? this._inner.call('inverseEntries', []) : this._inner.inverseEntries());
  } else {
    return iteratorFrom(protocol ? this._inner.call('entries', []) : this._inner.entries());
  }
};

BiMap.prototype.forEach = function(callback, scope) {
  scope = arguments.length > 1 ? scope : this;
  const list = this._isInverted ? (protocol ? this._inner.call('inverseEntries', []) : this._inner.inverseEntries()) : (protocol ? this._inner.call('entries', []) : this._inner.entries());
  for (let i = 0; i < list.length; i++) {
    callback.call(scope, list[i][1], list[i][0], this);
  }
};

BiMap.prototype[Symbol.iterator] = function() {
  return this.entries();
};

Object.defineProperty(BiMap.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

BiMap.from = function(iterable) {
  const items = [];
  obliterator.forEach(iterable, (val, key) => {
    if (iterable instanceof Map) {
      items.push([key, val]);
    } else if (Array.isArray(val)) {
      items.push([val[0], val[1]]);
    } else {
      items.push([key, val]);
    }
  });
  const map = new BiMap();
  if (protocol) {
    for (let i = 0; i < items.length; i++) map.set(items[i][0], items[i][1]);
  } else {
    const inner = native.biMapFrom(items);
    map._inner = inner;
    map.inverse._inner = inner;
  }
  return map;
};

module.exports = BiMap;
