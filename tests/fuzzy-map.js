'use strict';

const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function hashes(descriptor) {
  const write = Array.isArray(descriptor) ? descriptor[0] : descriptor;
  const read = Array.isArray(descriptor) ? descriptor[1] : descriptor;
  if (write !== undefined && typeof write !== 'function' || read !== undefined && typeof read !== 'function') {
    throw new Error('mnemonist/FuzzyMap.constructor: invalid hash function given.');
  }
  return [write || identity, read || identity];
}

function identity(value) { return value; }

function FuzzyMap(descriptor) {
  const functions = hashes(descriptor);
  this.items = new Map();
  this.writeHashFunction = functions[0];
  this.readHashFunction = functions[1];
  this._inner = protocol ? protocol.create('fuzzy-map') : null;
  this.size = 0;
}

FuzzyMap.prototype.clear = function() {
  if (this._inner) this._inner.call('clear', []);
  this.items.clear();
  this.size = 0;
};
FuzzyMap.prototype.add = function(item) {
  if (this._inner) {
    this._inner.callOpaque('set', [this.writeHashFunction(item), item]);
    this.size = this._inner.size();
    return this;
  }
  this.items.set(this.writeHashFunction(item), item);
  this.size = this.items.size;
  return this;
};
FuzzyMap.prototype.set = function(key, item) {
  if (this._inner) {
    this._inner.callOpaque('set', [this.writeHashFunction(key), item]);
    this.size = this._inner.size();
    return this;
  }
  this.items.set(this.writeHashFunction(key), item);
  this.size = this.items.size;
  return this;
};
FuzzyMap.prototype.get = function(key) {
  return this._inner ? this._inner.callOpaque('peek', [this.readHashFunction(key)]) : this.items.get(this.readHashFunction(key));
};
FuzzyMap.prototype.has = function(key) {
  return this._inner ? this._inner.callOpaque('has', [this.readHashFunction(key)]) : this.items.has(this.readHashFunction(key));
};
FuzzyMap.prototype.forEach = function(callback, scope) {
  const context = arguments.length > 1 ? scope : this;
  if (this._inner) {
    this._inner.call('values', []).forEach(value => callback.call(context, value, value));
    return;
  }
  this.items.forEach(value => callback.call(context, value, value));
};
FuzzyMap.prototype.values = function() { return this._inner ? this._inner.call('values', []).values() : this.items.values(); };
FuzzyMap.prototype[Symbol.iterator] = FuzzyMap.prototype.values;
FuzzyMap.from = function(iterable, descriptor, useSet) {
  const map = new FuzzyMap(descriptor);
  if (useSet) {
    for (const [key, value] of iterable) map.set(key, value);
  }
  else {
    for (const value of iterable) map.add(value);
  }
  return map;
};

module.exports = FuzzyMap;
