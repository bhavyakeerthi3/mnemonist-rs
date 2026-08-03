'use strict';

const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function pointerArray(size, values) {
  const Type = size <= 0xff ? Uint8Array : size <= 0xffff ? Uint16Array : Uint32Array;
  return new Type(values);
}

function KDTree(labels, points) {
  this.labels = labels;
  this.points = points;
  this._inner = protocol ? protocol.create('kd-tree', [labels, points]) : null;
  this.size = points.length;
  this.dimensions = points.length ? points[0].length : 0;
  const pivots = [];
  const lefts = [];
  const rights = [];

  const build = (indices, axis) => {
    if (!indices.length) return 0;
    indices.sort((a, b) => points[a][axis] - points[b][axis]);
    const index = indices.length >> 1;
    const pivot = indices[index];
    const position = pivots.length;
    pivots.push(pivot);
    lefts.push(0);
    rights.push(0);
    lefts[position] = build(indices.slice(0, index), (axis + 1) % this.dimensions);
    rights[position] = build(indices.slice(index + 1), (axis + 1) % this.dimensions);
    return position + 1;
  };

  build(Array.from({ length: this.size }, (_, i) => i), 0);
  this.pivots = pointerArray(this.size, pivots);
  this.lefts = pointerArray(this.size, lefts);
  this.rights = pointerArray(this.size, rights);
}

function distance(a, b) {
  let result = 0;
  for (let i = 0; i < a.length; i++) {
    const delta = a[i] - b[i];
    result += delta * delta;
  }
  return result;
}

KDTree.prototype.linearKNearestNeighbors = function(k, query) {
  if (this._inner) return this._inner.call('nearestLinear', [k, query]);
  return this.points
    .map((point, index) => ({ index, distance: distance(point, query) }))
    .sort((a, b) => {
      const byDistance = a.distance - b.distance;
      if (byDistance) return byDistance;
      const left = String([this.labels[a.index], this.points[a.index]]);
      const right = String([this.labels[b.index], this.points[b.index]]);
      return left < right ? -1 : left > right ? 1 : 0;
    })
    .slice(0, k)
    .map(candidate => this.labels[candidate.index]);
};

KDTree.prototype.kNearestNeighbors = function(k, query) {
  return this._inner ? this._inner.call('nearest', [k, query]) : this.linearKNearestNeighbors(k, query);
};
KDTree.prototype.nearestNeighbor = function(query) {
  return this.linearKNearestNeighbors(1, query)[0];
};

KDTree.from = function(iterable, dimensions) {
  const items = Array.from(iterable);
  if (!items.length) return new KDTree([], []);
  if (!Number.isInteger(dimensions) || dimensions <= 0) {
    throw new Error('mnemonist/KDTree.from: dimensions should be a positive integer.');
  }
  return new KDTree(items.map(item => item[0]), items.map(item => item[1]));
};

KDTree.fromAxes = function(axes, labels) {
  const size = axes.length ? axes[0].length : 0;
  const points = Array.from({ length: size }, (_, index) => axes.map(axis => axis[index]));
  return new KDTree(labels || Array.from({ length: size }, (_, index) => index), points);
};

module.exports = KDTree;
