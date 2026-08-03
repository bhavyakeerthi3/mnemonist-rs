'use strict';

const SENTINEL = {};
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function compareTokens(a, b) {
  if (a === SENTINEL) return b === SENTINEL ? 0 : -1;
  if (b === SENTINEL) return 1;
  return a < b ? -1 : a > b ? 1 : 0;
}

function compareSuffixes(sequence, a, b) {
  while (a < sequence.length && b < sequence.length) {
    const comparison = compareTokens(sequence[a], sequence[b]);
    if (comparison) return comparison;
    a++;
    b++;
  }
  return a === sequence.length ? (b === sequence.length ? 0 : -1) : 1;
}

function SuffixArray(string) {
  if (typeof string !== 'string' && !Array.isArray(string)) {
    throw new Error('mnemonist/SuffixArray.constructor: expecting a string or array.');
  }
  this.string = string;
  this._inner = protocol && typeof string === 'string'
    ? protocol.create('suffix-array', [string])
    : null;
  if (this._inner) {
    this.length = this._inner.call('length', []);
    this.array = this._inner.call('array', []);
    return;
  }
  const sequence = typeof string === 'string' ? string.split('') : string;
  this.length = sequence.length;
  this.array = Array.from({ length: sequence.length }, (_, index) => index);
  this.array.sort((a, b) => compareSuffixes(sequence, a, b));
}

function GeneralizedSuffixArray(strings) {
  if (!Array.isArray(strings)) {
    throw new Error('mnemonist/GeneralizedSuffixArray.constructor: expecting an array.');
  }
  this.strings = strings;
  this.size = strings.length;
  this._inner = protocol && strings.every(string => typeof string === 'string')
    ? protocol.create('generalized-suffix-array', [strings])
    : null;
  if (this._inner) {
    this.length = this._inner.call('length', []);
    this.array = this._inner.call('array', []);
    return;
  }
  this._sequences = strings.map(string => typeof string === 'string' ? string.split('') : string.slice());
  this._sequenceIsString = strings.every(string => typeof string === 'string');
  this._combined = [];
  for (let i = 0; i < this._sequences.length; i++) {
    this._combined.push.apply(this._combined, this._sequences[i]);
    if (i + 1 < this._sequences.length) this._combined.push(SENTINEL);
  }
  this.length = this._combined.length;
  this.array = Array.from({ length: this.length }, (_, index) => index);
  this.array.sort((a, b) => compareSuffixes(this._combined, a, b));
}

GeneralizedSuffixArray.prototype.longestCommonSubsequence = function() {
  if (this._inner) return this._inner.call('longestCommonSubsequence', []);
  if (!this._sequences.length) return this._sequenceIsString ? '' : [];
  const first = this._sequences[0];
  let best = [];

  for (let start = 0; start < first.length; start++) {
    for (let end = start + 1; end <= first.length; end++) {
      const candidate = first.slice(start, end);
      if (candidate.length <= best.length) continue;
      if (this._sequences.every(sequence => contains(sequence, candidate))) best = candidate;
    }
  }

  return this._sequenceIsString ? best.join('') : best;
};

function contains(sequence, candidate) {
  outer: for (let start = 0; start <= sequence.length - candidate.length; start++) {
    for (let i = 0; i < candidate.length; i++) {
      if (sequence[start + i] !== candidate[i]) continue outer;
    }
    return true;
  }
  return false;
}

SuffixArray.GeneralizedSuffixArray = GeneralizedSuffixArray;

module.exports = SuffixArray;
