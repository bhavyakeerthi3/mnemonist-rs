'use strict';

const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function identity(value) { return value; }

function hashes(descriptor) {
  const write = Array.isArray(descriptor) ? descriptor[0] : descriptor;
  const read = Array.isArray(descriptor) ? descriptor[1] : descriptor;
  if (write !== undefined && typeof write !== 'function' || read !== undefined && typeof read !== 'function') {
    throw new Error('mnemonist/FuzzyMultiMap.constructor: invalid hash function given.');
  }
  return [write || identity, read || identity];
}

function FuzzyMultiMap(descriptor, Container) {
  const functions = hashes(descriptor);
  this.Container = Container || Array;
  this.items = new Map();
  this.writeHashFunction = functions[0];
  this.readHashFunction = functions[1];
  this._inner = protocol ? protocol.create('fuzzy-multi-map', [this.Container === Set]) : null;
  this.size = 0;
  this.dimension = 0;
}

FuzzyMultiMap.prototype.clear = function() {
  if (this._inner) this._inner.call('clear', []);
  this.items.clear();
  this.size = 0;
  this.dimension = 0;
};

FuzzyMultiMap.prototype.add = function(item) {
  return this.set(item, item);
};

FuzzyMultiMap.prototype.set = function(key, item) {
  if (this._inner) {
    this._inner.callOpaque('set', [this.writeHashFunction(key), item]);
    this.size = this._inner.size();
    this.dimension = this._inner.call('dimension', []);
    return this;
  }
  key = this.writeHashFunction(key);
  let container = this.items.get(key);
  if (!container) {
    container = new this.Container();
    this.items.set(key, container);
    this.dimension++;
  }
  if (container instanceof Set) {
    const before = container.size;
    container.add(item);
    this.size += container.size - before;
  }
  else {
    container.push(item);
    this.size++;
  }
  return this;
};

FuzzyMultiMap.prototype.get = function(key) {
  const values = this._inner ? this._inner.callOpaque('get', [this.readHashFunction(key)]) : this.items.get(this.readHashFunction(key));
  return this._inner && values !== undefined && this.Container === Set ? new Set(values) : values;
};
FuzzyMultiMap.prototype.has = function(key) { return this._inner ? this._inner.callOpaque('has', [this.readHashFunction(key)]) : this.items.has(this.readHashFunction(key)); };
FuzzyMultiMap.prototype.forEach = function(callback, scope) {
  const context = arguments.length > 1 ? scope : this;
  if (this._inner) {
    this._inner.call('values', []).forEach(value => callback.call(context, value, value));
    return;
  }
  for (const container of this.items.values()) for (const value of container) callback.call(context, value, value);
};
FuzzyMultiMap.prototype.values = function*() {
  if (this._inner) {
    yield* this._inner.call('values', []);
    return;
  }
  for (const container of this.items.values()) yield* container;
};
FuzzyMultiMap.prototype[Symbol.iterator] = FuzzyMultiMap.prototype.values;
FuzzyMultiMap.from = function(iterable, descriptor, Container, useSet) {
  if (typeof Container === 'boolean') {
    useSet = Container;
    Container = Array;
  }
  const map = new FuzzyMultiMap(descriptor, Container);
  if (useSet) {
    for (const [key, value] of iterable) map.set(key, value);
  }
  else {
    for (const value of iterable) map.add(value);
  }
  return map;
};

module.exports = FuzzyMultiMap;
