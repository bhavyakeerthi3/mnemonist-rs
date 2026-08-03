'use strict';

const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol' ? require('../adapter/_protocol.js') : null;

function isPowerOfTwo(value) {
  return value > 0 && (value & (value - 1)) === 0;
}

function HashedArrayTree(ArrayClass, options) {
  if (arguments.length < 1) {
    throw new Error('mnemonist/hashed-array-tree: expecting at least a byte array constructor.');
  }

  let initialCapacity = typeof options === 'number' ? options : 0;
  let initialLength = 0;
  let blockSize = 1024;
  if (options && typeof options === 'object') {
    initialCapacity = options.initialCapacity || 0;
    initialLength = options.initialLength || 0;
    blockSize = options.blockSize || blockSize;
  }
  if (!isPowerOfTwo(blockSize)) {
    throw new Error('mnemonist/hashed-array-tree: block size should be a power of two.');
  }

  this.ArrayClass = ArrayClass;
  this.blockSize = blockSize;
  this._inner = protocol ? protocol.create('hashed-array-tree', [initialCapacity, initialLength, blockSize]) : null;
  this.length = initialLength;
  this.capacity = Math.ceil(Math.max(initialCapacity, initialLength) / blockSize) * blockSize;
  this._blocks = [];
  while (this._blocks.length * blockSize < this.capacity) this._blocks.push(new ArrayClass(blockSize));
}

HashedArrayTree.prototype._slot = function(index) {
  return [Math.floor(index / this.blockSize), index % this.blockSize];
};

HashedArrayTree.prototype.set = function(index, value) {
  if (protocol) {
    this._inner.call('set', [index, value]);
    return this;
  }
  if (index >= this.length) throw new Error('HashedArrayTree.set: index out of bounds.');
  const slot = this._slot(index);
  this._blocks[slot[0]][slot[1]] = value;
  return this;
};

HashedArrayTree.prototype.get = function(index) {
  if (protocol) return this._inner.call('get', [index]);
  if (index >= this.length) return undefined;
  const slot = this._slot(index);
  return this._blocks[slot[0]][slot[1]];
};

HashedArrayTree.prototype.grow = function(target) {
  if (protocol) {
    this._inner.call('grow', target === undefined ? [] : [target]);
    this.capacity = this._inner.call('capacity', []);
    return this;
  }
  const required = typeof target === 'number' ? target : this.capacity + this.blockSize;
  while (this.capacity < required) {
    this._blocks.push(new this.ArrayClass(this.blockSize));
    this.capacity += this.blockSize;
  }
  return this;
};

HashedArrayTree.prototype.resize = function(length) {
  if (protocol) {
    this._inner.call('resize', [length]);
    this.length = this._inner.call('length', []);
    this.capacity = this._inner.call('capacity', []);
    return this;
  }
  if (length > this.length) this.grow(length);
  this.length = length;
  return this;
};

HashedArrayTree.prototype.push = function(value) {
  if (protocol) {
    const length = this._inner.call('push', [value]);
    this.length = length;
    this.capacity = this._inner.call('capacity', []);
    return length;
  }
  if (this.length === this.capacity) this.grow();
  const slot = this._slot(this.length++);
  this._blocks[slot[0]][slot[1]] = value;
  return this.length;
};

HashedArrayTree.prototype.pop = function() {
  if (protocol) {
    const value = this._inner.call('pop', []);
    this.length = this._inner.call('length', []);
    return value;
  }
  if (this.length === 0) return undefined;
  const slot = this._slot(--this.length);
  return this._blocks[slot[0]][slot[1]];
};

module.exports = HashedArrayTree;
