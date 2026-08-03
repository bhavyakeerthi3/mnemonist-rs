const { native } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol' ? require('../adapter/_protocol.js') : null;

function StaticDisjointSet(size) {
  this._inner = protocol ? protocol.create('static-disjoint-set', [size]) : new native.StaticDisjointSetInner(size);
}

StaticDisjointSet.prototype.union = function(x, y) {
  return protocol ? this._inner.call('union', [x, y]) : this._inner.union(x, y);
};

StaticDisjointSet.prototype.connected = function(x, y) {
  return protocol ? this._inner.call('connected', [x, y]) : this._inner.connected(x, y);
};

StaticDisjointSet.prototype.mapping = function() {
  return protocol ? this._inner.call('mapping', []) : this._inner.mapping();
};

StaticDisjointSet.prototype.compile = function() {
  return protocol ? this._inner.call('compile', []) : this._inner.compile();
};

Object.defineProperty(StaticDisjointSet.prototype, 'size', {
  get: function() { return protocol ? this._inner.call('size', []) : this._inner.size; }
});

Object.defineProperty(StaticDisjointSet.prototype, 'dimension', {
  get: function() { return protocol ? this._inner.call('dimension', []) : this._inner.dimension; }
});

module.exports = StaticDisjointSet;
