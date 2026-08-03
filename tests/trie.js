'use strict';

const forEach = require('obliterator/foreach');
const TrieMap = require('./trie-map.js');

function Trie(Token) {
  TrieMap.call(this, Token);
}

Trie.prototype = Object.create(TrieMap.prototype);
Trie.prototype.constructor = Trie;

Trie.prototype.add = function(prefix) {
  return TrieMap.prototype.set.call(this, prefix, true);
};

Trie.prototype.find = function(prefix) {
  return TrieMap.prototype._entries.call(this, prefix).map(entry => entry[0]);
};

Trie.prototype.keys = function(prefix) {
  return TrieMap.prototype.keys.call(this, prefix);
};

Trie.prototype.prefixes = Trie.prototype.keys;
Trie.prototype[Symbol.iterator] = Trie.prototype.keys;
Trie.SENTINEL = TrieMap.SENTINEL;

Trie.from = function(iterable, Token) {
  const trie = new Trie(Token);
  forEach(iterable, value => trie.add(value));
  return trie;
};

module.exports = Trie;
