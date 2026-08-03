const { native, nullToUndefined } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function iteratorFrom(array) {
  let i = 0;
  return {
    next() {
      if (i >= array.length) return { done: true };
      return { value: array[i++], done: false };
    },
    [Symbol.iterator]() { return this; }
  };
}

function BitSet(length) {
  this._inner = protocol ? protocol.create('bit-set', [length]) : new native.BitSetInner(length);
}

BitSet.prototype.set = function(index, value) {
  if (value === undefined) {
    value = true;
  } else {
    value = !!value;
  }
  return protocol ? this._inner.call('set', [index, value]) : this._inner.set(index, value);
};

BitSet.prototype.get = function(index) {
  return protocol ? this._inner.call('get', [index]) : this._inner.get(index);
};

BitSet.prototype.test = function(index) {
  return protocol ? this._inner.call('test', [index]) : this._inner.test(index);
};

BitSet.prototype.reset = function(index) {
  return protocol ? this._inner.call('reset', [index]) : this._inner.reset(index);
};

BitSet.prototype.flip = function(index) {
  return protocol ? this._inner.call('flip', [index]) : this._inner.flip(index);
};

BitSet.prototype.rank = function(index) {
  return protocol ? this._inner.call('rank', [index]) : this._inner.rank(index);
};

BitSet.prototype.select = function(r) {
  return protocol ? this._inner.call('select', [r]) : nullToUndefined(this._inner.select(r));
};

BitSet.prototype.clear = function() {
  return protocol ? this._inner.call('clear', []) : this._inner.clear();
};

BitSet.prototype.values = function() {
  return iteratorFrom(protocol ? this._inner.call('values', []) : this._inner.values());
};

BitSet.prototype.entries = function() {
  return iteratorFrom(protocol ? this._inner.call('entries', []) : this._inner.entries());
};

BitSet.prototype.forEach = function(callback, scope) {
  scope = arguments.length > 1 ? scope : this;
  const values = protocol ? this._inner.call('values', []) : this._inner.values();
  for (let i = 0; i < values.length; i++) {
    callback.call(scope, values[i], i, this);
  }
};

BitSet.prototype.toJSON = function() {
  return protocol ? this._inner.call('toJson', []) : this._inner.toJson();
};

Object.defineProperty(BitSet.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

Object.defineProperty(BitSet.prototype, 'length', {
  get: function() { return protocol ? this._inner.call('length', []) : this._inner.length; }
});

Object.defineProperty(BitSet.prototype, 'array', {
  get: function() {
    return { length: protocol ? this._inner.call('wordLen', []) : this._inner.wordLen };
  }
});

module.exports = BitSet;
