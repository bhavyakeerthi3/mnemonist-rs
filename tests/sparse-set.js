const { native, addIterableMethods } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function SparseSet(length) {
  this._inner = protocol
    ? protocol.create('sparse-set', [length])
    : new native.SparseSetInner(length);
}

SparseSet.prototype.add = function(member) {
  if (protocol) this._inner.call('add', [member]);
  else this._inner.add(member);
  return this;
};

SparseSet.prototype.has = function(member) {
  return protocol ? this._inner.call('has', [member]) : this._inner.has(member);
};

SparseSet.prototype.delete = function(member) {
  return protocol ? this._inner.call('delete', [member]) : this._inner.delete(member);
};

SparseSet.prototype.clear = function() {
  return protocol ? this._inner.call('clear', []) : this._inner.clear();
};

Object.defineProperty(SparseSet.prototype, 'size', {
  get: function() { return protocol ? this._inner.size() : this._inner.size; }
});

Object.defineProperty(SparseSet.prototype, 'length', {
  get: function() { return protocol ? this._inner.call('length', []) : this._inner.length; }
});

addIterableMethods(SparseSet.prototype, function() {
  return protocol ? this._inner.call('values', []) : this._inner.values();
});

module.exports = SparseSet;
