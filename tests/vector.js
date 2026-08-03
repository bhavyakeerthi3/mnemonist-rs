'use strict';

const obliterator = require('obliterator');
const { native } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

const NATIVE_ARRAY_CLASSES = new Set([
  Int8Array,
  Uint8Array,
  Uint8ClampedArray,
  Int16Array,
  Uint16Array,
  Int32Array,
  Uint32Array,
  Float32Array,
  Float64Array
]);

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

function usesNativeStorage(ArrayClass) {
  return NATIVE_ARRAY_CLASSES.has(ArrayClass);
}

function coerce(ArrayClass, value) {
  const cell = new ArrayClass(1);
  cell[0] = value;
  return cell[0];
}

function copyNativeValues(vector, capacity) {
  const array = new vector.ArrayClass(capacity);
  const values = protocol ? vector._inner.call('values', []) : vector._inner.values();
  for (let i = 0; i < values.length; i++) array[i] = values[i];
  return array;
}

function Vector(ArrayClass, options) {
  if (arguments.length < 1) {
    throw new Error('vector: needs at least an ArrayClass.');
  }

  this.ArrayClass = ArrayClass;
  let initialLength = 0;
  let initialCapacity = 0;
  this.policy = null;

  if (typeof options === 'number') {
    initialCapacity = options;
  }
  else if (options && typeof options === 'object') {
    if (options.initialLength !== undefined) initialLength = options.initialLength;
    if (options.initialCapacity !== undefined) initialCapacity = options.initialCapacity;
    if (options.policy !== undefined) this.policy = options.policy;
  }

  if (initialCapacity === 0 && initialLength > 0) initialCapacity = initialLength;

  this.policy = this.policy || function(capacity) {
    return Math.max(1, Math.ceil(capacity * 1.5));
  };

  this.capacity = initialCapacity || 8;
  this.length = initialLength;
  this._inner = protocol ? protocol.create('vector', [this.capacity, initialLength]) :
    native && usesNativeStorage(ArrayClass) ? new native.VectorInner(this.capacity, initialLength) : null;
  this.array = this._inner ? copyNativeValues(this, this.capacity) : new ArrayClass(this.capacity);
}

Vector.prototype._coerce = function(value) {
  return coerce(this.ArrayClass, value);
};

Vector.prototype.set = function(index, value) {
  if (index >= this.length) {
    throw new Error('Vector.set: index out of bounds.');
  }

  if (this._inner) {
    const stored = this._coerce(value);
    if (protocol) this._inner.call('set', [index, stored]);
    else this._inner.set(index, stored);
    this.array[index] = stored;
  }
  else {
    this.array[index] = value;
  }
  return this;
};

Vector.prototype.get = function(index) {
  if (index >= this.length) return undefined;
  return this._inner ? (protocol ? this._inner.call('get', [index]) : this._inner.get(index)) : this.array[index];
};

Vector.prototype.applyPolicy = function(override) {
  const newCapacity = this.policy(override || this.capacity);

  if (typeof newCapacity !== 'number' || newCapacity < 0) {
    throw new Error('mnemonist/Vector: policy returned an invalid value (expecting a positive integer).');
  }
  if (newCapacity <= this.capacity) {
    throw new Error('mnemonist/Vector: policy returned a less or equal capacity to allocate.');
  }
  return newCapacity;
};

Vector.prototype.push = function(value) {
  if (this.capacity === this.length) this.grow();

  if (this._inner) {
    const stored = this._coerce(value);
    this.length = protocol ? this._inner.call('push', [stored]) : this._inner.push(stored);
    this.array[this.length - 1] = stored;
    return this.length;
  }

  this.array[this.length++] = value;
  return this.length;
};

Vector.prototype.pop = function() {
  if (this.length === 0) return undefined;
  if (this._inner) {
    const value = protocol ? this._inner.call('pop', []) : this._inner.pop();
    this.length = protocol ? this._inner.call('length', []) : this._inner.length;
    return value;
  }
  return this.array[--this.length];
};

Vector.prototype.clear = function() {
  this.length = 0;
  if (this._inner) {
    if (protocol) this._inner.call('clear', []);
    else this._inner.clear();
  }
};

Vector.prototype.reallocate = function(newCapacity) {
  const previousArray = this.array;
  if (newCapacity < this.length) this.length = newCapacity;

  if (this._inner) {
    if (protocol) {
      this._inner.call('reallocate', [newCapacity]);
      this.length = this._inner.call('length', []);
    }
    else {
      this._inner.reallocate(newCapacity);
      this.length = this._inner.length;
    }
  }

  const nextArray = new this.ArrayClass(newCapacity);
  for (let i = 0; i < Math.min(previousArray.length, newCapacity); i++) nextArray[i] = previousArray[i];
  this.array = nextArray;
  this.capacity = newCapacity;
  return this;
};

