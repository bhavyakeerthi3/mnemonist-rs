'use strict';

const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function DefaultWeakMap(factory) {
  if (typeof factory !== 'function') {
    throw new Error('mnemonist/DefaultWeakMap.constructor: expecting a function.');
  }

  this._factory = factory;
  this._items = new WeakMap();
  this._inner = protocol ? protocol.create('default-weak-map') : null;
}

DefaultWeakMap.prototype.clear = function() {
  if (this._inner) this._inner.callWeakMap('clear', []);
  this._items = new WeakMap();
};

DefaultWeakMap.prototype.get = function(key) {
  if (this._inner) {
    if (this._inner.callWeakMap('has', [key])) {
      return this._inner.callWeakMap('peek', [key]);
    }
    const created = this._factory(key);
    this._inner.callWeakMap('set', [key, created]);
    return created;
  }
  if (!this._items.has(key)) {
    this._items.set(key, this._factory(key));
  }
  return this._items.get(key);
};

DefaultWeakMap.prototype.peek = function(key) {
  if (this._inner) return this._inner.callWeakMap('peek', [key]);
  return this._items.get(key);
};

DefaultWeakMap.prototype.set = function(key, value) {
  if (this._inner) {
    this._inner.callWeakMap('set', [key, value]);
    return this;
  }
  this._items.set(key, value);
  return this;
};

DefaultWeakMap.prototype.has = function(key) {
  if (this._inner) return this._inner.callWeakMap('has', [key]);
  return this._items.has(key);
};

DefaultWeakMap.prototype.delete = function(key) {
  if (this._inner) return this._inner.callWeakMap('delete', [key]);
  return this._items.delete(key);
};

module.exports = DefaultWeakMap;
