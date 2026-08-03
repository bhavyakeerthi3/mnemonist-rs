'use strict';

const damerauLevenshtein = require('damerau-levenshtein');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function item(value) {
  const suggestions = new Set();
  if (typeof value === 'number') suggestions.add(value);
  return { suggestions, count: 0 };
}

function suggestion(term, distance, count) {
  return { term, distance, count };
}

function edits(word, distance, max, deletes) {
  deletes = deletes || new Set();
  distance++;
  if (word.length > 1) {
    for (let i = 0; i < word.length; i++) {
      const deleted = word.substring(0, i) + word.substring(i + 1);
      if (!deletes.has(deleted)) {
        deletes.add(deleted);
        if (distance < max) edits(deleted, distance, max, deletes);
      }
    }
  }
  return deletes;
}

function addLowestDistance(words, verbosity, target, word, index, deleted) {
  const first = target.suggestions.values().next().value;
  if (verbosity < 2 && target.suggestions.size && words[first].length - deleted.length > word.length - deleted.length) {
    target.suggestions = new Set();
    target.count = 0;
  }
  if (verbosity === 2 || !target.suggestions.size || words[first].length - deleted.length >= word.length - deleted.length) {
    target.suggestions.add(index);
  }
}

function distance(a, b) {
  return damerauLevenshtein(a, b).steps;
}

function lookup(dictionary, words, verbosity, maxDistance, maxLength, input) {
  const length = input.length;
  if (length - maxDistance > maxLength) return [];
  const candidates = [input];
  const candidateSet = new Set();
  const suggestionSet = new Set();
  let suggestions = [];

  while (candidates.length) {
    const candidate = candidates.shift();
    if (verbosity < 2 && suggestions.length && length - candidate.length > suggestions[0].distance) break;
    let target = dictionary[candidate];
    if (target !== undefined) {
      if (typeof target === 'number') target = item(target);
      if (target.count > 0 && !suggestionSet.has(candidate)) {
        suggestionSet.add(candidate);
        suggestions.push(suggestion(candidate, length - candidate.length, target.count));
        if (verbosity < 2 && length === candidate.length) break;
      }
      target.suggestions.forEach(index => {
        const term = words[index];
        if (suggestionSet.has(term)) return;
        suggestionSet.add(term);
        const steps = input === term ? 0 : distance(term, input);
        if (verbosity < 2 && suggestions.length && suggestions[0].distance > steps) suggestions = [];
        if (verbosity < 2 && suggestions.length && steps > suggestions[0].distance) return;
        if (steps <= maxDistance) {
          const entry = dictionary[term];
          if (entry !== undefined) suggestions.push(suggestion(term, steps, entry.count));
        }
      });
    }
    if (length - candidate.length < maxDistance) {
      if (verbosity < 2 && suggestions.length && length - candidate.length >= suggestions[0].distance) continue;
      for (let i = 0; i < candidate.length; i++) {
        const deleted = candidate.substring(0, i) + candidate.substring(i + 1);
        if (!candidateSet.has(deleted)) {
          candidateSet.add(deleted);
          candidates.push(deleted);
        }
      }
    }
  }
  return verbosity === 0 ? suggestions.slice(0, 1) : suggestions;
}

function SymSpell(options) {
  options = options || {};
  this.maxDistance = typeof options.maxDistance === 'number' ? options.maxDistance : 2;
  this.verbosity = typeof options.verbosity === 'number' ? options.verbosity : 2;
  if (this.maxDistance <= 0) throw new Error('mnemonist/SymSpell.constructor: invalid `maxDistance` option. Should be a integer greater than 0.');
  if (![0, 1, 2].includes(this.verbosity)) throw new Error('mnemonist/SymSpell.constructor: invalid `verbosity` option. Should be either 0, 1 or 2.');
  this._inner = protocol ? protocol.create('symspell', [this.maxDistance, this.verbosity]) : null;
  this.clear();
}

SymSpell.prototype.clear = function() {
  this.size = 0;
  this.dictionary = Object.create(null);
  this.maxLength = 0;
  this.words = [];
  if (this._inner) this._inner.call('clear', []);
};

SymSpell.prototype.add = function(word) {
  if (this._inner) {
    this._inner.call('add', [word]);
    this.size = this._inner.size();
    return this;
  }
  let target = this.dictionary[word];
  if (target !== undefined) {
    if (typeof target === 'number') {
      target = item(target);
      this.dictionary[word] = target;
    }
    target.count++;
  }
  else {
    target = item();
    target.count = 1;
    this.dictionary[word] = target;
    this.maxLength = Math.max(this.maxLength, word.length);
  }
  if (target.count === 1) {
    const index = this.words.length;
    this.words.push(word);
    edits(word, 0, this.maxDistance).forEach(deleted => {
      let entry = this.dictionary[deleted];
      if (entry !== undefined) {
        if (typeof entry === 'number') {
          entry = item(entry);
          this.dictionary[deleted] = entry;
        }
        if (!entry.suggestions.has(index)) addLowestDistance(this.words, this.verbosity, entry, word, index, deleted);
      }
      else {
        this.dictionary[deleted] = index;
      }
    });
  }
  this.size++;
  return this;
};

SymSpell.prototype.search = function(input) {
  if (this._inner) return this._inner.call('search', [input]);
  return lookup(this.dictionary, this.words, this.verbosity, this.maxDistance, this.maxLength, input);
};

SymSpell.from = function(iterable, options) {
  const index = new SymSpell(options);
  for (const word of iterable) index.add(word);
  return index;
};

module.exports = SymSpell;
