'use strict';

const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol' ? require('../adapter/_protocol.js') : null;

const LN2_SQUARED = Math.LN2 * Math.LN2;

function mul32(a, b) {
  return (a & 0xffff) * b + ((((a >>> 16) * b) & 0xffff) << 16) & 0xffffffff;
}

function sum32(a, b) {
  return (a & 0xffff) + (b >>> 16) + ((((a >>> 16) + b) & 0xffff) << 16) & 0xffffffff;
}

function murmurhash3(seed, data) {
  let hash = seed;
  let i;
  for (i = 0; i <= data.length - 4; i += 4) {
    let k1 = data[i] | (data[i + 1] << 8) | (data[i + 2] << 16) | (data[i + 3] << 24);
    k1 = mul32(k1, 0xcc9e2d51);
    k1 = (k1 << 15) | (k1 >>> 17);
    k1 = mul32(k1, 0x1b873593);
    hash ^= k1;
    hash = (hash << 13) | (hash >>> 19);
    hash = mul32(hash, 5);
    hash = sum32(hash, 0x6b64e654);
  }

  let k1 = 0;
  if ((data.length & 3) === 3) k1 ^= data[i + 2] << 16;
  if ((data.length & 3) >= 2) k1 ^= data[i + 1] << 8;
  if ((data.length & 3) >= 1) {
    k1 ^= data[i];
    k1 = mul32(k1, 0xcc9e2d51);
    k1 = (k1 << 15) | (k1 >>> 17);
    k1 = mul32(k1, 0x1b873593);
    hash ^= k1;
  }
  hash ^= data.length;
  hash ^= hash >>> 16;
  hash = mul32(hash, 0x85ebca6b);
  hash ^= hash >>> 13;
  hash = mul32(hash, 0xc2b2ae35);
  hash ^= hash >>> 16;
  return hash >>> 0;
}

function bytes(string) {
  const array = new Uint16Array(string.length);
  for (let i = 0; i < string.length; i++) array[i] = string.charCodeAt(i);
  return array;
}

function BloomFilter(capacityOrOptions) {
  if (!capacityOrOptions) {
    throw new Error('mnemonist/BloomFilter.constructor: a BloomFilter must be created with a capacity.');
  }
  const options = typeof capacityOrOptions === 'object' ? capacityOrOptions : { capacity: capacityOrOptions };
  if (typeof options.capacity !== 'number' || options.capacity <= 0) {
    throw new Error('mnemonist/BloomFilter.constructor: `capacity` option should be a positive integer.');
  }
  this.capacity = options.capacity;
  this.errorRate = options.errorRate || 0.005;
  if (typeof this.errorRate !== 'number' || options.errorRate <= 0) {
    throw new Error('mnemonist/BloomFilter.constructor: `errorRate` option should be a positive float.');
  }
  this._inner = protocol ? protocol.create('bloom-filter', [this.capacity, this.errorRate]) : null;
  this.clear();
}

BloomFilter.prototype.clear = function() {
  if (protocol) {
    this._inner.call('clear', []);
    this.hashFunctions = this._inner.call('hashFunctions', []);
    this.data = new Uint8Array(this._inner.call('data', []));
    return;
  }
  const bits = -this.capacity * Math.log(this.errorRate) / LN2_SQUARED;
  const length = bits / 8 | 0;
  this.hashFunctions = length * 8 / this.capacity * Math.LN2 | 0;
  this.data = new Uint8Array(length);
};

BloomFilter.prototype.add = function(string) {
  if (protocol) {
    this._inner.call('add', [string]);
    this.data = new Uint8Array(this._inner.call('data', []));
    return this;
  }
  const array = bytes(string);
  for (let seed = 0; seed < this.hashFunctions; seed++) {
    const index = murmurhash3((seed * 0xfba4c795) & 0xffffffff, array) % (this.data.length * 8);
    this.data[index >> 3] |= 1 << (7 & index);
  }
  return this;
};

BloomFilter.prototype.test = function(string) {
  if (protocol) return this._inner.call('test', [string]);
  const array = bytes(string);
  for (let seed = 0; seed < this.hashFunctions; seed++) {
    const index = murmurhash3((seed * 0xfba4c795) & 0xffffffff, array) % (this.data.length * 8);
    if (!(this.data[index >> 3] & (1 << (7 & index)))) return false;
  }
  return true;
};

BloomFilter.prototype.toJSON = function() {
  return this.data;
};

BloomFilter.from = function(iterable, options) {
  if (!options) {
    options = iterable.length || iterable.size;
    if (typeof options !== 'number') {
      throw new Error("BloomFilter.from: could not infer the filter's capacity. Try passing it as second argument.");
    }
  }
  const filter = new BloomFilter(options);
  for (const value of iterable) filter.add(value);
  return filter;
};

module.exports = BloomFilter;
