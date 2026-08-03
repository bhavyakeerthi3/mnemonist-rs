'use strict';

const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function DefaultMap(factory) {
  if (typeof factory !== 'function') {
    throw new Error('mnemonist/DefaultMap.constructor: expecting a function.');
  }

  this._factory = factory;
  this._items = new Map();
  this._inner = protocol ? protocol.create('default-map') : null;
}

DefaultMap.prototype.clear = function() {
  if (this._inner) this._inner.call('clear', []);
  this._items.clear();
};

DefaultMap.prototype.get = function(key) {
  if (this._inner) {
    if (this._inner.callOpaque('has', [key])) {
      return this._inner.callOpaque('peek', [key]);
    }
    const created = this._factory(key, this._inner.size());
    this._inner.callOpaque('set', [key, created]);
    return created;
  }
  if (!this._items.has(key)) {
    this._items.set(key, this._factory(key, this._items.size));
  }
  return this._items.get(key);
};

DefaultMap.prototype.peek = function(key) {
  if (this._inner) return this._inner.callOpaque('peek', [key]);
  return this._items.get(key);
};

DefaultMap.prototype.set = function(key, value) {
  if (this._inner) {
    this._inner.callOpaque('set', [key, value]);
    return this;
  }
  this._items.set(key, value);
  return this;
};

DefaultMap.prototype.has = function(key) {
  if (this._inner) return this._inner.callOpaque('has', [key]);
  return this._items.has(key);
};

DefaultMap.prototype.delete = function(key) {
  if (this._inner) return this._inner.callOpaque('delete', [key]);
  return this._items.delete(key);
};

DefaultMap.prototype.forEach = function(callback, scope) {
  if (this._inner) {
    const receiver = scope === undefined ? this : scope;
    this._inner.call('entries', []).forEach(([key, value]) => callback.call(receiver, value, key, this));
    return;
  }
  this._items.forEach(callback, scope === undefined ? this : scope);
};

DefaultMap.prototype.entries = function() {
  if (this._inner) return this._inner.call('entries', []).values();
  return this._items.entries();
};

DefaultMap.prototype.keys = function() {
  if (this._inner) return this._inner.call('keys', []).values();
  return this._items.keys();
};

DefaultMap.prototype.values = function() {
  if (this._inner) return this._inner.call('values', []).values();
  return this._items.values();
};

DefaultMap.prototype[Symbol.iterator] = DefaultMap.prototype.entries;

Object.defineProperty(DefaultMap.prototype, 'size', {
  get: function() { return this._inner ? this._inner.size() : this._items.size; }
});

DefaultMap.autoIncrement = function() {
  let next = 0;
  return function() { return next++; };
};

module.exports = DefaultMap;
