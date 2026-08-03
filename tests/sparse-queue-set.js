const { native, addIterableMethods, nullToUndefined } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function SparseQueueSet(capacity) {
  this._inner = protocol
    ? protocol.create('sparse-queue-set', [capacity])
    : new native.SparseQueueSetInner(capacity);
}

SparseQueueSet.prototype.enqueue = function(member) {
  if (protocol) this._inner.call('enqueue', [member]);
  else this._inner.enqueue(member);
  return this.size;
};

SparseQueueSet.prototype.dequeue = function() {
  return protocol ? this._inner.call('dequeue', []) : nullToUndefined(this._inner.dequeue());
};

SparseQueueSet.prototype.has = function(member) {
  return protocol ? this._inner.call('has', [member]) : this._inner.has(member);
};

SparseQueueSet.prototype.clear = function() {
  return protocol ? this._inner.call('clear', []) : this._inner.clear();
};

Object.defineProperty(SparseQueueSet.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

Object.defineProperty(SparseQueueSet.prototype, 'capacity', {
  get: function() { return protocol ? this._inner.call('capacity', []) : this._inner.capacity; }
});

addIterableMethods(SparseQueueSet.prototype, function() {
  return protocol ? this._inner.call('values', []) : this._inner.values();
});

module.exports = SparseQueueSet;
