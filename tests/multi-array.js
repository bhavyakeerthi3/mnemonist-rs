const { native } = require('../adapter/_helpers.js');
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

function MultiArray(typedArrayClass, capacity) {
  let cap = 0;
  this._typedArrayClass = null;

  if (typeof typedArrayClass === 'number') {
    cap = typedArrayClass;
  } else if (typeof typedArrayClass === 'function') {
    this._typedArrayClass = typedArrayClass;
    cap = capacity || 0;
  }

  this.capacity = cap > 0 ? cap : Infinity;
  this._inner = protocol ? protocol.create('multi-array', [0]) : new native.MultiArrayInner(0);
}

MultiArray.prototype.set = function(index, value) {
  if (this.size >= this.capacity) {
    throw new Error('mnemonist/MultiArray.set: capacity exceeded.');
  }
  return protocol ? this._inner.call('set', [index, value]) : this._inner.set(index, value);
};

MultiArray.prototype.push = function(value) {
  if (this.size >= this.capacity) {
    throw new Error('mnemonist/MultiArray.push: capacity exceeded.');
  }
  return protocol ? this._inner.call('push', [value]) : this._inner.push(value);
};

MultiArray.prototype.get = function(index) {
  const res = protocol ? this._inner.call('get', [index]) : this._inner.get(index);
  if (!res) return undefined;
  if (this._typedArrayClass) {
    return new this._typedArrayClass(res);
  }
  return res;
};

MultiArray.prototype.has = function(index) {
  return protocol ? this._inner.call('has', [index]) : this._inner.has(index);
};

MultiArray.prototype.count = function(index) {
  return protocol ? this._inner.call('count', [index]) : this._inner.count(index);
};

MultiArray.prototype.clear = function() {
  return protocol ? this._inner.call('clear', []) : this._inner.clear();
};

MultiArray.prototype.values = function(index) {
  if (index !== undefined) {
    return iteratorFrom(protocol ? this._inner.call('valuesAt', [index]) : this._inner.valuesAt(index));
  }
  return iteratorFrom(protocol ? this._inner.call('values', []) : this._inner.values());
};

MultiArray.prototype.keys = function() {
  return iteratorFrom(protocol ? this._inner.call('keys', []) : this._inner.keys());
};

MultiArray.prototype.containers = function() {
  const list = protocol ? this._inner.call('containers', []) : this._inner.containers();
  if (this._typedArrayClass) {
    return iteratorFrom(list.map(c => new this._typedArrayClass(c)));
  }
  return iteratorFrom(list);
};

MultiArray.prototype.associations = function() {
  const list = protocol ? this._inner.call('associations', []) : this._inner.associations();
  if (this._typedArrayClass) {
    return iteratorFrom(list.map(item => [item[0], new this._typedArrayClass(item[1])]));
  }
  return iteratorFrom(list);
};

MultiArray.prototype.entries = function() {
  return iteratorFrom(protocol ? this._inner.call('entries', []) : this._inner.entries());
};

Object.defineProperty(MultiArray.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

Object.defineProperty(MultiArray.prototype, 'dimension', {
  get: function() { return protocol ? this._inner.call('dimension', []) : this._inner.dimension; }
});

module.exports = MultiArray;
