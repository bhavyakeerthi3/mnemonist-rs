const { native, addIterableMethods } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function FixedStack(ArrayClass, capacity) {
  if (arguments.length < 2) {
    throw new Error('mnemonist/fixed-stack: expecting an Array class and a capacity.');
  }
  if (typeof capacity !== 'number' || capacity <= 0) {
    throw new Error('mnemonist/fixed-stack: `capacity` should be a positive number.');
  }
  this._inner = protocol
    ? protocol.create('fixed-stack', [capacity])
    : new native.FixedStackInner(capacity);
  this.capacity = capacity;
  this.ArrayClass = ArrayClass;
}

FixedStack.prototype.clear = function() { return protocol ? this._inner.call('clear', []) : this._inner.clear(); };
FixedStack.prototype.push = function(item) { return protocol ? this._inner.call('push', [item]) : this._inner.push(item); };
FixedStack.prototype.pop = function() { return protocol ? this._inner.call('pop', []) : this._inner.size === 0 ? undefined : this._inner.pop(); };
FixedStack.prototype.peek = function() { return protocol ? this._inner.call('peek', []) : this._inner.size === 0 ? undefined : this._inner.peek(); };
FixedStack.prototype.toArray = function() {
  const arr = protocol ? this._inner.call('toArray', []) : this._inner.toArray();
  if (this.ArrayClass && this.ArrayClass !== Array) {
    const typed = new this.ArrayClass(arr.length);
    for (let i = 0; i < arr.length; i++) typed[i] = arr[i];
    return typed;
  }
  return arr;
};

Object.defineProperty(FixedStack.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

addIterableMethods(FixedStack.prototype, function() {
  const arr = this.toArray();
  return Array.from(arr);
});

FixedStack.from = function(iterable, ArrayClass, capacity) {
  if (arguments.length < 3) {
    capacity = iterable.length;
    if (typeof capacity !== 'number') {
      throw new Error('mnemonist/fixed-stack.from: could not guess iterable length. Please provide desired capacity as last argument.');
    }
  }
  const stack = new FixedStack(ArrayClass, capacity);
  if (typeof iterable.length === 'number') {
    for (let i = 0; i < iterable.length; i++) stack.push(iterable[i]);
    return stack;
  }
  for (const value of iterable) stack.push(value);
  return stack;
};

module.exports = FixedStack;
