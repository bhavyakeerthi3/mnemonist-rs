'use strict';

const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function CritBitTreeMap(capacity) {
  this._inner = protocol ? protocol.create(
    capacity === undefined ? 'critbit-tree-map' : 'fixed-critbit-tree-map',
    capacity === undefined ? [] : [capacity]
  ) : null;
  this._items = this._inner ? null : new Map();
}

CritBitTreeMap.prototype.clear = function() {
  if (this._inner) return this._inner.call('clear', []);
  this._items.clear();
};

CritBitTreeMap.prototype.set = function(key, value) {
  if (this._inner) {
    this._inner.call('set', [key, value]);
    return this;
  }
  this._items.set(key, value);
  return this;
};

CritBitTreeMap.prototype.get = function(key) {
  if (this._inner) return this._inner.call('get', [key]);
  return this._items.get(key);
};

CritBitTreeMap.prototype.has = function(key) {
  if (this._inner) return this._inner.call('has', [key]);
  return this._items.has(key);
};

CritBitTreeMap.prototype.delete = function(key) {
  if (this._inner) return this._inner.call('delete', [key]);
  return this._items.delete(key);
};

CritBitTreeMap.prototype.forEach = function(callback, scope) {
  const context = arguments.length > 1 ? scope : this;
  if (this._inner) {
    const entries = this._inner.call('entries', []);
    for (let i = 0; i < entries.length; i++) callback.call(context, entries[i][1], entries[i][0], this);
    return;
  }
  for (const [key, value] of sortedEntries(this._items)) callback.call(context, value, key, this);
};

CritBitTreeMap.prototype.keys = function() {
  if (this._inner) return this._inner.call('entries', []).map(entry => entry[0])[Symbol.iterator]();
  return sortedEntries(this._items).map(entry => entry[0])[Symbol.iterator]();
};

CritBitTreeMap.prototype.values = function() {
  if (this._inner) return this._inner.call('entries', []).map(entry => entry[1])[Symbol.iterator]();
  return sortedEntries(this._items).map(entry => entry[1])[Symbol.iterator]();
};

CritBitTreeMap.prototype.entries = function() {
  if (this._inner) return this._inner.call('entries', [])[Symbol.iterator]();
  return sortedEntries(this._items)[Symbol.iterator]();
};

CritBitTreeMap.prototype[Symbol.iterator] = CritBitTreeMap.prototype.entries;

Object.defineProperty(CritBitTreeMap.prototype, 'size', {
  get: function() { return this._inner ? this._inner.call('size', []) : this._items.size; }
});

function sortedEntries(items) {
  return Array.from(items.entries()).sort((a, b) => a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0);
}

module.exports = CritBitTreeMap;
