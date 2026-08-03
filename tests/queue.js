const { native, addIterableMethods } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function Queue() {
  this._inner = protocol ? protocol.create('queue') : new native.QueueInner();
}

Queue.prototype.clear = function() { return protocol ? this._inner.call('clear', []) : this._inner.clear(); };
Queue.prototype.enqueue = function(item) { return protocol ? this._inner.call('enqueue', [item]) : this._inner.enqueue(item); };
Queue.prototype.dequeue = function() { return protocol ? this._inner.call('dequeue', []) : this._inner.size === 0 ? undefined : this._inner.dequeue(); };
Queue.prototype.peek = function() { return protocol ? this._inner.call('peek', []) : this._inner.size === 0 ? undefined : this._inner.peek(); };
Queue.prototype.toArray = function() { return protocol ? this._inner.call('toArray', []) : this._inner.toArray(); };

Object.defineProperty(Queue.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

addIterableMethods(Queue.prototype, function() {
  return this.toArray();
});

Queue.from = function(iterable) {
  const queue = new Queue();
  for (const value of iterable) queue.enqueue(value);
  return queue;
};

Queue.of = function() {
  return Queue.from(arguments);
};

module.exports = Queue;