Vector.prototype.grow = function(desiredCapacity) {
  let newCapacity;

  if (typeof desiredCapacity === 'number') {
    if (this.capacity >= desiredCapacity) return this;
    newCapacity = this.capacity;
    while (newCapacity < desiredCapacity) newCapacity = this.applyPolicy(newCapacity);
    return this.reallocate(newCapacity);
  }

  newCapacity = this.applyPolicy();
  return this.reallocate(newCapacity);
};

Vector.prototype.resize = function(newLength) {
  if (newLength === this.length) return this;

  if (newLength < this.length) {
    this.length = newLength;
    if (this._inner) {
      if (protocol) this._inner.call('resize', [newLength, 0]);
      else this._inner.resize(newLength);
    }
    return this;
  }

  if (this._inner) {
    const previousArray = this.array;
    const previousLength = this.length;
    const nextArray = new this.ArrayClass(newLength);
    for (let i = 0; i < Math.min(previousArray.length, newLength); i++) {
      nextArray[i] = previousArray[i];
    }
    if (protocol) {
      this._inner.call('reallocate', [newLength]);
      this._inner.call('resize', [newLength, 0]);
      for (let i = previousLength; i < newLength; i++) this._inner.call('set', [i, nextArray[i]]);
    }
    else {
      this._inner.reallocate(newLength);
      this._inner.resize(newLength);
      for (let i = previousLength; i < newLength; i++) this._inner.set(i, nextArray[i]);
    }
    this.array = nextArray;
    this.capacity = newLength;
    this.length = protocol ? this._inner.call('length', []) : this._inner.length;
    return this;
  }

  const nextArray = new this.ArrayClass(newLength);
  for (let i = 0; i < Math.min(this.array.length, newLength); i++) nextArray[i] = this.array[i];
  this.array = nextArray;
  this.capacity = newLength;
  this.length = newLength;
  return this;
};

Vector.prototype.values = function() {
  const values = this._inner ? (protocol ? this._inner.call('values', []) : this._inner.values()) : Array.from(this.array.slice(0, this.length));
  return iteratorFrom(values);
};

Vector.prototype.entries = function() {
  const values = this._inner ? (protocol ? this._inner.call('values', []) : this._inner.values()) : Array.from(this.array.slice(0, this.length));
  return iteratorFrom(values.map((value, index) => [index, value]));
};

Vector.prototype.forEach = function(callback, scope) {
  scope = arguments.length > 1 ? scope : this;
  const values = this._inner ? (protocol ? this._inner.call('values', []) : this._inner.values()) : Array.from(this.array.slice(0, this.length));
  for (let i = 0; i < values.length; i++) callback.call(scope, values[i], i, this);
};

Vector.prototype[Symbol.iterator] = function() {
  return this.values();
};

Vector.from = function(iterable, ArrayClass, capacity) {
  const items = [];
  obliterator.forEach(iterable, item => {
    items.push(item);
  });
  const vector = new Vector(ArrayClass, capacity === undefined ? items.length : capacity);
  for (let i = 0; i < items.length; i++) vector.push(items[i]);
  return vector;
};

function makeSubclass(ArrayClass) {
  function Subclass(options) {
    Vector.call(this, ArrayClass, options);
  }
  Subclass.prototype = Object.create(Vector.prototype);
  Subclass.prototype.constructor = Subclass;
  Subclass.from = function(iterable, capacity) {
    return Vector.from(iterable, ArrayClass, capacity);
  };
  return Subclass;
}

Vector.Uint8Vector = makeSubclass(Uint8Array);
Vector.Uint8ClampedVector = makeSubclass(Uint8ClampedArray);
Vector.Uint16Vector = makeSubclass(Uint16Array);
Vector.Uint32Vector = makeSubclass(Uint32Array);
Vector.Int8Vector = makeSubclass(Int8Array);
Vector.Int16Vector = makeSubclass(Int16Array);
Vector.Int32Vector = makeSubclass(Int32Array);
Vector.Float32Vector = makeSubclass(Float32Array);
Vector.Float64Vector = makeSubclass(Float64Array);

function PointerVector(options) {
  Vector.call(this, Uint8Array, options);
}
PointerVector.prototype = Object.create(Vector.prototype);
PointerVector.prototype.constructor = PointerVector;

PointerVector.prototype.push = function(value) {
  let nextClass = this.ArrayClass;
  if (value > 65535) nextClass = Uint32Array;
  else if (value > 255) nextClass = Uint16Array;

  if (nextClass !== this.ArrayClass) {
    this.ArrayClass = nextClass;
    this.array = this._inner ? copyNativeValues(this, this.capacity) : new nextClass(this.array);
  }

  return Vector.prototype.push.call(this, value);
};

Vector.PointerVector = PointerVector;

module.exports = Vector;
