'use strict';

const forEach = require('obliterator/foreach');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

const SENTINEL = String.fromCharCode(0);

function TrieMap(Token) {
  this._arrayMode = Token === Array;
  this._inner = protocol && !this._arrayMode ? protocol.create('trie-map') : null;
  this.clear();
}

TrieMap.prototype.clear = function() {
  this.root = {};
  this.size = 0;
  if (this._inner) this._inner.call('clear', []);
};

TrieMap.prototype._syncRoot = function() {
  if (!this._inner) return;
  const root = {};
  const entries = this._inner.call('entries', ['']);
  for (let i = 0; i < entries.length; i++) {
    const entry = entries[i];
    let node = root;
    const tokens = Array.from(entry[0]);
    for (let j = 0; j < tokens.length; j++) node = node[tokens[j]] || (node[tokens[j]] = {});
    node[SENTINEL] = entry[1];
  }
  this.root = root;
  this.size = this._inner.call('size', []);
};

TrieMap.prototype._tokens = function(prefix) {
  return this._arrayMode ? prefix : Array.from(prefix);
};

TrieMap.prototype._prefix = function(tokens) {
  return this._arrayMode ? tokens : tokens.join('');
};

TrieMap.prototype._node = function(prefix) {
  let node = this.root;
  const tokens = this._tokens(prefix);
  for (let i = 0; i < tokens.length; i++) {
    node = node[tokens[i]];
    if (!node) return null;
  }
  return node;
};

TrieMap.prototype.set = function(prefix, value) {
  if (this._inner) {
    this._inner.call('set', [prefix, value]);
    this._syncRoot();
    return this;
  }
  let node = this.root;
  const tokens = this._tokens(prefix);
  for (let i = 0; i < tokens.length; i++) {
    const token = tokens[i];
    node = node[token] || (node[token] = {});
  }
  if (!Object.prototype.hasOwnProperty.call(node, SENTINEL)) this.size++;
  node[SENTINEL] = value;
  return this;
};

TrieMap.prototype.get = function(prefix) {
  if (this._inner && typeof prefix !== 'string') return undefined;
  if (this._inner) return this._inner.call('get', [prefix]);
  const node = this._node(prefix);
  return node && Object.prototype.hasOwnProperty.call(node, SENTINEL) ? node[SENTINEL] : undefined;
};

TrieMap.prototype.has = function(prefix) {
  if (this._inner && typeof prefix !== 'string') return false;
  if (this._inner) return this._inner.call('has', [prefix]);
  const node = this._node(prefix);
  return !!node && Object.prototype.hasOwnProperty.call(node, SENTINEL);
};

TrieMap.prototype.update = function(prefix, updater) {
  return this.set(prefix, updater(this.get(prefix)));
};

TrieMap.prototype.delete = function(prefix) {
  if (this._inner && typeof prefix !== 'string') return false;
  if (this._inner) {
    const deleted = this._inner.call('delete', [prefix]);
    if (deleted) this._syncRoot();
    return deleted;
  }
  const tokens = this._tokens(prefix);
  const trail = [];
  let node = this.root;
  for (let i = 0; i < tokens.length; i++) {
    const token = tokens[i];
    if (!node[token]) return false;
    trail.push([node, token]);
    node = node[token];
  }
  if (!Object.prototype.hasOwnProperty.call(node, SENTINEL)) return false;
  delete node[SENTINEL];
  this.size--;
  for (let i = trail.length - 1; i >= 0 && Object.keys(node).length === 0; i--) {
    const parent = trail[i][0];
    delete parent[trail[i][1]];
    node = parent;
  }
  return true;
};

TrieMap.prototype._entries = function(prefix) {
  if (this._inner) return this._inner.call('entries', [prefix]);
  const tokens = this._tokens(prefix);
  const node = this._node(prefix);
  if (!node) return [];
  const results = [];
  const stack = [[node, tokens]];
  while (stack.length) {
    const current = stack.pop();
    const currentNode = current[0];
    const currentTokens = current[1];
    if (Object.prototype.hasOwnProperty.call(currentNode, SENTINEL)) {
      results.push([this._prefix(currentTokens), currentNode[SENTINEL]]);
    }
    const keys = Object.keys(currentNode);
    for (let i = 0; i < keys.length; i++) {
      if (keys[i] !== SENTINEL) stack.push([currentNode[keys[i]], currentTokens.concat(keys[i])]);
    }
  }
  return results;
};

TrieMap.prototype.find = function(prefix) {
  return this._entries(prefix);
};

TrieMap.prototype.keys = function(prefix) {
  return this._entries(prefix === undefined ? (this._arrayMode ? [] : '') : prefix)
    .map(entry => entry[0])[Symbol.iterator]();
};

TrieMap.prototype.prefixes = function(prefix) {
  return this.keys(prefix);
};

TrieMap.prototype.values = function(prefix) {
  return this._entries(prefix === undefined ? (this._arrayMode ? [] : '') : prefix)
    .map(entry => entry[1])[Symbol.iterator]();
};

TrieMap.prototype.entries = function(prefix) {
  return this._entries(prefix === undefined ? (this._arrayMode ? [] : '') : prefix)[Symbol.iterator]();
};

TrieMap.prototype[Symbol.iterator] = TrieMap.prototype.entries;
TrieMap.SENTINEL = SENTINEL;

TrieMap.from = function(iterable, Token) {
  const trie = new TrieMap(Token);
  forEach(iterable, (value, key) => trie.set(key, value));
  return trie;
};

module.exports = TrieMap;
