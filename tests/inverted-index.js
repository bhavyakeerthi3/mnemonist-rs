'use strict';

const forEach = require('obliterator/foreach');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function identity(value) {
  return value;
}

function InvertedIndex(descriptor) {
  if (Array.isArray(descriptor)) {
    this._documentTokenizer = descriptor[0];
    this._queryTokenizer = descriptor[1];
  } else {
    this._documentTokenizer = descriptor;
    this._queryTokenizer = descriptor;
  }

  this._documentTokenizer = this._documentTokenizer || identity;
  this._queryTokenizer = this._queryTokenizer || identity;

  if (typeof this._documentTokenizer !== 'function') {
    throw new Error('mnemonist/InvertedIndex.constructor: document tokenizer is not a function.');
  }
  if (typeof this._queryTokenizer !== 'function') {
    throw new Error('mnemonist/InvertedIndex.constructor: query tokenizer is not a function.');
  }

  this._inner = protocol ? protocol.create('inverted-index') : null;
  this.clear();
}

InvertedIndex.prototype.clear = function() {
  this._documents = [];
  this._postings = new Map();
  if (this._inner) this._inner.call('clear', []);
};

InvertedIndex.prototype.add = function(document) {
  const tokens = this._documentTokenizer(document);
  if (!Array.isArray(tokens)) {
    throw new Error('mnemonist/InvertedIndex.add: tokenizer function should return an array of tokens.');
  }

  if (this._inner) {
    this._inner.call('add', [document, tokens]);
    return this;
  }

  const index = this._documents.length;
  this._documents.push(document);
  const seen = new Set();
  for (let i = 0; i < tokens.length; i++) {
    const token = tokens[i];
    if (seen.has(token)) continue;
    seen.add(token);

    let posting = this._postings.get(token);
    if (!posting) {
      posting = [];
      this._postings.set(token, posting);
    }
    posting.push(index);
  }

  return this;
};

InvertedIndex.prototype.get = function(query) {
  if (!this._inner && this._documents.length === 0) return [];

  const tokens = this._queryTokenizer(query);
  if (!Array.isArray(tokens)) {
    throw new Error('mnemonist/InvertedIndex.query: tokenizer function should return an array of tokens.');
  }
  if (tokens.length === 0) return [];

  if (this._inner) return this._inner.call('get', [tokens]);

  let matches = this._postings.get(tokens[0]);
  if (!matches || matches.length === 0) return [];
  matches = matches.slice();

  for (let i = 1; i < tokens.length; i++) {
    const posting = this._postings.get(tokens[i]);
    if (!posting || posting.length === 0) return [];
    const accepted = new Set(posting);
    matches = matches.filter(index => accepted.has(index));
  }

  return matches.map(index => this._documents[index]);
};

InvertedIndex.prototype.forEach = function(callback, scope) {
  const receiver = scope === undefined ? this : scope;
  if (this._inner) {
    const documents = this._inner.call('documents', []);
    for (let i = 0; i < documents.length; i++) callback.call(receiver, documents[i], i, this);
    return;
  }
  for (let i = 0; i < this._documents.length; i++) {
    callback.call(receiver, this._documents[i], i, this);
  }
};

InvertedIndex.prototype.documents = function() {
  if (this._inner) return this._inner.call('documents', []).values();
  return this._documents.values();
};

InvertedIndex.prototype.tokens = function() {
  if (this._inner) return this._inner.call('tokens', []).values();
  return this._postings.keys();
};

InvertedIndex.prototype[Symbol.iterator] = InvertedIndex.prototype.documents;

Object.defineProperty(InvertedIndex.prototype, 'size', {
  get: function() { return this._inner ? this._inner.call('size', []) : this._documents.length; }
});

Object.defineProperty(InvertedIndex.prototype, 'dimension', {
  get: function() { return this._inner ? this._inner.call('dimension', []) : this._postings.size; }
});

InvertedIndex.from = function(iterable, descriptor) {
  const index = new InvertedIndex(descriptor);
  forEach(iterable, document => index.add(document));
  return index;
};

module.exports = InvertedIndex;
