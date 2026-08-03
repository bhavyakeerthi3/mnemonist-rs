'use strict';

const standardLevenshtein = require('leven');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function comparator(a, b) {
  return a.length > b.length ? -1 : a.length < b.length ? 1 : a < b ? -1 : a > b ? 1 : 0;
}

function partition(k, length) {
  const segments = k + 1;
  const small = length / segments | 0;
  const large = small + 1;
  const largeCount = length - small * segments;
  const smallCount = segments - largeCount;
  const result = new Array(segments);
  let i = 0;
  for (; i < smallCount; i++) result[i] = [i * small, small];
  const offset = i * small;
  for (let j = 0; j < largeCount; j++) result[i + j] = [offset + j * large, large];
  return result;
}

function segments(k, string) {
  return partition(k, string.length).map(part => string.slice(part[0], part[0] + part[1]));
}

function segmentPos(k, i, string) {
  return partition(k, string.length)[i][0];
}

function multiMatchAwareInterval(k, delta, i, s, pi, li) {
  const start1 = pi - i;
  const end1 = pi + i;
  const remaining = k - i;
  const start2 = pi + delta - remaining;
  const end2 = pi + delta + remaining;
  return [Math.max(0, start1, start2), Math.min(end1, end2, s - li)];
}

function multiMatchAwareSubstrings(k, string, length, i, pi, li) {
  const interval = multiMatchAwareInterval(k, string.length - length, i, string.length, pi, li);
  const result = [];
  let previous = null;
  for (let start = interval[0]; start <= interval[1]; start++) {
    const substring = string.slice(start, start + li);
    if (substring !== previous) result.push(substring);
    previous = substring;
  }
  return result;
}

function PassjoinIndex(levenshtein, k) {
  if (typeof levenshtein !== 'function') {
    throw new Error('mnemonist/passjoin-index: `levenshtein` should be a function returning edit distance between two strings.');
  }
  if (typeof k !== 'number' || k < 1) throw new Error('mnemonist/passjoin-index: `k` should be a number > 0');
  this.levenshtein = levenshtein;
  this.k = k;
  this._inner = protocol && levenshtein === standardLevenshtein
    ? protocol.create('passjoin-index', [k])
    : null;
  this.clear();
}

PassjoinIndex.prototype.clear = function() {
  this.size = 0;
  this.strings = [];
  if (this._inner) this._inner.call('clear', []);
};
PassjoinIndex.prototype.add = function(value) {
  if (this._inner) {
    this._inner.call('add', [value]);
    this.size = this._inner.size();
    return this;
  }
  this.strings.push(value);
  this.size++;
  return this;
};
PassjoinIndex.prototype.search = function(query) {
  if (this._inner) return new Set(this._inner.call('search', [query]));
  const matches = new Set();
  for (const candidate of this.strings) {
    if (Math.abs(candidate.length - query.length) <= this.k && this.levenshtein(query, candidate) <= this.k) matches.add(candidate);
  }
  return matches;
};
PassjoinIndex.prototype.forEach = function(callback, scope) {
  const context = arguments.length > 1 ? scope : this;
  if (this._inner) {
    this._inner.call('values', []).forEach((value, index) => callback.call(context, value, index, this));
    return;
  }
  this.strings.forEach((value, index) => callback.call(context, value, index, this));
};
PassjoinIndex.prototype.values = function() {
  return this._inner ? this._inner.call('values', []).values() : this.strings.values();
};
PassjoinIndex.prototype[Symbol.iterator] = PassjoinIndex.prototype.values;
PassjoinIndex.from = function(iterable, levenshtein, k) {
  const index = new PassjoinIndex(levenshtein, k);
  for (const value of iterable) index.add(value);
  return index;
};

PassjoinIndex.comparator = protocol
  ? (a, b) => protocol.invoke('passjoin-index', 'comparator', [a, b])
  : comparator;
PassjoinIndex.partition = protocol
  ? (k, length) => protocol.invoke('passjoin-index', 'partition', [k, length])
  : partition;
PassjoinIndex.segments = protocol
  ? (k, string) => protocol.invoke('passjoin-index', 'segments', [k, string])
  : segments;
PassjoinIndex.segmentPos = protocol
  ? (k, i, string) => protocol.invoke('passjoin-index', 'segmentPos', [k, i, string])
  : segmentPos;
PassjoinIndex.multiMatchAwareInterval = protocol
  ? (k, delta, i, s, pi, li) => protocol.invoke('passjoin-index', 'multiMatchAwareInterval', [k, delta, i, s, pi, li])
  : multiMatchAwareInterval;
PassjoinIndex.multiMatchAwareSubstrings = protocol
  ? (k, string, length, i, pi, li) => protocol.invoke('passjoin-index', 'multiMatchAwareSubstrings', [k, string, length, i, pi, li])
  : multiMatchAwareSubstrings;

module.exports = PassjoinIndex;
