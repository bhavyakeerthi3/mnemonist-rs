const { addIterableMethods } = require('./_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('./_protocol.js')
  : null;

function ringProto(Constructor, innerName, label) {
  Constructor.prototype.clear = function() { return protocol ? this._inner.call('clear', []) : this._inner.clear(); };
  Constructor.prototype.push = function(item) { return protocol ? this._inner.call('push', [item]) : this._inner.push(item); };
  Constructor.prototype.unshift = function(item) { return protocol ? this._inner.call('unshift', [item]) : this._inner.unshift(item); };
  Constructor.prototype.pop = function() { return protocol ? this._inner.call('pop', []) : this._inner.size === 0 ? undefined : this._inner.pop(); };
  Constructor.prototype.shift = function() { return protocol ? this._inner.call('shift', []) : this._inner.size === 0 ? undefined : this._inner.shift(); };
  Constructor.prototype.peekFirst = function() { return protocol ? this._inner.call('peekFirst', []) : this._inner.size === 0 ? undefined : this._inner.peekFirst(); };
  Constructor.prototype.peekLast = function() { return protocol ? this._inner.call('peekLast', []) : this._inner.size === 0 ? undefined : this._inner.peekLast(); };
  Constructor.prototype.get = function(index) {
    if (!Number.isInteger(index) || index < 0 || index >= this.size) return undefined;
    return protocol ? this._inner.call('get', [index]) : this._inner.get(index);
  };
  Constructor.prototype.toArray = function() {
    const arr = protocol ? this._inner.call('toArray', []) : this._inner.toArray();
    if (this.ArrayClass && this.ArrayClass !== Array) {
      const typed = new this.ArrayClass(arr.length);
      for (let i = 0; i < arr.length; i++) typed[i] = arr[i];
      return typed;
    }
    return arr;
  };

  Object.defineProperty(Constructor.prototype, 'size', {
    get: function() { return protocol ? this._inner.size() : this._inner.size; }
  });
  Object.defineProperty(Constructor.prototype, 'start', {
    get: function() { return protocol ? this._inner.call('start', []) : this._inner.start; }
  });

  addIterableMethods(Constructor.prototype, function() {
    return Array.from(this.toArray());
  });

  Constructor.from = function(iterable, ArrayClass, capacity) {
    if (arguments.length < 3) {
      capacity = iterable.length;
      if (typeof capacity !== 'number') {
        throw new Error(`mnemonist/${label}.from: could not guess iterable length. Please provide desired capacity as last argument.`);
      }
    }
    const deque = new Constructor(ArrayClass, capacity);
    if (typeof iterable.length === 'number') {
      for (let i = 0; i < iterable.length; i++) deque.push(iterable[i]);
      return deque;
    }
    for (const value of iterable) deque.push(value);
    return deque;
  };
}

module.exports = ringProto;
