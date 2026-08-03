'use strict';

const forEach = require('obliterator/foreach');
const levenshtein = require('leven');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function BKTree(distance) {
  if (typeof distance !== 'function') {
    throw new Error('mnemonist/BKTree.constructor: expecting a distance function.');
  }
  this._distance = distance;
  this._inner = protocol && distance === levenshtein ? protocol.create('bk-tree') : null;
  this._items = [];
}

BKTree.prototype.add = function(item) {
  if (this._inner) {
    if (typeof item !== 'string') throw new Error('mnemonist/BKTree: standalone string mode requires strings.');
    this._inner.call('add', [item]);
    return this;
  }
  if (!this._items.includes(item)) this._items.push(item);
  return this;
};

BKTree.prototype.clear = function() {
  if (this._inner) this._inner.call('clear', []);
  this._items.length = 0;
};

BKTree.prototype.search = function(radius, query) {
  if (this._inner) {
    if (typeof query !== 'string') throw new Error('mnemonist/BKTree: standalone string mode requires strings.');
    return this._inner.call('search', [radius, query]);
  }
  const matches = [];
  for (let i = 0; i < this._items.length; i++) {
    const item = this._items[i];
    const distance = this._distance(item, query);
    if (distance <= radius) matches.push({ item, distance });
  }
  return matches.sort((a, b) => a.distance - b.distance);
};

Object.defineProperty(BKTree.prototype, 'size', {
  get: function() { return this._inner ? this._inner.size() : this._items.length; }
});

BKTree.from = function(iterable, distance) {
  const tree = new BKTree(distance);
  forEach(iterable, item => tree.add(item));
  return tree;
};

module.exports = BKTree;
