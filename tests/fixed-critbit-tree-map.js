'use strict';

const CritBitTreeMap = require('./critbit-tree-map.js');

function FixedCritBitTreeMap(capacity) {
  if (!Number.isInteger(capacity) || capacity <= 0) {
    throw new Error('mnemonist/FixedCritBitTreeMap.constructor: capacity should be a positive integer.');
  }
  CritBitTreeMap.call(this, capacity);
  this.capacity = capacity;
}

FixedCritBitTreeMap.prototype = Object.create(CritBitTreeMap.prototype);
FixedCritBitTreeMap.prototype.constructor = FixedCritBitTreeMap;

FixedCritBitTreeMap.prototype.set = function(key, value) {
  if (this._inner) return CritBitTreeMap.prototype.set.call(this, key, value);
  if (!this._items.has(key) && this._items.size >= this.capacity) {
    throw new Error('mnemonist/FixedCritBitTreeMap.set: capacity exceeded.');
  }
  this._items.set(key, value);
  return this;
};

module.exports = FixedCritBitTreeMap;
