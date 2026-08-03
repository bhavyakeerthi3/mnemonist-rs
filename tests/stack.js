const { native, addIterableMethods } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function Stack() {
  this._inner = protocol ? protocol.create('stack') : new native.StackInner();
}

Stack.prototype.clear = function() {
  return protocol ? this._inner.call('clear', []) : this._inner.clear();
};

Stack.prototype.push = function(item) {
  return protocol ? this._inner.call('push', [item]) : this._inner.push(item);
};

Stack.prototype.pop = function() {
  if (protocol) return this._inner.call('pop', []);
  return this._inner.size === 0 ? undefined : this._inner.pop();
};

Stack.prototype.peek = function() {
  if (protocol) return this._inner.call('peek', []);
  return this._inner.size === 0 ? undefined : this._inner.peek();
};

Stack.prototype.toArray = function() {
  return protocol ? this._inner.call('toArray', []) : this._inner.toArray();
};

Object.defineProperty(Stack.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

addIterableMethods(Stack.prototype, function() {
  return this.toArray();
});

Stack.from = function(iterable) {
  const stack = new Stack();
  for (const value of iterable) stack.push(value);
  return stack;
};

Stack.of = function() {
  return Stack.from(arguments);
};

module.exports = Stack;
