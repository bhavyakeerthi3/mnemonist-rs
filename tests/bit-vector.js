const { native, nullToUndefined } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol' ? require('../adapter/_protocol.js') : null;

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

const DEFAULT_POLICY = function(capacity) {
  return Math.max(1, Math.ceil(capacity * 1.5));
};

function BitVector(options) {
  let initialLength = 0;
  let policy = DEFAULT_POLICY;

  if (typeof options === 'number') {
    initialLength = options;
  } else if (options && typeof options === 'object') {
    // Upstream quirk: initialCapacity silently becomes initialLength when
    // initialLength isn't also given.
    initialLength = options.initialLength || options.initialCapacity || 0;
    if (options.policy !== undefined) {
      policy = options.policy;
    }
  }

  this._inner = protocol ? protocol.create('bit-vector', [initialLength]) : new native.BitVectorInner(initialLength);
  this.capacity = Math.ceil(initialLength / 32) * 32;
  this._policy = policy;

  if (this.capacity > 0) {
    protocol ? this._inner.call('reallocate', [this.capacity]) : this._inner.reallocate(this.capacity);
  }
}

// Mirrors upstream's `applyPolicy`: the "did the policy make progress"
// check always compares against the instance's *current* capacity, even
// inside a multi-iteration `grow(n)` loop — not the loop's running value.
BitVector.prototype._applyPolicy = function(override) {
  const newCapacity = this._policy(override || this.capacity);

  if (typeof newCapacity !== 'number' || newCapacity < 0) {
    throw new Error('mnemonist/BitVector: policy returned an invalid value (expecting a positive integer).');
  }
  if (newCapacity <= this.capacity) {
    throw new Error('mnemonist/BitVector: policy returned a less or equal capacity to allocate.');
  }
  return Math.ceil(newCapacity / 32) * 32;
};

BitVector.prototype.set = function(index, value) {
  if (index >= this.length) {
    throw new Error('BitVector.set: index out of bounds.');
  }
  if (value === undefined) {
    value = true;
  } else {
    value = !!value;
  }
  protocol ? this._inner.call('set', [index, value]) : this._inner.set(index, value);
  return this;
};

BitVector.prototype.get = function(index) {
  if (index >= this.length) return undefined;
  return protocol ? this._inner.call('get', [index]) : this._inner.get(index);
};

BitVector.prototype.test = function(index) {
  if (index >= this.length) return false;
  return protocol ? this._inner.call('test', [index]) : this._inner.test(index);
};

BitVector.prototype.reset = function(index) {
  if (index >= this.length) return this;
  protocol ? this._inner.call('reset', [index]) : this._inner.reset(index);
  return this;
};

BitVector.prototype.flip = function(index) {
  if (index >= this.length) return this;
  protocol ? this._inner.call('flip', [index]) : this._inner.flip(index);
  return this;
};

BitVector.prototype.push = function(value) {
  value = !!value;
  if (this.capacity === this.length) {
    this.grow();
  }
  return protocol ? this._inner.call('push', [value]) : this._inner.push(value);
};

BitVector.prototype.pop = function() {
  return nullToUndefined(protocol ? this._inner.call('pop', []) : this._inner.pop());
};

BitVector.prototype.reallocate = function(capacity) {
  const virtualCapacity = capacity;
  capacity = Math.ceil(capacity / 32) * 32;

  if (virtualCapacity < this.length) {
    protocol ? this._inner.call('resize', [virtualCapacity]) : this._inner.resize(virtualCapacity);
  }

  if (capacity === this.capacity) {
    return this;
  }

  this.capacity = capacity;
  protocol ? this._inner.call('reallocate', [capacity]) : this._inner.reallocate(capacity);
  return this;
};

BitVector.prototype.grow = function(desiredCapacity) {
  let newCapacity;

  if (typeof desiredCapacity === 'number') {
    if (this.capacity >= desiredCapacity) {
      return this;
    }
    newCapacity = this.capacity;
    while (newCapacity < desiredCapacity) {
      newCapacity = this._applyPolicy(newCapacity);
    }
    this.reallocate(newCapacity);
    return this;
  }

  newCapacity = this._applyPolicy();
  this.reallocate(newCapacity);
  return this;
};

BitVector.prototype.resize = function(newLength) {
  protocol ? this._inner.call('resize', [newLength]) : this._inner.resize(newLength);
  if (newLength > this.capacity) {
    this.reallocate(newLength);
  }
  return this;
};

BitVector.prototype.rank = function(index) {
  return protocol ? this._inner.call('rank', [index]) : this._inner.rank(index);
};

BitVector.prototype.select = function(r) {
  if (this.size === 0 || r >= this.length) return -1;
  // Upstream's loop checks its running one-count after each bit. Therefore
  // `select(0)` returns index 0 only when the first bit is unset.
  if (r === 0) return this._inner.get(0) === 0 ? 0 : undefined;
  if (r > this.size) return undefined;
  const result = protocol ? this._inner.call('select', [r]) : this._inner.select(r);
  return result === null ? -1 : result;
};

BitVector.prototype.values = function() {
  return iteratorFrom(protocol ? this._inner.call('values', []) : this._inner.values());
};

BitVector.prototype.entries = function() {
  return iteratorFrom(protocol ? this._inner.call('entries', []) : this._inner.entries());
};

BitVector.prototype.forEach = function(callback, scope) {
  scope = arguments.length > 1 ? scope : this;
  const values = protocol ? this._inner.call('values', []) : this._inner.values();
  for (let i = 0; i < values.length; i++) {
    callback.call(scope, values[i], i, this);
  }
};

BitVector.prototype.toJSON = function() {
  return protocol ? this._inner.call('toJson', []) : this._inner.toJson();
};

Object.defineProperty(BitVector.prototype, 'size', {
  get: function() { return protocol ? this._inner.call('size', []) : this._inner.size; }
});

Object.defineProperty(BitVector.prototype, 'length', {
  get: function() { return protocol ? this._inner.call('length', []) : this._inner.length; }
});

Object.defineProperty(BitVector.prototype, 'array', {
  get: function() {
    return { length: this.capacity / 32 };
  }
});

module.exports = BitVector;
