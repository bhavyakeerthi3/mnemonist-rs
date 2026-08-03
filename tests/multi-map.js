const { native } = require('../adapter/_helpers.js');
const forEach = require('obliterator/foreach');
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

function MultiMap(containerClass) {
  this._containerClass = containerClass || Array;
  const useSet = this._containerClass === Set;
  this._inner = protocol ? protocol.create('multi-map', [useSet]) : new native.MultiMapInner(useSet);
}

function wrapContainer(Container, values) {
  if (Container === Array) return values;
  if (Container === Set) return new Set(values);

  const container = new Container();
  for (let i = 0; i < values.length; i++) {
    container.push(values[i]);
  }
  return container;
}

MultiMap.prototype.clear = function() {
  return protocol ? this._inner.call('clear', []) : this._inner.clear();
};

MultiMap.prototype.set = function(key, value) {
  return protocol ? this._inner.call('set', [key, value]) : this._inner.set(key, value);
};

MultiMap.prototype.get = function(key) {
  const values = protocol ? this._inner.call('get', [key]) : this._inner.get(key);
  if (!values) return undefined;
  return wrapContainer(this._containerClass, values);
};

MultiMap.prototype.has = function(key) {
  return protocol ? this._inner.call('has', [key]) : this._inner.has(key);
};

MultiMap.prototype.contains = function(key, value) {
  return protocol ? this._inner.call('contains', [key, value]) : this._inner.contains(key, value);
};

MultiMap.prototype.delete = function(key) {
  return protocol ? this._inner.call('delete', [key]) : this._inner.delete(key);
};

MultiMap.prototype.remove = function(key, value) {
  return protocol ? this._inner.call('remove', [key, value]) : this._inner.remove(key, value);
};

MultiMap.prototype.multiplicity = function(key) {
  return protocol ? this._inner.call('multiplicity', [key]) : this._inner.multiplicity(key);
};

MultiMap.prototype.keys = function() {
  return iteratorFrom(protocol ? this._inner.call('keys', []) : this._inner.keys());
};

MultiMap.prototype.values = function() {
  return iteratorFrom(protocol ? this._inner.call('values', []) : this._inner.values());
};

MultiMap.prototype.entries = function() {
  return iteratorFrom(protocol ? this._inner.call('entries', []) : this._inner.entries());
};

MultiMap.prototype.containers = function() {
  const list = protocol ? this._inner.call('containers', []) : this._inner.containers();
  const wrapped = list.map(item => {
    // item is [key, values]
    const values = item[1];
    return wrapContainer(this._containerClass, values);
  });
  return iteratorFrom(wrapped);
};

MultiMap.prototype.associations = function() {
  const list = protocol ? this._inner.call('associations', []) : this._inner.associations();
  const wrapped = list.map(item => {
    // item is [key, values]
    const key = item[0];
    const values = item[1];
    return [key, wrapContainer(this._containerClass, values)];
  });
  return iteratorFrom(wrapped);
};

MultiMap.prototype.forEach = function(callback, scope) {
  scope = arguments.length > 1 ? scope : this;
  const list = protocol ? this._inner.call('forEach', []) : this._inner.forEach();
  for (let i = 0; i < list.length; i++) {
    // callback is called with (value, key)
    callback.call(scope, list[i][0], list[i][1], this);
  }
};

MultiMap.prototype.forEachAssociation = function(callback, scope) {
  scope = arguments.length > 1 ? scope : this;
  const list = protocol ? this._inner.call('forEachAssociation', []) : this._inner.forEachAssociation();
  for (let i = 0; i < list.length; i++) {
    // callback is called with (container, key)
    const key = list[i][0];
    const values = list[i][1];
    const container = wrapContainer(this._containerClass, values);
    callback.call(scope, container, key, this);
  }
};

MultiMap.prototype[Symbol.iterator] = function() {
  return this.entries();
};

Object.defineProperty(MultiMap.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

Object.defineProperty(MultiMap.prototype, 'dimension', {
  get: function() { return protocol ? this._inner.call('dimension', []) : this._inner.dimension; }
});

MultiMap.from = function(iterable, Container) {
  const map = new MultiMap(Container);
  forEach(iterable, (value, key) => {
    map.set(key, value);
  });
  return map;
};

module.exports = MultiMap;
