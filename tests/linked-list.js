const { native, addIterableMethods } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function LinkedList() {
  this._inner = protocol ? protocol.create('linked-list') : new native.LinkedListInner();
}

LinkedList.prototype.clear = function() { return protocol ? this._inner.call('clear', []) : this._inner.clear(); };
LinkedList.prototype.push = function(item) { return protocol ? this._inner.call('push', [item]) : this._inner.push(item); };
LinkedList.prototype.unshift = function(item) { return protocol ? this._inner.call('unshift', [item]) : this._inner.unshift(item); };
LinkedList.prototype.shift = function() { return protocol ? this._inner.call('shift', []) : this._inner.size === 0 ? undefined : this._inner.shift(); };
LinkedList.prototype.first = function() { return protocol ? this._inner.call('first', []) : this._inner.size === 0 ? undefined : this._inner.first(); };
LinkedList.prototype.peek = LinkedList.prototype.first;
LinkedList.prototype.last = function() { return protocol ? this._inner.call('last', []) : this._inner.size === 0 ? undefined : this._inner.last(); };
LinkedList.prototype.toArray = function() { return protocol ? this._inner.call('toArray', []) : this._inner.toArray(); };
LinkedList.prototype.toJSON = function() { return this.toArray(); };

Object.defineProperty(LinkedList.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

addIterableMethods(LinkedList.prototype, function() {
  return this.toArray();
});

LinkedList.from = function(iterable) {
  const list = new LinkedList();
  const values = (iterable && typeof iterable[Symbol.iterator] === 'function')
    ? iterable
    : Object.values(iterable);
  for (const value of values) list.push(value);
  return list;
};

module.exports = LinkedList;
